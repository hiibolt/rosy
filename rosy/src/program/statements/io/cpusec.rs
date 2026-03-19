//! # CPUSEC Statement
//!
//! Returns the elapsed CPU time in the process and assigns it to a variable.
//!
//! ## Syntax
//!
//! ```text
//! CPUSEC v;
//! ```
//!
//! ## Example
//!
//! ```text
//! VARIABLE (RE) t;
//! CPUSEC t;
//! WRITE 6 t;
//! ```

use std::collections::BTreeSet;
use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*, program::expressions::core::variable_identifier::VariableIdentifier,
    transpile::{TranspilationInputContext, TranspilationOutput, Transpile, TranspileableStatement, VariableScope}
};

/// AST node for `CPUSEC v;`.
#[derive(Debug)]
pub struct CpusecStatement {
    pub identifier: VariableIdentifier,
}

impl FromRule for CpusecStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::cpusec,
            "Expected `cpusec` rule when building CPUSEC statement, found: {:?}", pair.as_rule());

        let mut inner = pair.into_inner();

        let expr_pair = inner.next()
            .context("Missing variable expression in CPUSEC!")?;

        // The argument must be a variable identifier (assignable l-value).
        // We parse it as an Expr first to get the pair, then extract the
        // variable_identifier from the inner of the expr.
        let identifier = VariableIdentifier::from_rule(expr_pair)
            .context("Failed to build variable identifier in CPUSEC")?
            .ok_or_else(|| anyhow::anyhow!("Expected variable identifier in CPUSEC"))?;

        Ok(Some(CpusecStatement { identifier }))
    }
}

impl TranspileableStatement for CpusecStatement {}

impl Transpile for CpusecStatement {
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();

        // Serialize the target variable identifier (l-value)
        let serialized_identifier = match self.identifier.transpile(context) {
            Ok(output) => {
                requested_variables.extend(output.requested_variables.clone());
                output.serialization
            },
            Err(vec_err) => {
                for err in vec_err {
                    errors.push(err.context(format!(
                        "...while transpiling identifier expression for CPUSEC into '{}'",
                        self.identifier.name
                    )));
                }
                String::new()
            }
        };

        if !errors.is_empty() {
            return Err(errors);
        }

        // Determine deref prefix based on variable scope
        let dereference = match context.variables.get(&self.identifier.name)
            .ok_or_else(|| vec![anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.identifier.name)])?
            .scope
        {
            VariableScope::Local => "",
            VariableScope::Arg => "*",
            VariableScope::Higher => {
                requested_variables.insert(self.identifier.name.clone());
                "*"
            }
        };

        let serialization = format!(
            "{}{} = {{\n\
                static __CPUSEC_START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();\n\
                let __start = __CPUSEC_START.get_or_init(std::time::Instant::now);\n\
                __start.elapsed().as_secs_f64()\n\
            }};",
            dereference, serialized_identifier
        );

        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
