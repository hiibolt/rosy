use crate::ast::{FromRule, Rule};
use crate::transpile::*;
use crate::program::expressions::Expr;
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, anyhow, Context};

#[derive(Debug, PartialEq)]
pub struct StringConvertExpr {
    pub expr: Box<Expr>,
}

impl FromRule for StringConvertExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::st, "Expected st rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `ST`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `ST`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `ST`"))?);
        Ok(Some(StringConvertExpr { expr }))
    }
}

impl TranspileWithType for StringConvertExpr {}
impl TypeOf for StringConvertExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let expr_type = self.expr.type_of(context)?;
        crate::rosy_lib::intrinsics::st::get_return_type(&expr_type)
            .ok_or(anyhow::anyhow!("Cannot convert type '{expr_type}' to 'ST'!"))
    }
}
impl Transpile for StringConvertExpr {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        string_convert_transpile_helper(&self.expr, context)
    }
}

pub fn string_convert_transpile_helper (
    expr: &Expr,
    context: &mut TranspilationInputContext
) -> Result<TranspilationOutput, Vec<Error>> {
    // First, ensure the type is convertible to ST
    let expr_type = expr.type_of(context)
        .map_err(|e| vec!(e))?;
    let _ = crate::rosy_lib::intrinsics::st::get_return_type(&expr_type)
        .ok_or(vec!(anyhow!(
            "Cannot convert type '{}' to 'ST'!", expr_type
        )))?;

    // Then, transpile the expression
    let TranspilationOutput {
        serialization: expr_serialization,
        requested_variables
    } = expr.transpile(context)
        .map_err(|e| e.into_iter().map(|err| {
            err.context("...while transpiling expression for STRING conversion")
        }).collect::<Vec<Error>>())?;

    // Finally, serialize the conversion
    let serialization = format!("&mut RosyST::rosy_to_string(&*{})", expr_serialization);
    Ok(TranspilationOutput {
        serialization,
        requested_variables
    })
}