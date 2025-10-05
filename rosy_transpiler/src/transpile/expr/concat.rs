use std::collections::BTreeSet;

use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow};
use rosy_lib::RosyType;

impl TypeOf for ConcatExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let mut terms = self.terms.clone();

        let mut r#type = match terms.pop() {
            Some(term) => term.type_of(context)?,
            None => return Err(anyhow::anyhow!("Cannot concatenate zero terms!"))
        };
        while let Some(term_expr) = terms.pop() {
            let term_type = term_expr.type_of(context)?;
            r#type = rosy_lib::operators::concat::get_return_type(&r#type, &term_type)
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
            let mut terms = self.terms.clone();

            // Serialize the first term as a base
            let _ = terms.get(0)
                .ok_or(vec!(anyhow!("Concatenation expression must have at least one term!")))?;
            let first_term = terms.remove(0);
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
            for (i, term) in terms.into_iter().enumerate() {
                serialization = format!(
                    "&mut RosyConcat::rosy_concat(&*{}, &*{})",
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