use std::collections::BTreeSet;

use crate::ast::*;
use super::super::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error};
use rosy_lib::RosyType;

impl TypeOf for ExtractExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let object_type = self.object.type_of(context)
            .map_err(|e| e.context("...while determining type of object expression for extraction"))?;
        let index_type = self.index.type_of(context)
            .map_err(|e| e.context("...while determining type of index expression for extraction"))?;

        let result_type = rosy_lib::operators::extract::get_return_type(&object_type, &index_type)
            .ok_or(anyhow::anyhow!(
                "Cannot extract from type '{}' using index of type '{}'!",
                object_type, index_type
            ))?;

        Ok(result_type)
    }
}
impl Transpile for ExtractExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, ensure the types are compatible
        let _ = self.type_of(context)
            .map_err(|e| vec!(e.context("...while verifying types of extraction expression")))?;

        // Then, transpile both sides and combine
        let mut serialization = String::from("&mut RosyExtract::rosy_extract(&*");
        let mut errors = Vec::new();
        let mut requested_variables = BTreeSet::new();

        // Transpile object
        match self.object.transpile(context) {
            Ok(output) => {
                serialization.push_str(&output.serialization);
                requested_variables.extend(output.requested_variables);
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling object of extraction"));
                }
            }
        }

        // Transpile index
        serialization.push_str(", &*");
        match self.index.transpile(context) {
            Ok(output) => {
                serialization.push_str(&output.serialization);
                requested_variables.extend(output.requested_variables);
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling index of extraction"));
                }
            }
        }
        serialization.push_str(").context(\"...while trying to extract an element\")?");

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