use std::collections::BTreeSet;

use crate::{ast::*, transpile::{add_context_to_all, shared::string_convert::string_convert_transpile_helper}};
use super::super::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};


impl Transpile for WriteStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut serialized_exprs = Vec::new();
        let mut requested_variables = BTreeSet::new();
        for expr in &self.exprs {
            let TranspilationOutput {
                serialization: serialized_expr,
                requested_variables: expr_requested_variables
            } = string_convert_transpile_helper(expr, context)
                .map_err(|err_vec| {
                    add_context_to_all(err_vec, format!(
                        "...while transpiling expression '{:?}' for WRITE statement", expr
                    ))
                })?;

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