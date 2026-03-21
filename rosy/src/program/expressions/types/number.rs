//! # RE — Numeric Literal
//!
//! Real number literals parsed as `f64`. The `FromRule`, `TranspileableExpr`, and
//! `Transpile` traits are implemented directly on `f64`.
//!
//! ## Syntax
//!
//! ```text
//! 3.14
//! 42
//! -7
//! 1.23E-4
//! ```
//!
//! All numeric literals produce the `RE` type.

use std::collections::{BTreeSet, HashSet};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error};

use crate::{
    ast::{FromRule, Rule},
    rosy_lib::RosyType,
    transpile::{Transpile, TranspileableExpr, TranspilationInputContext, TranspilationOutput, ValueKind}
};

impl FromRule for f64 {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::number, "Expected number rule, got {:?}", pair.as_rule());
        let n = pair.as_str().parse::<f64>()?;
        Ok(Some(n))
    }
}
impl TranspileableExpr for f64 {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::RE())
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::RE()))
    }
}
impl Transpile for f64 {
    fn transpile(&self, _context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("{}f64", self),
            requested_variables: BTreeSet::new(),
            value_kind: ValueKind::Owned,
        })
    }
}
