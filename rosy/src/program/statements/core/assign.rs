//! # Assignment Statement
//!
//! Assigns the result of an expression to a variable (optionally indexed).
//!
//! ## Syntax
//!
//! ```text
//! name := expr;
//! name(i) := expr;          { indexed assignment }
//! name(i)(j) := expr;       { multi-dim indexed assignment }
//! ```
//!
//! ## Type Checking
//!
//! The right-hand side expression must be type-compatible with the target
//! variable. Indexed assignments check that the resulting element type matches.
//!
//! ## Example
//!
//! ```text
//! VARIABLE (RE) x;
//! x := 42;
//! ```

use std::collections::{BTreeSet, HashSet};

use crate::{ast::*, program::{expressions::{Expr, core::variable_identifier::VariableIdentifier}, statements::SourceLocation}, resolve::{ExprRecipe, ResolutionRule, ScopeContext, TypeResolver}, transpile::{TranspileableExpr, TranspileableStatement, VariableScope}};
use crate::rosy_lib::{RosyType, RosyBaseType};
use super::super::super::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Context, Error, anyhow, ensure};

/// AST node for the assignment statement `name := expr;`.
#[derive(Debug)]
pub struct AssignStatement {
    pub identifier: VariableIdentifier,
    pub value: Expr,
}

impl FromRule for AssignStatement {
    fn from_rule(pair: pest::iterators::Pair<crate::ast::Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == crate::ast::Rule::assignment, 
            "Expected `assignment` rule when building assignment statement, found: {:?}", pair.as_rule());
        let mut inner = pair.into_inner();

        let lhs = inner.next()
            .context("Missing first token `variable_name`!")?;
        let identifier = VariableIdentifier::from_rule(lhs)
            .context("...while building variable identifier for assignment statement")?
            .ok_or_else(|| anyhow::anyhow!("Expected variable identifier for assignment statement"))?;

        let expr_pair = inner.next()
            .context("Missing second token `expr`!")?;
        let expr = Expr::from_rule(expr_pair)
            .context("Failed to build expression for assignment statement!")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for assignment statement"))?;

        Ok(Some(AssignStatement { 
            identifier,
            value: expr
        }))
    }
}
impl TranspileableStatement for AssignStatement {
    fn discover_dependencies(
        &self,
        resolver: &mut TypeResolver,
        ctx: &mut ScopeContext,
        source_location: SourceLocation
    ) -> Option<Result<()>> {
        // Discover function call sites within the RHS expression
        if let Err(e) = resolver.discover_expr_function_calls(&self.value, ctx) {
            return Some(Err(e.context("...while discovering function call dependencies in assignment statement")));
        }

        let var_name = &self.identifier.name;
        let var_slot = match ctx.variables.get(var_name) {
            Some(s) => s.clone(),
            None => return Some(Ok(())), // unknown variable, skip
        };

        // Build a recipe for the RHS expression and collect its dependencies
        let mut deps = HashSet::new();
        let recipe = resolver.build_expr_recipe(&self.value, ctx, &mut deps);

        if let Some(node) = resolver.nodes.get(&var_slot) {
            if node.resolved.is_some() {
                // Already has an explicit type — check that the new
                // assignment is compatible (if evaluable now).
                // Account for indexing on the LHS: X[I, J] := expr
                // means we're assigning to a sub-element, so reduce
                // the declared dimensions by the number of indices.
                let mut explicit_type = node.resolved.as_ref().unwrap().clone();
                let num_indices = self.identifier.num_index_dimensions();
                explicit_type.dimensions = explicit_type.dimensions
                    .saturating_sub(num_indices);
                if let Ok(new_type) = resolver.evaluate_recipe(&recipe) {
                    if new_type != explicit_type {
                        let scope_str = if ctx.scope_path.is_empty() {
                            "global scope".to_string()
                        } else {
                            format!("'{}'", ctx.scope_path.join(" > "))
                        };
                        let decl_hint = node.declared_at.as_ref()
                            .map(|loc| format!("\n│  📍 Declared at: {}", loc))
                            .unwrap_or_default();
                        let assign_hint = format!("\n│  📍 Assigned at: {}", source_location);
                        return Some(Err(anyhow!(
                            "\n╭─ Type Conflict ──────────────────────────────────────────\n\
                                │\n\
                                │  Variable '{}' (in {}) is declared as {} but is\n\
                                │  assigned a value of type {}.{}{}\n\
                                │\n\
                                │  💡 Either:\n\
                                │     • Change the explicit type to match the assignment, or\n\
                                │     • Split into separate variables: {}_{:?}  and  {}_{:?}\n\
                                │\n\
                                ╰──────────────────────────────────────────────────────────",
                            var_name, scope_str, explicit_type, new_type,
                            decl_hint, assign_hint,
                            var_name, explicit_type.base_type, var_name, new_type.base_type,
                        )));
                    }
                }
                return Some(Ok(())); // already has explicit type, no inference needed
            }
        } else {
            return Some(Ok(()));
        }

        // Check for conflicting re-assignment: if a previous assignment
        // already established an inference recipe, verify the new one
        // produces the same type (when both are evaluable).
        let has_existing_rule = matches!(
            resolver.nodes.get(&var_slot).map(|n| &n.rule),
            Some(ResolutionRule::InferredFrom { .. })
        );

        if has_existing_rule {
            let old_recipe = if let Some(node) = resolver.nodes.get(&var_slot) {
                if let ResolutionRule::InferredFrom { recipe: ref r, .. } = node.rule {
                    Some(r.clone())
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(ref old_recipe) = old_recipe {
                // Try to evaluate both recipes. If the new recipe is
                // self-referential (e.g. Y:=Y&I), evaluate_recipe will
                // fail because Y isn't resolved yet. In that case,
                // temporarily resolve Y to the old type so we can
                // evaluate what the new assignment produces.
                let old_type_result = resolver.evaluate_recipe(old_recipe);
                let mut new_type_result = resolver.evaluate_recipe(&recipe);

                // If the new recipe failed but old succeeded, try again
                // with a temporary assumption that the variable has the
                // old type (handles self-referential patterns like Y:=Y&I)
                if new_type_result.is_err() {
                    if let Ok(ref old_type) = old_type_result {
                        // Temporarily mark this slot as resolved
                        if let Some(node) = resolver.nodes.get_mut(&var_slot) {
                            node.resolved = Some(old_type.clone());
                        }
                        new_type_result = resolver.evaluate_recipe(&recipe);
                        // Undo the temporary resolution
                        if let Some(node) = resolver.nodes.get_mut(&var_slot) {
                            node.resolved = None;
                        }
                    }
                }

                if let (Ok(old_type), Ok(new_type)) = (
                    old_type_result,
                    new_type_result,
                ) {
                    if old_type != new_type {
                        // RE ↔ VE coercion: if one is RE and the other is VE,
                        // upgrade to VE. This supports COSY's dynamic typing
                        // pattern where a variable is initialized as a scalar
                        // (Y:=1) and then grown into a vector (Y:=Y&I).
                        let re = RosyType::RE();
                        let ve = RosyType::VE();
                        if (old_type == re && new_type == ve) || (old_type == ve && new_type == re) {
                            tracing::debug!(
                                "RE↔VE coercion for '{}': upgrading to VE (RE assignments will wrap in vec![])",
                                var_name
                            );
                            // Use a literal VE recipe — we already know the
                            // result type. Using the self-referential recipe
                            // (e.g. Concat(Y, I)) would create a circular
                            // dependency that can't be resolved.
                            if let Some(node) = resolver.nodes.get_mut(&var_slot) {
                                node.rule = ResolutionRule::InferredFrom {
                                    recipe: ExprRecipe::Literal(RosyType::VE()),
                                    reason: "inferred from assignment (RE→VE upgrade)".to_string(),
                                };
                                node.depends_on.clear();
                            }
                            return Some(Ok(()));
                        }

                        let scope_str = if ctx.scope_path.is_empty() {
                            "global scope".to_string()
                        } else {
                            format!("'{}'", ctx.scope_path.join(" > "))
                        };
                        let first_assign_hint = resolver.nodes.get(&var_slot)
                            .and_then(|n| n.assigned_at.as_ref())
                            .map(|loc| format!("\n│  📍 First assigned at:  {}", loc))
                            .unwrap_or_default();
                        let second_assign_hint = format!("\n│  📍 Then assigned at:   {}", source_location);
                        return Some(Err(anyhow!(
                            "\n╭─ Type Conflict ──────────────────────────────────────────\n\
                                │\n\
                                │  Variable '{}' (in {}) is assigned conflicting types:\n\
                                │     • First inferred as:  {}\n\
                                │     • Then assigned as:   {}{}{}\n\
                                │\n\
                                │  Type elision requires each variable to have exactly one type.\n\
                                │\n\
                                │  💡 Either:\n\
                                │     • Add an explicit type:  VARIABLE ({:?}) {} ;\n\
                                │     • Split into separate variables: {}_{:?}  and  {}_{:?}\n\
                                │\n\
                                ╰──────────────────────────────────────────────────────────",
                            var_name, scope_str, old_type, new_type,
                            first_assign_hint, second_assign_hint,
                            old_type.base_type, var_name,
                            var_name, old_type.base_type, var_name, new_type.base_type,
                        )));
                    }
                }
            }

            // Keep the first (non-self-referential) assignment's recipe.
            // Subsequent assignments are mutations — the variable's type
            // is established by its first assignment. This prevents false
            // cycles when a variable is re-assigned from a value that
            // transitively depends on itself (e.g. X1 := f(X3) after
            // X3 := g(X1)).
            return Some(Ok(()));
        }

        if let Some(node) = resolver.nodes.get_mut(&var_slot) {
            // Remove self-reference from deps if present — the variable's
            // type is being established by this very assignment
            deps.remove(&var_slot);
            node.rule = ResolutionRule::InferredFrom {
                recipe,
                reason: "inferred from assignment".to_string(),
            };
            node.depends_on = deps;
            node.assigned_at = Some(source_location);
        }

        Some(Ok(()))
    }
}
impl Transpile for AssignStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Get the variable type and ensure the value type is compatible
        let variable_type = self.identifier.type_of(context)
            .map_err(|e| vec!(
                e.context("...while determining type of variable identifier for assignment")
            ))?;
        let value_type = self.value.type_of(context)
            .map_err(|e| vec!(
                e.context("...while determining type of value expression for assignment")
            ))?;
        // Check for RE→VE coercion: if variable is VE and value is RE,
        // we'll wrap the value in vec![...] to create a one-element vector.
        // This supports COSY's dynamic typing pattern (e.g. Y:=1; Y:=Y&I;).
        let needs_re_to_ve_coercion = variable_type.base_type == RosyBaseType::VE
            && variable_type.dimensions == 0
            && value_type == RosyType::RE();

        if variable_type != value_type && !needs_re_to_ve_coercion {
            return Err(vec!(anyhow!(
                "Cannot assign value of type '{}' to variable '{}' of type '{}'!", 
                value_type, self.identifier.name, variable_type
            )));
        }

        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        
        // Serialize the identifier
        let serialized_identifier = match self.identifier.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(vec_err) => {
                for err in vec_err {
                    errors.push(err.context(format!(
                        "...while transpiling identifier expression for assigment to '{}'", self.identifier.name
                    )));
                }
                String::new() // dummy value to collect more errors
            }
        };

        // Serialize the value
        let serialized_value = match self.value.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables);
                output.serialization
            },
            Err(value_errors) => {
                for err in value_errors {
                    errors.push(err.context(format!(
                        "...while transpiling value expression for assignment to '{}'", self.identifier.name
                    )));
                }
                String::new() // dummy value to collect more errors
            }
        };

        // If RE→VE coercion is needed, wrap the value in vec![...]
        // We dereference with (*...).to_owned() since the RE expression
        // may be a reference (e.g. &mut 1f64 or &TEMP).
        let serialized_value = if needs_re_to_ve_coercion {
            format!("vec![(*{}).to_owned()]", serialized_value)
        } else {
            serialized_value
        };

        // Serialize the entire function
        let dereference = match context.variables.get(&self.identifier.name)
            .ok_or(vec!(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.identifier.name)))? 
            .scope
        {
            VariableScope::Local => "",
            VariableScope::Arg => "*",
            VariableScope::Higher => {
                // Also add to requested variables
                requested_variables.insert(self.identifier.name.clone());
                "*"
            }
        };
        let serialization = format!(
            "{}{} = ({}).to_owned();",
            dereference, serialized_identifier, serialized_value
        );
        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables,
                ..Default::default()
            })
        } else {
            Err(errors)
        }
    }
}