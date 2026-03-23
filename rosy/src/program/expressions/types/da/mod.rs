//! # DA — Differential Algebra Constructor
//!
//! Creates a DA (Differential Algebra / Taylor series) value from
//! a variable index.
//!
//! ## Syntax
//!
//! ```text
//! DA(n)          { creates DA identity for variable n }
//! ```
//!
//! ## Rosy Example
#![doc = include_str!("test.rosy")]
//! **Output**:
#![doc = include_str!("rosy_output.txt")]
//! ## COSY Example
#![doc = include_str!("test.fox")]
//! **Output**:
#![doc = include_str!("cosy_output.txt")]

use crate::{
    ast::{FromRule, Rule},
    program::expressions::Expr,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr, ValueKind}
};
use anyhow::{Error, Context};
use std::collections::HashSet;
use crate::rosy_lib::RosyType;
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `DA(n)` constructor expression.
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
impl TranspileableExpr for DAExpr {
    fn type_of(&self, _context: &TranspilationInputContext) -> anyhow::Result<RosyType> {
        Ok(RosyType::DA())
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::DA()))
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
        let serialization = format!(
            "DA::variable({} as usize)?",
            index_output.as_value()
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables: index_output.requested_variables,
            value_kind: ValueKind::Owned,
        })
    }
}