use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileWithType, TypeOf};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;

/// LST(n) - String memory size estimator (COSY compatibility).
/// Returns n as RE (Rosy doesn't need memory management, but returns a value
/// for backwards compatibility).
#[derive(Debug, PartialEq)]
pub struct LstExpr {
    pub expr: Box<Expr>,
}

impl FromRule for LstExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::lst, "Expected lst rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `LST`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `LST`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `LST`"))?);
        Ok(Some(LstExpr { expr }))
    }
}
impl TranspileWithType for LstExpr {}
impl Transpile for LstExpr {
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
            "&mut RosyLST::rosy_lst(&*{})",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TypeOf for LstExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::RE())
    }
}
