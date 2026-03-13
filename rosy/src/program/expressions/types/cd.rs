//! # CD — Complex Differential Algebra Constructor
//!
//! Creates a CD (Complex Differential Algebra / complex Taylor series)
//! value from a variable index.
//!
//! ## Syntax
//!
//! ```text
//! CD(n)          { creates CD identity for variable n }
//! ```
//!
//! ## Example
//!
//! ```text
//! VARIABLE (CD) g;
//! g := CD(1) + CD(2);    { g = x + y (complex Taylor) }
//! ```

use crate::{
    ast::{FromRule, Rule},
    program::expressions::Expr,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr}
};
use anyhow::{Error, Context};
use std::collections::HashSet;
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `CD(n)` constructor expression.
#[derive(Debug, PartialEq)]
pub struct CDExpr {
    pub index: Box<Expr>,
}

impl FromRule for CDExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> anyhow::Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::cd_intrinsic, "Expected cd_intrinsic rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `CD`!")?;
        let index = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `CD`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `CD`"))?);
        Ok(Some(CDExpr { index }))
    }
}
impl TranspileableExpr for CDExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> anyhow::Result<RosyType> {
        Ok(RosyType::CD())
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::CD()))
    }
}
impl Transpile for CDExpr {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the index expression
        let index_output = self.index.transpile(context)
            .map_err(|errs| {
                errs.into_iter()
                    .map(|e| e.context("...while transpiling CD index expression"))
                    .collect::<Vec<_>>()
            })?;

        // Use CD::variable(usize) to create a CD differential variable
        let serialization = format!(
            "(&mut CD::variable(({}).clone() as usize)?)",
            index_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables: index_output.requested_variables,
        })
    }
}
