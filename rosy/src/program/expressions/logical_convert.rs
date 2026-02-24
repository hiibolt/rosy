use crate::ast::{FromRule, Rule};
use crate::transpile::TranspileWithType;
use crate::program::expressions::Expr;
use crate::transpile::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, anyhow, Context};
use crate::rosy_lib::RosyType;

#[derive(Debug, PartialEq)]
pub struct LogicalConvertExpr {
    pub expr: Box<Expr>,
}

impl FromRule for LogicalConvertExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::lo, "Expected lo rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `LO`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `LO`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `LO`"))?);
        Ok(Some(LogicalConvertExpr { expr }))
    }
}
impl TranspileWithType for LogicalConvertExpr {}
impl TypeOf for LogicalConvertExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let expr_type = self.expr.type_of(context)?;
        crate::rosy_lib::intrinsics::lo::get_return_type(&expr_type)
            .ok_or(anyhow::anyhow!("Cannot convert type '{expr_type}' to 'LO'!"))
    }
}
impl Transpile for LogicalConvertExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, ensure the type is convertible to LO
        let expr_type = self.expr.type_of(context)
            .map_err(|e| vec!(e))?;
        let _ = crate::rosy_lib::intrinsics::lo::get_return_type(&expr_type)
            .ok_or(vec!(anyhow!(
                "Cannot convert type '{}' to 'LO'!", expr_type
            )))?;

        // Then, transpile the expression
        let TranspilationOutput {
            serialization: expr_serialization,
            requested_variables
        } = self.expr.transpile(context)
            .map_err(|e| e.into_iter().map(|err| {
                err.context("...while transpiling expression for LO conversion")
            }).collect::<Vec<Error>>())?;

        // Finally, serialize the conversion
        let serialization = format!("&mut RosyLO::rosy_to_logical(&*{})", expr_serialization);
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}