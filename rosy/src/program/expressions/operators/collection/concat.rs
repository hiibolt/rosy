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
//! ## Example
//!
//! ```text
//! VARIABLE (VE) v;
//! v := 1 & 2 & 3 & 4;     { builds VE from scalars }
//! v := v & 5;              { append to vector }
//!
//! VARIABLE (ST) s;
//! s := 'hello' & ' world'; { string concatenation }
//! ```

use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::ast::{FromRule, Rule};
use crate::program::expressions::Expr;
use crate::transpile::TranspileableExpr;
use crate::transpile::{Transpile, TranspilationInputContext, TranspilationOutput};
use anyhow::{Result, Context, Error, anyhow};
use crate::rosy_lib::RosyType;
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
        //
        // Sneaky way to check that all terms are compatible :3
        let _ = self.type_of(context)
            .map_err(|e| vec!(e.context("...while verifying types of concatenation expression")))?;

        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        let serialization = {
            // Serialize the first term as a base
            let first_term = self.terms.get(0)
                .ok_or(vec!(anyhow!("Concatenation expression must have at least one term!")))?;
            let mut serialization = match first_term.transpile(context) {
                Ok(output) => {
                    requested_variables.extend(output.requested_variables);
                    output.serialization
                },
                Err(mut e) => {
                    for err in e.drain(..) {
                        errors.push(err.context("...while transpiling first term of concatenation"));
                    }
                    String::new() // dummy value to collect more errors
                }
            };

            // Then, for each subsequent term, serialize and concatenate
            for (i, term) in self.terms.iter().skip(1).enumerate() {
                serialization = format!(
                    "&mut RosyConcat::rosy_concat(&*{}, &*{})?",
                    serialization,
                    match term.transpile(context) {
                        Ok(output) => {
                            requested_variables.extend(output.requested_variables);
                            output.serialization
                        },
                        Err(vec_err) => {
                            for err in vec_err {
                                errors.push(err.context(format!(
                                    "...while transpiling term {} of concatenation", i+2
                                )));
                            }
                            String::new() // dummy value to collect more errors
                        }
                    }
                );
            }
            
            serialization
        };
        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables,
                ..Default::default()
            })
        } else {
            Err(errors)
        }
    }
}