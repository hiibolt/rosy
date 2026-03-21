//! # Variable Expressions & Function Call Disambiguation
//!
//! A `variable_identifier` in the parse tree can represent either a plain
//! variable access (with optional indexing) or a user-defined function call.
//!
//! At transpile time, [`VarExpr::classify`] applies a decision tree to
//! determine which interpretation is correct based on scope context.
//!
//! ## Decision Tree
//!
//! | Paren Groups | Args per Group | Bracket Indices | Result |
//! |-------------|----------------|-----------------|--------|
//! | 0 | — | any | Variable |
//! | 1 | multiple | — | Function call |
//! | 1 | 1 | — | Context-dependent (see below) |
//! | ≥2 | 1 each | — | Multi-dim index (Variable) |
//!
//! ### Single-arg disambiguation (1 paren group, 1 arg)
//!
//! | Variable? | Function? | Variable dims | Result |
//! |-----------|-----------|---------------|--------|
//! | yes | no | — | Variable (index) |
//! | no | yes | — | Function call |
//! | yes | yes | >0 (array) | Variable (index) |
//! | yes | yes | 0 (scalar) | Function call (recursion) |
//! | no | no | — | Error |
//!
//! ## Example
//!
//! ```text
//! X              { Variable }
//! X(3)           { Variable with 1D index, OR function call — resolved by context }
//! X(I)(J)        { Variable with 2D indexing }
//! MYFUNC(a, b)   { Function call (multiple args) }
//! X[I,J]         { Variable with bracket indexing }
//! ```

use crate::ast::{FromRule, Rule};
use super::variable_identifier::VariableIdentifier;
use crate::transpile::TranspileableExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput, VariableScope, ValueKind};
use anyhow::{Result, Context, Error, anyhow};
use crate::rosy_lib::RosyType;
use crate::program::expressions::Expr;
use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// What a `variable_identifier` AST node actually represents,
/// determined at transpile time via the decision tree.
#[derive(Debug)]
pub enum VarExprKind {
    /// Plain variable or variable with indexing: `X`, `X(I)`, `X(I)(J)`, `X[I,J]`
    Variable,
    /// Function call: `FUNC(a, b)` or `FUNC(x)` when FUNC is a known function
    FunctionCall,
}

#[derive(Debug, PartialEq)]
pub struct VarExpr {
    pub identifier: VariableIdentifier,
}

impl VarExpr {
    /// Apply the disambiguation decision tree:
    ///
    /// - Multiple paren groups → multi-dimensional indexing (Variable)
    /// - One paren group with multiple args → function call
    /// - One paren group with one arg → check context: function wins if only function matches,
    ///   variable wins if only variable matches
    /// - No paren groups → variable
    /// - Any invalid combo → error
    pub fn classify(&self, context: &TranspilationInputContext) -> Result<VarExprKind, Vec<Error>> {
        let ident = &self.identifier;
        let num_groups = ident.paren_groups.len();
        let has_brackets = !ident.bracket_indices.is_empty();

        match num_groups {
            0 => {
                // No parens — always a variable (possibly with bracket indices)
                Ok(VarExprKind::Variable)
            }
            1 => {
                let num_args = ident.paren_groups[0].len();
                if num_args > 1 {
                    // Multiple args in one paren group → function call
                    // But must not also have bracket indices
                    if has_brackets {
                        return Err(vec![anyhow::anyhow!(
                            "'{}': function call with bracket indexing is not valid", ident.name
                        )]);
                    }
                    Ok(VarExprKind::FunctionCall)
                } else {
                    // Single paren group, single arg → check context
                    let is_var = context.variables.contains_key(&ident.name);
                    let is_func = context.functions.contains_key(&ident.name);

                    match (is_var, is_func) {
                        (true, false) => Ok(VarExprKind::Variable),
                        (false, true) => {
                            if has_brackets {
                                return Err(vec![anyhow::anyhow!(
                                    "'{}': function call with bracket indexing is not valid", ident.name
                                )]);
                            }
                            Ok(VarExprKind::FunctionCall)
                        }
                        (true, true) => {
                            // Both exist — disambiguate by checking variable dimensions.
                            // A scalar variable (0 dimensions) cannot be indexed, so
                            // parentheses must be a function call (e.g. recursion where
                            // the function name doubles as the return variable).
                            let var_data = context.variables.get(&ident.name).unwrap();
                            if var_data.data.r#type.dimensions > 0 {
                                // Variable is an array — prefer indexing
                                Ok(VarExprKind::Variable)
                            } else {
                                // Variable is a scalar — can't index, must be a function call
                                if has_brackets {
                                    return Err(vec![anyhow::anyhow!(
                                        "'{}': function call with bracket indexing is not valid", ident.name
                                    )]);
                                }
                                Ok(VarExprKind::FunctionCall)
                            }
                        }
                        (false, false) => {
                            Err(vec![anyhow::anyhow!(
                                "'{}' is neither a defined variable nor a defined function in this scope!", ident.name
                            )])
                        }
                    }
                }
            }
            _ => {
                // Multiple paren groups → multi-dimensional indexing
                // Validate: each group must have exactly one arg
                for (i, group) in ident.paren_groups.iter().enumerate() {
                    if group.len() != 1 {
                        return Err(vec![anyhow::anyhow!(
                            "'{}': paren group {} has {} args — multi-dimensional indexing requires exactly 1 arg per group",
                            ident.name, i + 1, group.len()
                        )]);
                    }
                }
                Ok(VarExprKind::Variable)
            }
        }
    }
}

impl FromRule for VarExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::variable_identifier, "Expected variable_identifier rule, got {:?}", pair.as_rule());
        let identifier = VariableIdentifier::from_rule(pair)
            .context("Failed to build variable identifier!")?
            .ok_or_else(|| anyhow::anyhow!("Expected variable identifier"))?;
        Ok(Some(VarExpr { identifier }))
    }
}
impl TranspileableExpr for VarExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        match self.classify(context).map_err(|errs| {
            errs.into_iter().next().unwrap_or_else(|| anyhow::anyhow!("Unknown classification error"))
        })? {
            VarExprKind::FunctionCall => {
                let func_ctx = context.functions.get(&self.identifier.name)
                    .ok_or_else(|| anyhow::anyhow!("Function '{}' not found", self.identifier.name))?;
                Ok(func_ctx.return_type.clone())
            }
            VarExprKind::Variable => {
                self.identifier.type_of(context)
                    .context(format!(
                        "...while determining type of variable identifier '{}'", self.identifier.name
                    ))
            }
        }
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, ctx: &ScopeContext, deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        let ident = &self.identifier;
        if let Some(slot) = ctx.variables.get(&ident.name) {
            deps.insert(slot.clone());
            Some(ExprRecipe::Variable(slot.clone()))
        } else {
            Some(ExprRecipe::Unknown)
        }
    }
}
impl Transpile for VarExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        match self.classify(context)? {
            VarExprKind::FunctionCall => {
                // Delegate to function call helper — args are the single paren group
                function_call_transpile_helper(
                    &self.identifier.name,
                    &self.identifier.paren_groups[0],
                    context
                )
            }
            VarExprKind::Variable => {
                let ident_output = self.identifier.transpile(context)
                    .map_err(|e| e.into_iter().map(|err| {
                        err.context(format!(
                            "...while transpiling variable identifier '{}'", self.identifier.name
                        ))
                    }).collect::<Vec<Error>>())?;

                let var_data = context.variables.get(&self.identifier.name)
                    .ok_or(vec!(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.identifier.name)))?;
                let var_type = var_data.data.r#type.clone();

                let (reference, value_kind) = match var_data.scope {
                    VariableScope::Local => {
                        if var_type.is_copy() {
                            ("", ValueKind::Owned)   // Copy local: just `X`, value is copied
                        } else {
                            ("&", ValueKind::Ref)    // non-Copy local: `&X`, reference
                        }
                    },
                    VariableScope::Arg => ("", ValueKind::Ref),    // already a reference
                    VariableScope::Higher => ("", ValueKind::Ref),  // already a reference
                };
                Ok(TranspilationOutput {
                    serialization: format!("{}{}", reference, ident_output.serialization),
                    requested_variables: ident_output.requested_variables,
                    value_kind,
                })
            }
        }
    }
}

pub fn function_call_transpile_helper (
    name: &String,
    args: &Vec<Expr>,
    context: &mut TranspilationInputContext
) -> Result<TranspilationOutput, Vec<Error>> {
    // Start by checking that the function exists
    let func_context = match context.functions.get(name) {
        Some(ctx) => ctx,
        None => return Err(vec!(anyhow!("Function '{}' is not defined in this scope, can't transpile function call!", name)))
    }.clone();

    // Check that the number of arguments is correct
    if func_context.args.len() != args.len() {
        return Err(vec!(anyhow!(
            "Function '{}' expects {} arguments, but {} were provided!",
            name, func_context.args.len(), args.len()
        )));
    }
    let mut errors = Vec::new();
    let mut requested_variables = BTreeSet::new();
    let mut serialized_args = Vec::new();
    // Serialize the requested variables from the function context
    for var in &func_context.requested_variables {
        let var_data = context.variables.get(var)
            .ok_or(vec!(anyhow!(
                "Could not find variable '{}' requested by function '{}'",
                var, name
            )))?;
        
        let serialized_arg = match var_data.scope {
            VariableScope::Higher => format!("{}", var),
            VariableScope::Arg => format!("{}", var),
            VariableScope::Local => format!("&mut {}", var)
        };
        serialized_args.push(serialized_arg);
    }

    // Add the manual arguments
    for (i, arg_expr) in args.iter().enumerate() {
        match arg_expr.transpile(context) {
            Ok(arg_output) => {
                // Check the type is correct
                let provided_type = arg_expr.type_of(context)
                    .map_err(|e| vec!(e))?;
                let expected_type = func_context
                    .args
                    .get(i)
                    .ok_or(vec!(anyhow!(
                        "Function '{}' expects {} arguments, but {} were provided!",
                        name, func_context.args.len(), args.len()
                    )))?
                    .r#type
                    .clone();
                if provided_type != expected_type {
                    errors.push(anyhow!(
                        "Function '{}' expects argument {} ('{}') to be of type '{}', but type '{}' was provided!",
                        name, i+1, func_context.args[i].name, expected_type, provided_type
                    ));
                } else {
                    // If the type is correct, add the serialization
                    // Functions take &T args, so use as_ref()
                    serialized_args.push(arg_output.as_ref());
                    requested_variables.extend(arg_output.requested_variables);
                }
            },
            Err(arg_errors) => {
                for e in arg_errors {
                    errors.push(e.context(format!(
                        "...while transpiling argument {} for function '{}'", i+1, name
                    )));
                }
            }
        }
    }

    // Serialize the function call.
    // Uses the `__fn_` prefix to match the generated Rust function name
    // (the prefix avoids shadowing by the implicit return variable).
    let rust_fn_name = format!("__fn_{}", name);
    let serialization = format!(
        "({}({})? as {})",
        rust_fn_name, serialized_args.join(", "), func_context.return_type.as_rust_type()
    );
    if errors.is_empty() {
        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            value_kind: ValueKind::Owned,
        })
    } else {
        Err(errors)
    }
}