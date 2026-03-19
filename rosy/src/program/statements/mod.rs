//! # Statements
//!
//! All executable statements in the ROSY language. Each statement is represented
//! as a [`Statement`] struct wrapping a [`StatementEnum`] variant and a boxed
//! [`Transpile`] implementation.
//!
//! ## Sub-modules
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`core`] | Variable declarations, assignment, control flow, functions, procedures |
//! | [`io`] | File I/O — `WRITE`, `READ`, `OPENF`, `CLOSEF`, binary I/O |
//! | [`da`] | Differential Algebra — `OV` (DA init), `DAPRV`, `DAREV` |
//! | [`math`] | Optimization — `FIT` loop |
//!
//! ## Statement Lifecycle
//!
//! 1. **Parse** — pest grammar matches a rule
//! 2. **AST build** — `Statement::from_rule` dispatches to the correct `FromRule` impl
//! 3. **Transpile** — `Transpile::transpile` generates equivalent Rust code

pub mod core;
pub mod da;
pub mod io;
pub mod math;

pub use core::assign::AssignStatement;
pub use core::var_decl::{VarDeclStatement, VariableDeclarationData};
pub use core::r#loop::LoopStatement;
pub use core::while_loop::WhileStatement;
pub use core::ploop::PLoopStatement;
pub use core::r#if::IfStatement;
pub use core::function_call::FunctionCallStatement;
pub use core::procedure_call::ProcedureCallStatement;
pub use core::function::FunctionStatement;
pub use core::procedure::ProcedureStatement;
pub use core::break_statement::BreakStatement;

pub use da::da_init::DAInitStatement;
pub use da::daprv::DaprvStatement;
pub use da::darev::DarevStatement;

pub use io::write::WriteStatement;
pub use io::read::ReadStatement;
pub use io::writeb::WritebStatement;
pub use io::readb::ReadbStatement;
pub use io::openf::OpenfStatement;
pub use io::openfb::OpenfbStatement;
pub use io::closef::ClosefStatement;

pub use math::fit::FitStatement;
pub use math::ldet::LdetStatement;
pub use math::linv::LinvStatement;
pub use math::polval::PolvalStatement;

pub use core::quit::QuitStatement;
pub use core::scrlen::ScrlenStatement;
pub use core::substr::SubstrStatement;
pub use core::velset::VelsetStatement;

pub use io::cpusec::CpusecStatement;
pub use io::os_call::OsCallStatement;
pub use io::velget::VelgetStatement;

pub use da::daeps::DaepsStatement;
pub use da::danot::DanotStatement;
pub use da::datrn::DatrnStatement;

use crate::{ast::{FromRule, Rule}, transpile::*};
use anyhow::{Context, Error, Result, bail};

/// Source location captured from the pest parse span.
/// Used in error messages for diagnostics.
#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub line: usize,
    pub col: usize,
    /// A short snippet of the source text (first line, truncated).
    pub snippet: String,
}
impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, col {}: {}", self.line, self.col, self.snippet)
    }
}
impl SourceLocation {
    /// Build from a pest pair's span, before the pair is consumed.
    pub fn from_pair(pair: &pest::iterators::Pair<Rule>) -> Self {
        let span = pair.as_span();
        let (line, col) = span.start_pos().line_col();
        let text = span.as_str();
        // Take first line, truncate to 60 chars
        let first_line = text.lines().next().unwrap_or("");
        let snippet = if first_line.len() > 60 {
            format!("{}...", &first_line[..57])
        } else {
            first_line.to_string()
        };
        SourceLocation { line, col, snippet }
    }
}

#[derive(Debug)]
pub struct Statement {
    pub enum_variant: StatementEnum,
    pub inner: Box<dyn TranspileableStatement>,
    pub source_location: SourceLocation,
}
#[derive(Debug)]
pub enum StatementEnum {
    DAInit,
    DaPrv,
    DaRev,
    VarDecl,
    Write,
    Writeb,
    Read,
    Readb,
    Openf,
    Openfb,
    Closef,
    Assign,
    Procedure,
    ProcedureCall,
    Function,
    FunctionCall,
    Loop,
    WhileLoop,
    PLoop,
    If,
    Break,
    Fit,
    Cpusec,
    DaEps,
    DaNot,
    DaTrn,
    Ldet,
    Linv,
    OsCall,
    Polval,
    Quit,
    Scrlen,
    Substr,
    Velget,
    Velset,
}
impl TranspileableStatement for Statement {}
impl FromRule for Statement {
    fn from_rule (
        pair: pest::iterators::Pair<Rule>
    ) -> Result<Option<Statement>> {
        // Capture source location before the pair is consumed
        let loc = SourceLocation::from_pair(&pair);
        match pair.as_rule() {
            Rule::daini => DAInitStatement::from_rule(pair)
                .context("...while building DA initialization statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::DAInit,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::daprv => DaprvStatement::from_rule(pair)
                .context("...while building DAPRV statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::DaPrv,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::darev => DarevStatement::from_rule(pair)
                .context("...while building DAREV statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::DaRev,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::var_decl => VarDeclStatement::from_rule(pair)
                .context("...while building variable declaration!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::VarDecl,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::write => WriteStatement::from_rule(pair)
                .context("...while building write statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Write,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::read => ReadStatement::from_rule(pair)
                .context("...while building read statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Read,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::writeb => WritebStatement::from_rule(pair)
                .context("...while building WRITEB statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Writeb,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::readb => ReadbStatement::from_rule(pair)
                .context("...while building READB statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Readb,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::openf => OpenfStatement::from_rule(pair)
                .context("...while building OPENF statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Openf,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::openfb => OpenfbStatement::from_rule(pair)
                .context("...while building OPENFB statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Openfb,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::closef => ClosefStatement::from_rule(pair)
                .context("...while building CLOSEF statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Closef,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::assignment => AssignStatement::from_rule(pair)
                .context("...while building assignment statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Assign,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::r#loop => LoopStatement::from_rule(pair)
                .context("...while building loop statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Loop,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::while_loop => WhileStatement::from_rule(pair)
                .context("...while building while statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::WhileLoop,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::ploop => PLoopStatement::from_rule(pair)
                .context("...while building ploop statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::PLoop,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::procedure => ProcedureStatement::from_rule(pair)
                .context("...while building procedure declaration!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Procedure,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::procedure_call => ProcedureCallStatement::from_rule(pair)
                .context("...while building procedure call!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::ProcedureCall,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::function => FunctionStatement::from_rule(pair)
                .context("...while building function declaration!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Function,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::function_call => FunctionCallStatement::from_rule(pair)
                .context("...while building function call!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::FunctionCall,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::if_statement => IfStatement::from_rule(pair)
                .context("...while building if statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::If,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::break_statement => BreakStatement::from_rule(pair)
                .context("...while building break statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Break,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::fit_statement => FitStatement::from_rule(pair)
                .context("...while building FIT statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Fit,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),

            Rule::scrlen => ScrlenStatement::from_rule(pair)
                .context("...while building SCRLEN statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Scrlen,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::cpusec => CpusecStatement::from_rule(pair)
                .context("...while building CPUSEC statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Cpusec,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::quit => QuitStatement::from_rule(pair)
                .context("...while building QUIT statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Quit,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::os_call => OsCallStatement::from_rule(pair)
                .context("...while building OS statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::OsCall,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::danot => DanotStatement::from_rule(pair)
                .context("...while building DANOT statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::DaNot,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::daeps => DaepsStatement::from_rule(pair)
                .context("...while building DAEPS statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::DaEps,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::datrn => DatrnStatement::from_rule(pair)
                .context("...while building DATRN statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::DaTrn,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::linv => LinvStatement::from_rule(pair)
                .context("...while building LINV statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Linv,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::ldet => LdetStatement::from_rule(pair)
                .context("...while building LDET statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Ldet,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::substr => SubstrStatement::from_rule(pair)
                .context("...while building SUBSTR statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Substr,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::velset => VelsetStatement::from_rule(pair)
                .context("...while building VELSET statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Velset,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::velget => VelgetStatement::from_rule(pair)
                .context("...while building VELGET statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Velget,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),
            Rule::polval => PolvalStatement::from_rule(pair)
                .context("...while building POLVAL statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Polval,
                    inner: Box::new(stmt),
                    source_location: loc.clone(),
                })),

            // Ignored
            Rule::begin | Rule::end | Rule::EOI | Rule::end_procedure |
            Rule::end_function | Rule::end_loop | Rule::end_while | Rule::endif |
            Rule::end_fit => Ok(None),
            other => bail!("Unexpected statement: {:?}", other),
        }
    }
}
impl Transpile for Statement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        self.inner.transpile(context)
            .map_err(|err_vec| {
                add_context_to_all(err_vec, format!(
                    "...while transpiling {:?} at {}",
                    self.enum_variant, self.source_location
                ))
            })
    }
}