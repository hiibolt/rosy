use std::collections::BTreeSet;

use crate::program::expressions::Expr;
use crate::transpile::TranspileWithType;
use crate::transpile::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Context, Error, anyhow};
use crate::rosy_lib::RosyType;

#[derive(Debug, PartialEq)]
pub struct ConcatExpr {
    pub terms: Vec<Expr>
}

impl TranspileWithType for ConcatExpr {}
impl TypeOf for ConcatExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let mut r#type = self.terms.last()
            .ok_or(anyhow::anyhow!("Cannot concatenate zero terms!"))?
            .type_of(context)
            .context("...while determining type of last term in concatenation")?;

        for term_expr in self.terms.iter().rev().skip(1) {
            let term_type = term_expr.type_of(context)
                .context("...while determining type of term in concatenation")?;

            r#type = crate::rosy_lib::operators::concat::get_return_type(&r#type, &term_type)
                .ok_or(anyhow::anyhow!(
                    "Cannot concatenate types '{}' and '{}' together!",
                    r#type, term_type
                ))?;
        }

        Ok(r#type)
    }
}
impl Transpile for ConcatExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, do a type check 
        //
        // Sneaky way to check that all terms are compatible :3
        let _ = self.type_of(context)
            .map_err(|e| vec!(e.context("...while verifying types of concatenation expression")))?;

        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        let serialization = {
            // Serialize the first term as a base
            let first_term = self.terms.get(0)
                .ok_or(vec!(anyhow!("Concatenation expression must have at least one term!")))?;
            let mut serialization = match first_term.transpile(context) {
                Ok(output) => {
                    requested_variables.extend(output.requested_variables);
                    output.serialization
                },
                Err(mut e) => {
                    for err in e.drain(..) {
                        errors.push(err.context("...while transpiling first term of concatenation"));
                    }
                    String::new() // dummy value to collect more errors
                }
            };

            // Then, for each subsequent term, serialize and concatenate
            for (i, term) in self.terms.iter().skip(1).enumerate() {
                serialization = format!(
                    "&mut RosyConcat::rosy_concat(&*{}, &*{})?",
                    serialization,
                    match term.transpile(context) {
                        Ok(output) => {
                            requested_variables.extend(output.requested_variables);
                            output.serialization
                        },
                        Err(vec_err) => {
                            for err in vec_err {
                                errors.push(err.context(format!(
                                    "...while transpiling term {} of concatenation", i+2
                                )));
                            }
                            String::new() // dummy value to collect more errors
                        }
                    }
                );
            }
            
            serialization
        };
        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables
            })
        } else {
            Err(errors)
        }
    }
}