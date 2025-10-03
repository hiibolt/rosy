use std::collections::BTreeSet;

use crate::{ast::*, transpile::TypeOf};
use super::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};
use rosy_lib::RosyType;


impl Transpile for WriteStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut serialized_exprs = Vec::new();
        let mut requested_variables = BTreeSet::new();
        for expr in &self.exprs {
            // First, ensure the expression is an ST
            let expr_type = expr.type_of(context)
                .map_err(|e| vec!(e.context("...while determining type of expression in WRITE statement")))?;
            if expr_type != RosyType::ST() {
                return Err(vec!(anyhow!(
                    "Cannot WRITE expression of type '{}'! Only expressions of type 'ST' can be written.",
                    expr_type
                )));
            }

            // Second, transpile the expression
            let TranspilationOutput {
                serialization: serialized_expr,
                requested_variables: expr_requested_variables
            } = expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling expression in WRITE statement")
                }).collect::<Vec<Error>>())?;
            serialized_exprs.push(serialized_expr);
            requested_variables.extend(expr_requested_variables);
        }

        // Emulate the checking of the unit
        match self.unit {
            6 => {},
            _ => return Err(vec!(anyhow!(
                "Only WRITE to unit 6 (standard output) is supported, found unit {}!", self.unit
            ))),
        }

        // Serialize the entire function
        let serialization = format!(
            "println!(\"{}\", {});",
            serialized_exprs.iter().map(|_| "{}").collect::<Vec<&str>>().join(""),
            serialized_exprs.join(", ")
        );
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}