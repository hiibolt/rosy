use std::collections::BTreeSet;

use anyhow::{Result, Error, anyhow};
use crate::ast::{FromRule, Rule};
use crate::rosy_lib::RosyType;
use crate::transpile::{Transpile, TranspileWithType, TypeOf, TranspilationInputContext, TranspilationOutput};
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

impl TranspileWithType for NegExpr {}

impl TypeOf for NegExpr {
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
}

impl Transpile for NegExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile as: RosySub::rosy_sub(&*0.0f64, &*operand)
        let mut serialization = String::from("&mut RosySub::rosy_sub(&*&mut 0.0f64, &*");
        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        match self.operand.transpile(context) {
            Ok(output) => {
                serialization.push_str(&output.serialization);
                requested_variables.extend(output.requested_variables);
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling operand of negation"));
                }
            }
        }
        serialization.push_str(")?");

        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables,
            })
        } else {
            Err(errors)
        }
    }
}

