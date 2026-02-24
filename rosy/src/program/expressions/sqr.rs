use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileWithType, TypeOf};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct SqrExpr {
    pub expr: Box<Expr>,
}

impl FromRule for SqrExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::sqr, "Expected sqr rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `SQR`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `SQR`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `SQR`"))?);
        Ok(Some(SqrExpr { expr }))
    }
}
impl TranspileWithType for SqrExpr {}
impl Transpile for SqrExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the inner expression
        let inner_output = self.expr.transpile(context)?;
        
        // Combine requested variables
        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        // Generate the transpiled code
        let serialization = format!(
            "&mut RosySQR::rosy_sqr(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TypeOf for SqrExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::sqr;
        
        // Get the type of the inner expression
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in SQR")?;
        
        // Use the SQR registry to get the return type
        sqr::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "SQR not supported for type: {:?}",
                    inner_type
                )
            })
    }
}
