use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use std::collections::BTreeSet;
use anyhow::{Result, Error};
use crate::rosy_lib::{RosyBaseType, RosyType};

impl Transpile for VariableData {
    // note that this transpiles as the default value for the type
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let base_value = match self.r#type.base_type {
            RosyBaseType::RE => "0.0",
            RosyBaseType::ST => "\"\".to_string()",
            RosyBaseType::LO => "false",
            RosyBaseType::CM => "(0.0, 0.0)",
            RosyBaseType::VE => "vec![]",
            RosyBaseType::DA => "DA::new()"
        }.to_string();

        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        let serialization = if self.dimension_exprs.is_empty() {
            base_value
        } else {
            let mut result = base_value;
            for dim in self.dimension_exprs.iter().rev() {
                // ensure the type compiles down to a RE
                if let Err(e) = dim.type_of(context).and_then(|t| {
                    let expected_type = RosyType::RE();
                    if t == expected_type {
                        Ok(())
                    } else {
                        Err(anyhow::anyhow!("Array dimension expression must be of type {expected_type}, found {t}"))
                    }
                }) {
                    errors.push(e.context("...while checking array dimension expression type"));
                    continue;
                }

                // transpile each dimension expression
                match dim.transpile(context) {
                    Ok(output) => {
                        result = format!("vec![{}; ({}).to_owned() as usize]", result, output.serialization);
                        requested_variables.extend(output.requested_variables);
                    },
                    Err(dim_errors) => {
                        for e in dim_errors {
                            errors.push(e.context("...while transpiling array dimension expression!"));
                        }
                    }
                }
            }
            result
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