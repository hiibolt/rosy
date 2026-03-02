use crate::ast::{FromRule, Rule};
use crate::program::expressions::variable_identifier::VariableIdentifier;
use crate::program::expressions::function_call::function_call_transpile_helper;
use crate::transpile::TranspileWithType;
use crate::transpile::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput, VariableScope, };
use anyhow::{Result, Context, Error};
use crate::rosy_lib::RosyType;

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
                            // Both exist — prefer variable (indexing), since function calls
                            // typically have multiple arguments
                            Ok(VarExprKind::Variable)
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
impl TranspileWithType for VarExpr {}
impl TypeOf for VarExpr {
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
}
impl Transpile for VarExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
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
                let TranspilationOutput {
                    serialization: serialized_identifier,
                    requested_variables
                } = self.identifier.transpile(context)
                    .map_err(|e| e.into_iter().map(|err| {
                        err.context(format!(
                            "...while transpiling variable identifier '{}'", self.identifier.name
                        ))
                    }).collect::<Vec<Error>>())?;
                
                let reference = match context.variables.get(&self.identifier.name)
                    .ok_or(vec!(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.identifier.name)))? 
                    .scope
                {
                    VariableScope::Local => "&",
                    VariableScope::Arg => "",
                    VariableScope::Higher => ""
                };
                Ok(TranspilationOutput {
                    serialization: format!("{}{}", reference, serialized_identifier),
                    requested_variables
                })
            }
        }
    }
}