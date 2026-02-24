pub mod assign;
pub mod var_decl;
pub mod write;
pub mod read;
pub mod da_init;
pub mod r#loop;
pub mod while_loop;
pub mod ploop;
pub mod r#if;
pub mod function_call;
pub mod procedure_call;
pub mod function;
pub mod procedure;
pub mod break_statement;
pub mod fit;

pub use assign::AssignStatement;
pub use var_decl::{VarDeclStatement, VariableDeclarationData};
pub use write::WriteStatement;
pub use read::ReadStatement;
pub use da_init::DAInitStatement;
pub use r#loop::LoopStatement;
pub use while_loop::WhileStatement;
pub use ploop::PLoopStatement;
pub use r#if::IfStatement;
pub use function_call::FunctionCallStatement;
pub use procedure_call::ProcedureCallStatement;
pub use function::FunctionStatement;
pub use procedure::ProcedureStatement;
pub use break_statement::BreakStatement;
pub use fit::FitStatement;

use crate::{ast::{FromRule, Rule}, transpile::*};
use anyhow::{Context, Error, Result, bail};

#[derive(Debug)]
pub struct Statement {
    pub enum_variant: StatementEnum,
    pub inner: Box<dyn Transpile>,
}
#[derive(Debug)]
pub enum StatementEnum {
    DAInit,
    VarDecl,
    Write,
    Read,
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
}

impl FromRule for Statement {
    fn from_rule (
        pair: pest::iterators::Pair<Rule>
    ) -> Result<Option<Statement>> {
        match pair.as_rule() {
            Rule::daini => DAInitStatement::from_rule(pair)
                .context("...while building DA initialization statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::DAInit,
                    inner: Box::new(stmt)
                })),
            Rule::var_decl => VarDeclStatement::from_rule(pair)
                .context("...while building variable declaration!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::VarDecl,
                    inner: Box::new(stmt)
                })),
            Rule::write => WriteStatement::from_rule(pair)
                .context("...while building write statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Write,
                    inner: Box::new(stmt)
                })),
            Rule::read => ReadStatement::from_rule(pair)
                .context("...while building read statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Read,
                    inner: Box::new(stmt)
                })),
            Rule::assignment => AssignStatement::from_rule(pair)
                .context("...while building assignment statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Assign,
                    inner: Box::new(stmt)
                })),
            Rule::r#loop => LoopStatement::from_rule(pair)
                .context("...while building loop statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Loop,
                    inner: Box::new(stmt)
                })),
            Rule::while_loop => WhileStatement::from_rule(pair)
                .context("...while building while statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::WhileLoop,
                    inner: Box::new(stmt)
                })),
            Rule::ploop => PLoopStatement::from_rule(pair)
                .context("...while building ploop statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::PLoop,
                    inner: Box::new(stmt)
                })),
            Rule::procedure => ProcedureStatement::from_rule(pair)
                .context("...while building procedure declaration!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Procedure,
                    inner: Box::new(stmt)
                })),
            Rule::procedure_call => ProcedureCallStatement::from_rule(pair)
                .context("...while building procedure call!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::ProcedureCall,
                    inner: Box::new(stmt)
                })),
            Rule::function => FunctionStatement::from_rule(pair)
                .context("...while building function declaration!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Function,
                    inner: Box::new(stmt)
                })),
            Rule::function_call => FunctionCallStatement::from_rule(pair)
                .context("...while building function call!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::FunctionCall,
                    inner: Box::new(stmt)
                })),
            Rule::if_statement => IfStatement::from_rule(pair)
                .context("...while building if statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::If,
                    inner: Box::new(stmt)
                })),
            Rule::break_statement => BreakStatement::from_rule(pair)
                .context("...while building break statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Break,
                    inner: Box::new(stmt)
                })),
            Rule::fit_statement => FitStatement::from_rule(pair)
                .context("...while building FIT statement!")
                .map(|opt| opt.map(|stmt| Statement {
                    enum_variant: StatementEnum::Fit,
                    inner: Box::new(stmt)
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
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        self.inner.transpile(context)
            .map_err(|err_vec| {
                add_context_to_all(err_vec, format!("...while transpiling statement: {:?}", self.enum_variant))
            })
    }
}