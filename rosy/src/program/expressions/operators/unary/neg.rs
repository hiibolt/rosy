//! # Unary Negation (`-expr`)
//!
//! Negates a numeric value. Transpiled as `0 - expr` using the subtraction operator.
//!
//! ## Syntax
//!
//! ```text
//! -expr
//! ```
//!
//! ## Supported Types
//!
//! Any type that supports `RE - T`, including RE, CM, VE, DA, CD.
//!
//! ## Example
//!
//! ```text
//! VARIABLE (RE) x;
//! x := -5;               { unary negation }
//! x := -(3 + 4);         { negation of expression }
//! ```

use std::collections::BTreeSet;
use std::collections::HashSet;

use anyhow::{Result, Error, anyhow};
use crate::ast::{FromRule, Rule};
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use crate::transpile::{Transpile, TranspileableExpr, TranspilationInputContext, TranspilationOutput, ValueKind};
use crate::program::expressions::Expr;

/// Unary negation expression: `-expr`
/// Transpiled as `0 - expr` using the existing subtraction operator.
#[derive(Debug, PartialEq)]
pub struct NegExpr {
    pub operand: Box<Expr>,
}

impl FromRule for NegExpr {
    fn from_rule(_pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        // NegExpr is created directly in the Pratt parser's map_primary, not via FromRule
        anyhow::bail!("NegExpr should be created by the Pratt parser, not FromRule")
    }
}

impl TranspileableExpr for NegExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        // Negation has the same type as its operand (validated via subtraction from 0)
        let operand_type = self.operand.type_of(context)?;
        // Use the sub registry to check: RE - operand_type should work
        let zero_type = RosyType::RE();
        crate::rosy_lib::operators::sub::get_return_type(&zero_type, &operand_type)
            .ok_or_else(|| anyhow!(
                "Cannot negate type '{}' (no subtraction from RE defined)", operand_type
            ))
    }
    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        Some(resolver.discover_expr_function_calls(&self.operand, ctx))
    }
    fn build_expr_recipe(&self, resolver: &TypeResolver, ctx: &ScopeContext, deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(resolver.build_expr_recipe(&self.operand, ctx, deps))
    }
}

impl Transpile for NegExpr {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let operand_type = self.operand.type_of(context).map_err(|e| vec!(e))?;
        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        let operand_output = match self.operand.transpile(context) {
            Ok(output) => output,
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling operand of negation"));
                }
                TranspilationOutput::default()
            }
        };
        requested_variables.extend(operand_output.requested_variables.iter().cloned());

        use crate::rosy_lib::RosyBaseType;
        let serialization = if operand_type.base_type == RosyBaseType::RE && operand_type.dimensions == 0 {
            format!("(-{})", operand_output.as_value())
        } else {
            format!("RosySub::rosy_sub(&0.0f64, {})?", operand_output.as_ref())
        };

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
}

