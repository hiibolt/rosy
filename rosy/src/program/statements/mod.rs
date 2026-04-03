//! # Statements
//!
//! Everything in ROSY that *does* something — declarations, control flow, I/O,
//! and more.
//!
//! ## Looking for something?
//!
//! | I want to... | Go to |
//! |--------------|-------|
//! | Declare a variable (`VARIABLE`) | **[`core::var_decl`]** |
//! | Assign a value (`:=`) | **[`core::assign`]** |
//! | Use `IF` / `ELSEIF` / `ELSE` | **[`core::if`]** |
//! | Use `LOOP` or `WHILE` | **[`core::loop`]**, **[`core::while_loop`]** |
//! | Use `PLOOP` (MPI parallel loop) | **[`core::ploop`]** |
//! | Define a `FUNCTION` or `PROCEDURE` | **[`core::function`]**, **[`core::procedure`]** |
//! | Call a function or procedure | **[`core::function_call`]**, **[`core::procedure_call`]** |
//! | Print output (`WRITE`) | **[`io::write`]** |
//! | Read input (`READ`) | **[`io::read`]** |
//! | Open/close files | **[`io::openf`]**, **[`io::closef`]** |
//! | Use binary I/O | **[`io::writeb`]**, **[`io::readb`]** |
//! | Initialize DA (`OV`) | **[`da::da_init`]** |
//! | Print DA values | **[`da::daprv`]**, **[`da::darev`]** |
//! | Print DA by variable/order | **[`da::dapew`]** |
//! | Read DA from file | **[`da::darea`]** |
//! | Configure DA | **[`da::daeps`]**, **[`da::danot`]**, **[`da::datrn`]** |
//! | Scale / negate DA | **[`da::dascl`]**, **[`da::dasgn`]** |
//! | Differentiate / integrate DA | **[`da::dader`]**, **[`da::daint`]** |
//! | Filter DA terms | **[`da::danoro`]**, **[`da::danors`]** |
//! | Substitute variable in DA | **[`da::daplu`]** |
//! | DA division / shift | **[`da::dadiu`]**, **[`da::dadmu`]** |
//! | Extract DA coefficients | **[`da::dacliw`]**, **[`da::dacqlc`]**, **[`da::dapee`]**, **[`da::dapea`]**, **[`da::dapep`]** |
//! | Estimate DA term size | **[`da::daest`]** |
//! | DA tree evaluation | **[`da::mtree`]** |
//! | Use `FIT` (optimization) | **[`math::fit`]** |
//! | Use `BREAK` or `QUIT` | **[`core::break`]**, **[`core::quit`]** |
//! | Measure time | **[`io::cpusec`]**, **[`io::pwtime`]** |
//! | Run a shell command | **[`io::os_call`]** |
//! | Read vectors from files | **[`io::velget`]** |
//! | Extract a substring | **[`core::substr`]** |
//! | Parse string to number | **[`core::stcre`]** |
//! | Format number as string | **[`core::recst`]** |
//! | Set a vector component | **[`core::velset`]** |
//! | Get a random number | **[`core::reran`]** |
//! | Set RNG seed | **[`core::ranseed`]** |
//! | Get imaginary unit | **[`core::imunit`]** |
//! | Get process count | **[`core::pnpro`]** |
//! | Matrix operations | **[`math::linv`]**, **[`math::ldet`]**, **[`math::lev`]**, **[`math::mblock`]** |
//! | Polynomial evaluation | **[`math::polval`]** |
//! | Vector math | **[`math::vedot`]**, **[`math::veunit`]**, **[`math::vezero`]** |

pub mod core;
pub mod da;
pub mod io;
pub mod math;

pub use core::assign::AssignStatement;
pub use core::r#break::BreakStatement;
pub use core::function::FunctionStatement;
pub use core::function_call::FunctionCallStatement;
pub use core::r#if::IfStatement;
pub use core::r#loop::LoopStatement;
pub use core::ploop::PLoopStatement;
pub use core::procedure::ProcedureStatement;
pub use core::procedure_call::ProcedureCallStatement;
pub use core::var_decl::{VarDeclStatement, VariableDeclarationData};
pub use core::while_loop::WhileStatement;

pub use da::da_init::DAInitStatement;
pub use da::daprv::DaprvStatement;
pub use da::darev::DarevStatement;

pub use io::closef::ClosefStatement;
pub use io::openf::OpenfStatement;
pub use io::openfb::OpenfbStatement;
pub use io::read::ReadStatement;
pub use io::readb::ReadbStatement;
pub use io::readm::ReadmStatement;
pub use io::write::WriteStatement;
pub use io::writeb::WritebStatement;
pub use io::writem::WritemStatement;
pub use io::rewf::RewfStatement;
pub use io::backf::BackfStatement;
pub use io::reads::ReadsStatement;

pub use math::fit::FitStatement;
pub use math::intpol::IntpolStatement;
pub use math::ldet::LdetStatement;
pub use math::lev::LevStatement;
pub use math::linv::LinvStatement;
pub use math::lsline::LslineStatement;
pub use math::mblock::MblockStatement;
pub use math::polval::PolvalStatement;
pub use math::rkco::RkcoStatement;

pub use core::quit::QuitStatement;
pub use core::scrlen::ScrlenStatement;
pub use core::substr::SubstrStatement;
pub use core::velset::VelsetStatement;

pub use io::cpusec::CpusecStatement;
pub use io::os_call::OsCallStatement;
pub use io::pwtime::PwtimeStatement;
pub use io::velget::VelgetStatement;

pub use math::vedot::VedotStatement;
pub use math::veunit::VeunitStatement;
pub use math::vezero::VezeroStatement;

pub use core::imunit::ImunitStatement;
pub use core::pnpro::PnproStatement;
pub use core::recst::RecstStatement;
pub use core::ranseed::RanseedStatement;
pub use core::reran::ReranStatement;
pub use core::sleepm::SleepmStatement;
pub use core::argget::ArggetStatement;
pub use core::memdpv::MemdpvStatement;
pub use core::memfre::MemfreStatement;
pub use core::memall::MemallStatement;
pub use core::memwrt::MemwrtStatement;
pub use core::ltrue::LtrueStatement;
pub use core::lfalse::LfalseStatement;
pub use core::stcre::StcreStatement;

pub use da::dacliw::DacliwStatement;
pub use da::dacqlc::DacqlcStatement;
pub use da::dader::DaderStatement;
pub use da::dadiu::DadiuStatement;
pub use da::dadmu::DadmuStatement;
pub use da::daeps::DaepsStatement;
pub use da::daest::DaestStatement;
pub use da::daint::DaintStatement;
pub use da::danoro::DanoroStatement;
pub use da::danors::DanorsStatement;
pub use da::danot::DanotStatement;
pub use da::dapea::DapeaStatement;
pub use da::dapee::DapeeStatement;
pub use da::dapep::DapepStatement;
pub use da::dapew::DapewStatement;
pub use da::daplu::DapluStatement;
pub use da::darea::DareaStatement;
pub use da::dascl::DasclStatement;
pub use da::dasgn::DasgnStatement;
pub use da::datrn::DatrnStatement;
pub use da::mtree::MtreeStatement;

use crate::{
    ast::{FromRule, Rule},
    resolve::*,
    transpile::*,
};
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
    Vedot,
    Veunit,
    Vezero,
    Stcre,
    Recst,
    Ranseed,
    Reran,
    Pwtime,
    Pnpro,
    Imunit,
    Lev,
    Lsline,
    Mblock,
    Mtree,
    Rkco,
    DaScl,
    DaSgn,
    DaDer,
    DaInt,
    DaNoro,
    DaNors,
    DaPlu,
    DaDiu,
    DaDmu,
    DaCliw,
    DaCqlc,
    DaArea,
    DaPew,
    DaPee,
    DaPea,
    DaPep,
    DaEst,
    Sleepm,
    Argget,
    Memdpv,
    Memfre,
    Memall,
    Memwrt,
    Ltrue,
    Lfalse,
    Readm,
    Writem,
    Rewf,
    Backf,
    Reads,
    Intpol,
}
impl TranspileableStatement for Statement {
    fn register_typeslot_declaration(
        &self,
        _resolver: &mut TypeResolver,
        _ctx: &mut ScopeContext,
        _source_location: SourceLocation,
    ) -> TypeslotDeclarationResult {
        TypeslotDeclarationResult::NotAVarFuncOrProcedureDecl
    }
    fn wire_inference_edges(
        &self,
        _resolver: &mut TypeResolver,
        _ctx: &mut ScopeContext,
        _source_location: SourceLocation,
    ) -> InferenceEdgeResult {
        InferenceEdgeResult::NoEdges
    }
    fn hydrate_resolved_types(
        &mut self,
        _resolver: &TypeResolver,
        _current_scope: &[String],
    ) -> TypeHydrationResult {
        TypeHydrationResult::NothingToHydrate
    }
}
impl FromRule for Statement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>> {
        // Capture source location before the pair is consumed
        let loc = SourceLocation::from_pair(&pair);
        match pair.as_rule() {
            Rule::daini => DAInitStatement::from_rule(pair)
                .context("...while building DA initialization statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DAInit,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::daprv => DaprvStatement::from_rule(pair)
                .context("...while building DAPRV statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaPrv,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::darev => DarevStatement::from_rule(pair)
                .context("...while building DAREV statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaRev,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::var_decl => VarDeclStatement::from_rule(pair)
                .context("...while building variable declaration!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::VarDecl,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::write => WriteStatement::from_rule(pair)
                .context("...while building write statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Write,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::reads => ReadsStatement::from_rule(pair)
                .context("...while building READS statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Reads,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::read => ReadStatement::from_rule(pair)
                .context("...while building read statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Read,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::writeb => WritebStatement::from_rule(pair)
                .context("...while building WRITEB statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Writeb,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::readb => ReadbStatement::from_rule(pair)
                .context("...while building READB statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Readb,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::openf => OpenfStatement::from_rule(pair)
                .context("...while building OPENF statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Openf,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::openfb => OpenfbStatement::from_rule(pair)
                .context("...while building OPENFB statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Openfb,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::closef => ClosefStatement::from_rule(pair)
                .context("...while building CLOSEF statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Closef,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::rewf => RewfStatement::from_rule(pair)
                .context("...while building REWF statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Rewf,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::backf => BackfStatement::from_rule(pair)
                .context("...while building BACKF statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Backf,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::assignment => AssignStatement::from_rule(pair)
                .context("...while building assignment statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Assign,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::r#loop => LoopStatement::from_rule(pair)
                .context("...while building loop statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Loop,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::while_loop => WhileStatement::from_rule(pair)
                .context("...while building while statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::WhileLoop,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::ploop => PLoopStatement::from_rule(pair)
                .context("...while building ploop statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::PLoop,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::procedure => ProcedureStatement::from_rule(pair)
                .context("...while building procedure declaration!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Procedure,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::procedure_call => ProcedureCallStatement::from_rule(pair)
                .context("...while building procedure call!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::ProcedureCall,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::function => FunctionStatement::from_rule(pair)
                .context("...while building function declaration!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Function,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::function_call => FunctionCallStatement::from_rule(pair)
                .context("...while building function call!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::FunctionCall,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::if_statement => IfStatement::from_rule(pair)
                .context("...while building if statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::If,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::break_statement => BreakStatement::from_rule(pair)
                .context("...while building break statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Break,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::fit_statement => FitStatement::from_rule(pair)
                .context("...while building FIT statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Fit,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),

            Rule::scrlen => ScrlenStatement::from_rule(pair)
                .context("...while building SCRLEN statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Scrlen,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::cpusec => CpusecStatement::from_rule(pair)
                .context("...while building CPUSEC statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Cpusec,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::quit => QuitStatement::from_rule(pair)
                .context("...while building QUIT statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Quit,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::os_call => OsCallStatement::from_rule(pair)
                .context("...while building OS statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::OsCall,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::danot => DanotStatement::from_rule(pair)
                .context("...while building DANOT statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaNot,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::daeps => DaepsStatement::from_rule(pair)
                .context("...while building DAEPS statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaEps,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::datrn => DatrnStatement::from_rule(pair)
                .context("...while building DATRN statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaTrn,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::linv => LinvStatement::from_rule(pair)
                .context("...while building LINV statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Linv,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::ldet => LdetStatement::from_rule(pair)
                .context("...while building LDET statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Ldet,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::substr => SubstrStatement::from_rule(pair)
                .context("...while building SUBSTR statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Substr,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::velset => VelsetStatement::from_rule(pair)
                .context("...while building VELSET statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Velset,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::velget => VelgetStatement::from_rule(pair)
                .context("...while building VELGET statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Velget,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::intpol => IntpolStatement::from_rule(pair)
                .context("...while building INTPOL statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Intpol,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::polval => PolvalStatement::from_rule(pair)
                .context("...while building POLVAL statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Polval,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::vedot => VedotStatement::from_rule(pair)
                .context("...while building VEDOT statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Vedot,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::veunit => VeunitStatement::from_rule(pair)
                .context("...while building VEUNIT statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Veunit,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::vezero => VezeroStatement::from_rule(pair)
                .context("...while building VEZERO statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Vezero,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::stcre => StcreStatement::from_rule(pair)
                .context("...while building STCRE statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Stcre,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::recst => RecstStatement::from_rule(pair)
                .context("...while building RECST statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Recst,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::reran => ReranStatement::from_rule(pair)
                .context("...while building RERAN statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Reran,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::ranseed => RanseedStatement::from_rule(pair)
                .context("...while building RANSEED statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Ranseed,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::pwtime => PwtimeStatement::from_rule(pair)
                .context("...while building PWTIME statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Pwtime,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::pnpro => PnproStatement::from_rule(pair)
                .context("...while building PNPRO statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Pnpro,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::imunit => ImunitStatement::from_rule(pair)
                .context("...while building IMUNIT statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Imunit,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::lev => LevStatement::from_rule(pair)
                .context("...while building LEV statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Lev,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::mblock => MblockStatement::from_rule(pair)
                .context("...while building MBLOCK statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Mblock,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::mtree => MtreeStatement::from_rule(pair)
                .context("...while building MTREE statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Mtree,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::lsline => LslineStatement::from_rule(pair)
                .context("...while building LSLINE statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Lsline,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::rkco => RkcoStatement::from_rule(pair)
                .context("...while building RKCO statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Rkco,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dascl => DasclStatement::from_rule(pair)
                .context("...while building DASCL statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaScl,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dasgn => DasgnStatement::from_rule(pair)
                .context("...while building DASGN statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaSgn,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dader => DaderStatement::from_rule(pair)
                .context("...while building DADER statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaDer,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::daint => DaintStatement::from_rule(pair)
                .context("...while building DAINT statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaInt,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::danoro => DanoroStatement::from_rule(pair)
                .context("...while building DANORO statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaNoro,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::danors => DanorsStatement::from_rule(pair)
                .context("...while building DANORS statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaNors,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::daplu => DapluStatement::from_rule(pair)
                .context("...while building DAPLU statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaPlu,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dadiu => DadiuStatement::from_rule(pair)
                .context("...while building DADIU statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaDiu,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dadmu => DadmuStatement::from_rule(pair)
                .context("...while building DADMU statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaDmu,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dacliw => DacliwStatement::from_rule(pair)
                .context("...while building DACLIW statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaCliw,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dacqlc => DacqlcStatement::from_rule(pair)
                .context("...while building DACQLC statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaCqlc,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::darea => DareaStatement::from_rule(pair)
                .context("...while building DAREA statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaArea,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dapew => DapewStatement::from_rule(pair)
                .context("...while building DAPEW statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaPew,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dapee => DapeeStatement::from_rule(pair)
                .context("...while building DAPEE statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaPee,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dapea => DapeaStatement::from_rule(pair)
                .context("...while building DAPEA statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaPea,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::dapep => DapepStatement::from_rule(pair)
                .context("...while building DAPEP statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaPep,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::daest => DaestStatement::from_rule(pair)
                .context("...while building DAEST statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::DaEst,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),

            Rule::sleepm => SleepmStatement::from_rule(pair)
                .context("...while building SLEEPM statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Sleepm,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::argget => ArggetStatement::from_rule(pair)
                .context("...while building ARGGET statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Argget,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::memdpv => MemdpvStatement::from_rule(pair)
                .context("...while building MEMDPV statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Memdpv,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::memfre => MemfreStatement::from_rule(pair)
                .context("...while building MEMFRE statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Memfre,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::memall => MemallStatement::from_rule(pair)
                .context("...while building MEMALL statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Memall,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::memwrt => MemwrtStatement::from_rule(pair)
                .context("...while building MEMWRT statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Memwrt,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::ltrue => LtrueStatement::from_rule(pair)
                .context("...while building LTRUE statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Ltrue,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::lfalse => LfalseStatement::from_rule(pair)
                .context("...while building LFALSE statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Lfalse,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),

            Rule::readm => ReadmStatement::from_rule(pair)
                .context("...while building READM statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Readm,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),
            Rule::writem => WritemStatement::from_rule(pair)
                .context("...while building WRITEM statement!")
                .map(|opt| {
                    opt.map(|stmt| Statement {
                        enum_variant: StatementEnum::Writem,
                        inner: Box::new(stmt),
                        source_location: loc.clone(),
                    })
                }),

            // Ignored
            Rule::begin
            | Rule::end
            | Rule::EOI
            | Rule::end_procedure
            | Rule::end_function
            | Rule::end_loop
            | Rule::end_while
            | Rule::endif
            | Rule::end_fit => Ok(None),
            other => bail!("Unexpected statement: {:?}", other),
        }
    }
}
impl Transpile for Statement {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>> {
        self.inner.transpile(context).map_err(|err_vec| {
            add_context_to_all(
                err_vec,
                format!(
                    "...while transpiling {:?} at {}",
                    self.enum_variant, self.source_location
                ),
            )
        })
    }
}
