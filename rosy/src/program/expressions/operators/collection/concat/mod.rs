//! # Concatenation Operator (`&`)
//!
//! Concatenates scalars and vectors into larger vectors, or strings together.
//! Multiple `&` operators in a row are flattened into a single `ConcatExpr`
//! with multiple terms for efficient code generation.
//!
//! ## Syntax
//!
//! ```text
//! expr & expr & ...
//! ```
//!
//! ## Type Compatibility
//!
//! | Left | Right | Result | Comment |
//! |------|-------|--------|---------|
//! | RE | RE | VE | Concatenate two Reals to a Vector |
//! | RE | VE | VE | Prepend a Real to the left of a Vector |
//! | ST | ST | ST | Concatenate two Strings |
//! | VE | RE | VE | Append a Real to the right of a Vector |
//! | VE | VE | VE | Concatenate two Vectors |
//!
//! ## Rosy Example
#![doc = include_str!("test.rosy")]
//! **Output**:
#![doc = include_str!("rosy_output.txt")]
//! ## COSY Example
#![doc = include_str!("test.fox")]
//! **Output**:
#![doc = include_str!("cosy_output.txt")]

use std::collections::BTreeSet;
use std::collections::HashSet;
use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileableExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput, ValueKind};
use anyhow::{Result, Context, Error, anyhow};
use crate::rosy_lib::{RosyType, RosyBaseType};
use crate::resolve::{TypeResolver, ScopeContext, TypeSlot, ExprRecipe};

/// AST node for the concatenation operator (`&`).
///
/// Unlike other binary operators, concatenation flattens chains:
/// `a & b & c` becomes `ConcatExpr { terms: [a, b, c] }` rather than nested nodes.
#[derive(Debug, PartialEq)]
pub struct ConcatExpr {
    pub terms: Vec<Expr>
}

impl FromRule for ConcatExpr {
    fn from_rule(_pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        // ConcatExpr is created by the infix parser, not directly from a rule
        anyhow::bail!("ConcatExpr should be created by infix parser, not FromRule")
    }
}
impl TranspileableExpr for ConcatExpr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let mut r#type = self.terms.last()
            .ok_or(anyhow::anyhow!("Cannot concatenate zero terms!"))?
            .type_of(context)
            .context("...while determining type of last term in concatenation")?;

        for term_expr in self.terms.iter().rev().skip(1) {
            let term_type = term_expr.type_of(context)
                .context("...while determining type of term in concatenation")?;

            r#type = crate::rosy_lib::operators::concat::get_return_type(&r#type, &term_type)
                .ok_or(anyhow::anyhow!(
                    "Cannot concatenate types '{}' and '{}' together!",
                    r#type, term_type
                ))?;
        }

        Ok(r#type)
    }
    fn discover_expr_function_calls(&self, resolver: &mut TypeResolver, ctx: &ScopeContext) -> Option<Result<()>> {
        for term in &self.terms {
            if let Err(e) = resolver.discover_expr_function_calls(term, ctx) {
                return Some(Err(e));
            }
        }
        Some(Ok(()))
    }
    fn build_expr_recipe(&self, resolver: &TypeResolver, ctx: &ScopeContext, deps: &mut HashSet<TypeSlot>) -> Option<ExprRecipe> {
        let recipes: Vec<ExprRecipe> = self.terms.iter()
            .map(|t| resolver.build_expr_recipe(t, ctx, deps))
            .collect();
        Some(ExprRecipe::Concat(recipes))
    }
    fn extend_concat(&mut self, right: Expr) -> bool {
        self.terms.push(right);
        true
    }
}
impl Transpile for ConcatExpr {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // First, do a type check
        let _ = self.type_of(context)
            .map_err(|e| vec!(e.context("...while verifying types of concatenation expression")))?;

        // Check if all terms are the same scalar type for direct emission
        let term_types: Vec<_> = self.terms.iter()
            .map(|t| t.type_of(context))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|e| vec![e])?;

        let all_re_scalar = term_types.iter().all(|t| t.base_type == RosyBaseType::RE && t.dimensions == 0);
        let all_st_scalar = term_types.iter().all(|t| t.base_type == RosyBaseType::ST && t.dimensions == 0);

        // Direct vec![...] emission for all-RE terms (avoids N-1 intermediate allocations)
        if all_re_scalar {
            let mut parts = Vec::new();
            let mut requested_variables = BTreeSet::new();
            let mut errors = Vec::new();
            for (i, term) in self.terms.iter().enumerate() {
                match term.transpile(context) {
                    Ok(output) => {
                        requested_variables.extend(output.requested_variables.iter().cloned());
                        parts.push(output.as_value());
                    },
                    Err(mut e) => {
                        for err in e.drain(..) {
                            errors.push(err.context(format!("...while transpiling term {} of concatenation", i+1)));
                        }
                    }
                }
            }
            return if errors.is_empty() {
                Ok(TranspilationOutput {
                    serialization: format!("vec![{}]", parts.join(", ")),
                    requested_variables,
                    value_kind: ValueKind::Owned,
                })
            } else {
                Err(errors)
            };
        }

        // Direct format!(...) emission for all-ST terms (single allocation)
        if all_st_scalar {
            let mut parts = Vec::new();
            let mut requested_variables = BTreeSet::new();
            let mut errors = Vec::new();
            for (i, term) in self.terms.iter().enumerate() {
                match term.transpile(context) {
                    Ok(output) => {
                        requested_variables.extend(output.requested_variables.iter().cloned());
                        parts.push(output.as_ref());
                    },
                    Err(mut e) => {
                        for err in e.drain(..) {
                            errors.push(err.context(format!("...while transpiling term {} of concatenation", i+1)));
                        }
                    }
                }
            }
            return if errors.is_empty() {
                Ok(TranspilationOutput {
                    serialization: format!("format!(\"{}\"{})",
                        "{}".repeat(parts.len()),
                        parts.iter().map(|p| format!(", {p}")).collect::<String>()),
                    requested_variables,
                    value_kind: ValueKind::Owned,
                })
            } else {
                Err(errors)
            };
        }

        // General case: accumulator-based chaining via RosyConcat trait
        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();

        let first_term = self.terms.get(0)
            .ok_or(vec!(anyhow!("Concatenation expression must have at least one term!")))?;
        let mut accumulator = match first_term.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables.iter().cloned());
                output
            },
            Err(mut e) => {
                for err in e.drain(..) {
                    errors.push(err.context("...while transpiling first term of concatenation"));
                }
                TranspilationOutput::default()
            }
        };

        for (i, term) in self.terms.iter().skip(1).enumerate() {
            let term_output = match term.transpile(context) {
                Ok(output) => {
                    requested_variables.extend(output.requested_variables.iter().cloned());
                    output
                },
                Err(mut vec_err) => {
                    for err in vec_err.drain(..) {
                        errors.push(err.context(format!(
                            "...while transpiling term {} of concatenation", i+2
                        )));
                    }
                    TranspilationOutput::default()
                }
            };
            accumulator = TranspilationOutput {
                serialization: format!("RosyConcat::rosy_concat({}, {})?", accumulator.as_ref(), term_output.as_ref()),
                requested_variables: BTreeSet::new(),
                value_kind: ValueKind::Owned,
            };
        }

        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization: accumulator.serialization,
                requested_variables,
                value_kind: ValueKind::Owned,
            })
        } else {
            Err(errors)
        }
    }
}