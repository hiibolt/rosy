use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileWithType, TypeOf};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;

#[derive(Debug, PartialEq)]
pub struct VmaxExpr {
    pub expr: Box<Expr>,
}

impl FromRule for VmaxExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::vmax, "Expected vmax rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `VMAX`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `VMAX`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `VMAX`"))?);
        Ok(Some(VmaxExpr { expr }))
    }
}
impl TranspileWithType for VmaxExpr {}
impl Transpile for VmaxExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let inner_output = self.expr.transpile(context)?;
        
        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        let serialization = format!(
            "&mut RosyVMAX::rosy_vmax(&*{})?",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TypeOf for VmaxExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in VMAX")?;
        
        match inner_type {
            t if t == RosyType::VE() => Ok(RosyType::RE()),
            _ => anyhow::bail!(
                "VMAX not supported for type: {:?}. Only VE is supported.",
                inner_type
            ),
        }
    }
}
