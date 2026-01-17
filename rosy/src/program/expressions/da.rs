use crate::{
    ast::{FromRule, Rule},
    program::expressions::Expr,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileWithType, TypeOf}
};
use anyhow::{Error, Context};
use crate::rosy_lib::RosyType;

#[derive(Debug, PartialEq)]
pub struct DAExpr {
    pub index: Box<Expr>,
}

impl FromRule for DAExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> anyhow::Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::da, "Expected da rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `DA`!")?;
        let index = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `DA`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `DA`"))?);
        Ok(Some(DAExpr { index }))
    }
}
impl TranspileWithType for DAExpr {}
impl TypeOf for DAExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> anyhow::Result<RosyType> {
        Ok(RosyType::DA())
    }
}
impl Transpile for DAExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the index expression
        let index_output = self.index.transpile(context)
            .map_err(|errs| {
                errs.into_iter()
                    .map(|e| e.context("...while transpiling DA index expression"))
                    .collect::<Vec<_>>()
            })?;

        // Use DA::variable(usize) to create a DA differential variable
        // Clone the index (which is &mut T) to get an owned value for casting
        let serialization = format!(
            "(&mut DA::variable(({}).clone() as usize)?)",
            index_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables: index_output.requested_variables,
        })
    }
}