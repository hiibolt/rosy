//! # LO — Boolean Literal
//!
//! Logical literals `TRUE` and `FALSE`. Produce the `LO` type.
//!
//! ## Syntax
//!
//! ```text
//! TRUE
//! FALSE
//! ```
//!
//! ```rosy_test_raw
//! --- rosy ---
//! BEGIN;
//!     VARIABLE (LO) B;
//!     B := TRUE;
//!     WRITE 6 B;
//!     B := FALSE;
//!     WRITE 6 B;
//! END;
//! --- fox ---
//! BEGIN;
//! PROCEDURE RUN;
//!     VARIABLE B 1;
//!     B := TRUE;
//!     WRITE 6 B;
//!     B := FALSE;
//!     WRITE 6 B;
//! ENDPROCEDURE;
//! RUN;
//! END;
//! ```

use std::collections::{BTreeSet, HashSet};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};
use anyhow::{Result, Error, bail};

use crate::{
    ast::{FromRule, Rule},
    rosy_lib::RosyType,
    transpile::{Transpile, TranspileableExpr, TranspilationInputContext, TranspilationOutput, ValueKind}
};

impl FromRule for bool {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        anyhow::ensure!(pair.as_rule() == Rule::boolean, "Expected boolean rule, got {:?}", pair.as_rule());
        let b = match pair.as_str() {
            "TRUE" => true,
            "FALSE" => false,
            _ => bail!("Unexpected boolean value: {}", pair.as_str()),
        };
        Ok(Some(b))
    }
}
impl TranspileableExpr for bool {
    fn type_of(&self, _context: &TranspilationInputContext) -> Result<RosyType> {
        Ok(RosyType::LO())
    }
    fn build_expr_recipe(&self, _resolver: &TypeResolver, _ctx: &ScopeContext, _deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        Some(ExprRecipe::Literal(RosyType::LO()))
    }
}
impl Transpile for bool {
    fn transpile(&self, _context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        Ok(TranspilationOutput {
            serialization: format!("{}", self),
            requested_variables: BTreeSet::new(),
            value_kind: ValueKind::Owned,
        })
    }
}
