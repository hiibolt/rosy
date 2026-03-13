//! # LENGTH Function
//!
//! Returns the length/size of a value. For vectors, returns the number of
//! elements. For strings, returns the character count. For scalars, returns 1.
//!
//! ## Syntax
//!
//! ```text
//! LENGTH(expr)
//! ```
//!
//! ## Type Compatibility
//!
//! | Input | Result |
//! |-------|--------|
//! | RE | RE |
//! | ST | RE |
//! | LO | RE |
//! | CM | RE |
//! | VE | RE |
//! | DA | RE |
//! | CD | RE |
//!
//! ## Example
//!
//! ```text
//! VARIABLE (VE) v;
//! VARIABLE (RE) n;
//! v := 1 & 2 & 3;
//! n := LENGTH(v);        { 3.0 }
//! ```

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use std::collections::{BTreeSet, HashSet};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the `LENGTH(expr)` system function.
#[derive(Debug, PartialEq)]
pub struct LengthExpr {
    pub expr: Box<Expr>,
}

impl FromRule for LengthExpr {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::length, "Expected length rule, got {:?}", pair.as_rule());
        let mut inner = pair.into_inner();
        let expr_pair = inner.next()
            .context("Missing inner expression for `LENGTH`!")?;
        let expr = Box::new(Expr::from_rule(expr_pair)
            .context("Failed to build expression for `LENGTH`")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for `LENGTH`"))?);
        Ok(Some(LengthExpr { expr }))
    }
}
impl Transpile for LengthExpr {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the inner expression
        let inner_output = self.expr.transpile(context)?;
        
        // Combine requested variables
        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(inner_output.requested_variables);

        // Generate the transpiled code
        let serialization = format!(
            "&mut RosyLENGTH::rosy_length(&*{})",
            inner_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
impl TranspileableExpr for LengthExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        use crate::rosy_lib::intrinsics::length;

        // Get the type of the inner expression
        let inner_type = self.expr.type_of(context)
            .context("Failed to determine type of inner expression in LENGTH")?;

        // Use the LENGTH registry to get the return type
        length::get_return_type(&inner_type)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "LENGTH not supported for type: {:?}",
                    inner_type
                )
            })
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::RE()))
    }
}