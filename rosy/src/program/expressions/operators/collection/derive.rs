//! # Derivation Operator (`%`)
//!
//! Computes partial derivatives or anti-derivatives of DA/CD Taylor series.
//!
//! ## Syntax
//!
//! ```text
//! da_expr % n       { partial derivative w.r.t. variable n (n > 0) }
//! da_expr % (-n)    { anti-derivative (integral) w.r.t. variable n }
//! ```
//!
//! ## Supported Types
//!
//! | Object | Result |
//! |--------|--------|
//! | DA | DA |
//! | CD | CD |
//!
//! ## Example
//!
//! ```text
//! VARIABLE (DA) f;
//! VARIABLE (DA) df;
//! f := DA(1) * DA(1) + DA(2);  { f = x² + y }
//! df := f % 1;                  { df/dx = 2x }
//! ```

use crate::program::expressions::Expr;
use crate::transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableExpr};
use crate::rosy_lib::RosyType;
use anyhow::{Result, Error, Context as AnyhowContext};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe, BinaryOpKind};
use std::collections::{BTreeSet, HashSet};

/// DA%n = partial derivative w.r.t. variable n (positive n)
/// DA%(-n) = anti-derivative (integral) w.r.t. variable n (negative n)
#[derive(Debug, PartialEq)]
pub struct DeriveExpr {
    pub object: Box<Expr>,
    pub index: Box<Expr>,
}

impl Transpile for DeriveExpr {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let object_output = self.object.transpile(context)?;
        let index_output = self.index.transpile(context)?;

        let mut requested_variables = BTreeSet::new();
        requested_variables.extend(object_output.requested_variables);
        requested_variables.extend(index_output.requested_variables);

        // Generate code that checks the sign of the index at runtime:
        // positive => derivative, negative => antiderivative
        let serialization = format!(
            "&mut RosyDerive::rosy_derive(&*{}, ({}).clone() as i64)?",
            object_output.serialization,
            index_output.serialization
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
impl TranspileableExpr for DeriveExpr {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType> {
        let object_type = self.object.type_of(context)
            .context("Failed to determine type of object in % (derive) expression")?;
        
        match object_type {
            t if t == RosyType::DA() => Ok(RosyType::DA()),
            t if t == RosyType::CD() => Ok(RosyType::CD()),
            _ => anyhow::bail!(
                "Derivation operator % not supported for type: {:?}. Only DA and CD are supported.",
                object_type
            ),
        }
    }
    fn build_expr_recipe(&self, resolver: &TypeResolver, ctx: &ScopeContext, deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        let left = resolver.build_expr_recipe(&self.object, ctx, deps);
        let right = resolver.build_expr_recipe(&self.index, ctx, deps);
        Some(ExprRecipe::BinaryOp { op: BinaryOpKind::Derive, left: Box::new(left), right: Box::new(right) })
    }
}
