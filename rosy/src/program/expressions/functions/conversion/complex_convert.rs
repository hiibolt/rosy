use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileWithType;
use crate::transpile::{Transpile, TypeOf, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Error, Context};
use crate::rosy_lib::RosyType;

#[derive(Debug, PartialEq)]
pub struct ComplexConvertExpr {
    pub expr: Box<Expr>,
}

impl FromRule for ComplexConvertExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::cm, "Expected cm rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `CM`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `CM`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `CM`"))?);
        Ok(Some(ComplexConvertExpr { expr }))
    }
}
impl TranspileWithType for ComplexConvertExpr {}
impl TypeOf for ComplexConvertExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let expr_type = self.expr.type_of(context)
            .map_err(|e| e.context("...while determining type of expression for complex conversion"))?;
        let result_type = crate::rosy_lib::intrinsics::cm::get_return_type(&expr_type)
            .ok_or(anyhow::anyhow!(
                "Cannot convert type '{}' to 'CM'!",
                expr_type
            ))?;
        Ok(result_type)
    }
}
impl Transpile for ComplexConvertExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, ensure the type is convertible to CM
        //
        // Sneaky way to check that the type is compatible :3
        let _ = self.type_of(context)
            .map_err(|e| vec!(e.context("...while verifying types of complex conversion expression")))?;

        // Then, transpile the expression
        let TranspilationOutput {
            serialization: expr_serialization,
            requested_variables
        } = self.expr.transpile(context)
            .map_err(|e| e.into_iter().map(|err| {
                err.context("...while transpiling expression for CM conversion")
            }).collect::<Vec<Error>>())?;

        // Finally, serialize the conversion
        let serialization = format!("&mut RosyCM::rosy_cm(&*{}).context(\"...while trying to convert to (CM)\")?", expr_serialization);
        Ok(TranspilationOutput {
            serialization,
            requested_variables
        })
    }
}