//! # POLVAL Statement
//!
//! Evaluates / composes polynomials stored as DA vectors.
//!
//! ## Syntax
//!
//! ```text
//! POLVAL L P NP A NA R NR;
//! ```
//!
//! | Arg | Role                                    |
//! |-----|-----------------------------------------|
//! | L   | evaluation mode (1 = Horner, normally 1)|
//! | P   | array of NP DA polynomial vectors       |
//! | NP  | number of polynomials                   |
//! | A   | array of NA arguments (RE or DA)        |
//! | NA  | number of arguments                     |
//! | R   | result array (output variable)          |
//! | NR  | number of results                       |

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*, program::expressions::Expr,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement, add_context_to_all}
};

/// AST node for `POLVAL L P NP A NA R NR;`.
#[derive(Debug)]
pub struct PolvalStatement {
    pub l_expr:  Expr,
    pub p_expr:  Expr,
    pub np_expr: Expr,
    pub a_expr:  Expr,
    pub na_expr: Expr,
    pub r_expr:  Expr,
    pub nr_expr: Expr,
}

impl FromRule for PolvalStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::polval,
            "Expected `polval` rule when building POLVAL statement, found: {:?}", pair.as_rule());

        let mut inner = pair.into_inner();

        macro_rules! next_expr {
            ($name:literal) => {{
                let p = inner.next()
                    .context(concat!("Missing `", $name, "` parameter in POLVAL!"))?;
                Expr::from_rule(p)
                    .context(concat!("Failed to build `", $name, "` expression in POLVAL"))?
                    .ok_or_else(|| anyhow::anyhow!(concat!("Expected `", $name, "` expression in POLVAL")))?
            }};
        }

        let l_expr  = next_expr!("L");
        let p_expr  = next_expr!("P");
        let np_expr = next_expr!("NP");
        let a_expr  = next_expr!("A");
        let na_expr = next_expr!("NA");
        let r_expr  = next_expr!("R");
        let nr_expr = next_expr!("NR");

        Ok(Some(PolvalStatement { l_expr, p_expr, np_expr, a_expr, na_expr, r_expr, nr_expr }))
    }
}

impl TranspileableStatement for PolvalStatement {}

impl Transpile for PolvalStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();

        let l_out  = self.l_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling L in POLVAL".to_string()))?;
        requested_variables.extend(l_out.requested_variables.iter().cloned());

        let p_out  = self.p_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling P in POLVAL".to_string()))?;
        requested_variables.extend(p_out.requested_variables.iter().cloned());

        let np_out = self.np_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling NP in POLVAL".to_string()))?;
        requested_variables.extend(np_out.requested_variables.iter().cloned());

        let a_out  = self.a_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling A in POLVAL".to_string()))?;
        requested_variables.extend(a_out.requested_variables.iter().cloned());

        let na_out = self.na_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling NA in POLVAL".to_string()))?;
        requested_variables.extend(na_out.requested_variables.iter().cloned());

        // R is the output variable — needs a mutable borrow
        let r_out  = self.r_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling R in POLVAL".to_string()))?;
        requested_variables.extend(r_out.requested_variables.clone());

        let nr_out = self.nr_expr.transpile(context)
            .map_err(|e| add_context_to_all(e, "...while transpiling NR in POLVAL".to_string()))?;
        requested_variables.extend(nr_out.requested_variables.iter().cloned());

        // Build a mutable reference to the result variable.
        // as_ref() gives either `&expr` (Owned) or the Ref serialization which
        // already starts with `&`. Replace the leading `&` with `&mut `.
        let r_mut = r_out.as_ref().replacen('&', "&mut ", 1);

        let serialization = format!(
            "rosy_lib::core::polval::rosy_polval_re({}, {}, {} as usize, {}, {} as usize, {}, {} as usize)?;",
            l_out.as_value(),
            p_out.as_ref(),
            np_out.as_value(),
            a_out.as_ref(),
            na_out.as_value(),
            r_mut,
            nr_out.as_value(),
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
            ..Default::default()
        })
    }
}
