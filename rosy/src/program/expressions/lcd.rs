use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileWithType, TypeOf};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::BTreeSet;

/// LCD(ve) - DA memory size estimator (COSY compatibility).
/// Takes a VE with (order & num_vars) and returns estimated DA memory size.
/// Rosy doesn't need memory management, but returns a reasonable value.
#[derive(Debug, PartialEq)]
pub struct LcdExpr {
    pub expr: Box<Expr>,
}

impl FromRule for LcdExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::lcd, "Expected lcd rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `LCD`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `LCD`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `LCD`"))?);
        Ok(Some(LcdExpr { expr }))
    }
}
impl TranspileWithType for LcdExpr {}
impl Transpile for LcdExpr {
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
            "&mut RosyLCD::rosy_lcd(&*{})",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TypeOf for LcdExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::RE())
    }
}
