use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, anyhow, ensure};

use crate::{
    ast::*,
    transpile::{Transpile, TranspilationInputContext, TranspilationOutput, add_context_to_all, shared::string_convert::string_convert_transpile_helper}
};

impl StatementFromRule for WriteStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
        ensure!(pair.as_rule() == Rule::write, 
            "Expected `write` rule when building write statement, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();

        let unit = inner.next()
            .context("Missing first token `unit`!")?
            .as_str()
            .parse::<u8>()
            .context("Failed to parse `unit` as u8 in `write` statement!")?;

        let exprs = {
            let mut exprs = Vec::new();
            while let Some(expr_pair) = inner.next() {
                if expr_pair.as_rule() == Rule::semicolon {
                    break;
                }

                let expr = build_expr(expr_pair)
                    .context("Failed to build expression in `write` statement!")?;
                exprs.push(expr);
            }
            exprs
        };

        Ok(Some(Statement {
            enum_variant: StatementEnum::Write,
            inner: Box::new(WriteStatement { unit, exprs })
        }))
    }
}

impl Transpile for WriteStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
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
