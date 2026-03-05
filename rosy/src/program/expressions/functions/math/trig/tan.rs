use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileWithType, TypeOf};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct TanExpr {
    pub expr: Box<Expr>,
}

impl FromRule for TanExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::tan_fn, "Expected tan_fn rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `TAN`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `TAN`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `TAN`"))?);
        Ok(Some(TanExpr { expr }))
    }
}
impl TranspileWithType for TanExpr {}
impl Transpile for TanExpr {
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
            "&mut RosyTAN::rosy_tan(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TypeOf for TanExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::tan;
        
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in TAN")?;
        
        tan::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "TAN not supported for type: {:?}",
                    inner_type
                )
            })
    }
}
