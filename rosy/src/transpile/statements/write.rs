use std::collections::BTreeSet;

use crate::ast::*;
use super::super::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};


impl Transpile for WriteStatement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut serialized_exprs = Vec::new();
        let mut requested_variables = BTreeSet::new();
        for expr in &self.exprs {
            // Second, transpile the expression
            let expr_as_st = Expr::StringConvert(StringConvertExpr { 
                expr: Box::new(expr.clone()) 
            });
            let TranspilationOutput {
                serialization: serialized_expr,
                requested_variables: expr_requested_variables
            } = expr_as_st.transpile(context)
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