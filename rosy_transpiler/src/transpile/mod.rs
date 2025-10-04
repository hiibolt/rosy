mod expr;
mod var_decl;
mod procedure;
mod function;
mod assign;
mod variable_data;
mod write;
mod function_call;
mod procedure_call;
mod r#loop;
mod r#if;
mod read;

use crate::ast::*;
use std::collections::{BTreeSet, HashMap};
use anyhow::{Result, Error};
use rosy_lib::RosyType;

fn indent ( st: String ) -> String {
    st.lines()
        .map(|line| format!("\t{}", line))
        .collect::<Vec<String>>()
        .join("\n")
}
fn add_context_to_all ( arr: Vec<Error>, context: String ) -> Vec<Error> {
    arr.into_iter()
        .map(|err| err.context(context.clone()))
        .collect()
}

pub trait TypeOf {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType>;
}
impl TypeOf for Expr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        Ok(match self {
            Expr::Number(_) => RosyType::RE(),
            Expr::String(_) => RosyType::ST(),
            Expr::Boolean(_) => RosyType::LO(),
            Expr::Var(name) => {
                let var_data = context.variables.get(name)
                    .ok_or(anyhow::anyhow!("Variable '{}' is not defined in this scope!", name))?;

                var_data.data.r#type.clone()
            },
            Expr::Add { left, right } => {
                rosy_lib::operators::add::get_return_type(
                    &left.type_of(context)?,
                    &right.type_of(context)?
                ).ok_or(anyhow::anyhow!(
                    "Cannot add types '{}' and '{}' together!",
                    left.type_of(context)?,
                    right.type_of(context)?
                ))?
            },
            Expr::StringConvert { expr } => {
                let expr_type = expr.type_of(context)?;
                rosy_lib::intrinsics::st::get_return_type(&expr_type)
                    .ok_or(anyhow::anyhow!("Cannot convert type '{expr_type}' to 'ST'!"))?
            },
            Expr::FunctionCall { name, .. } => context.functions.get(name)
                .ok_or(anyhow::anyhow!("Function '{}' is not defined in this scope, can't call it from expression!", name))?
                .return_type
                .clone(),
            _ => todo!()
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableScope {
    Local,
    Arg,
    Higher
}
#[derive(Debug, Clone)]
pub struct ScopedVariableData {
    pub scope: VariableScope,
    pub data: VariableData
}
#[derive(Debug, Clone)]
pub struct TranspilationInputFunctionContext {
    pub return_type: RosyType,
    pub args: Vec<VariableData>,
    pub requested_variables: BTreeSet<String>
}
#[derive(Debug, Clone)]
pub struct TranspilationInputProcedureContext {
    pub args: Vec<VariableData>,
    pub requested_variables: BTreeSet<String>
}
#[derive(Clone, Default)]
pub struct TranspilationInputContext {
    pub variables:  HashMap<String, ScopedVariableData>,
    pub functions:  HashMap<String, TranspilationInputFunctionContext>,
    pub procedures: HashMap<String, TranspilationInputProcedureContext>
}
#[derive(Default)]
pub struct TranspilationOutput {
    pub serialization: String,
    requested_variables: BTreeSet<String>
}
pub trait Transpile {
    fn transpile ( 
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>>;
}


impl Transpile for Program {
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        let mut serialization = Vec::new();
        let mut errors = Vec::new();
        for statement in &self.statements {
            match statement.transpile(context) {
                Ok(output) => {
                    serialization.push(output.serialization);
                },
                Err(stmt_errors) => {
                    for e in stmt_errors {
                        errors.push(e.context("...while transpiling a top-level statement"));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization: serialization.join("\n"),
                requested_variables: BTreeSet::new(),
            })
        } else {
            Err(errors)
        }
    }
}
impl Transpile for Statement {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Handle analyzing the specific statement
        match &self {
            Statement::VarDecl(var_decl_stmt) => match var_decl_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling variable declaration for variable {}",
                        var_decl_stmt.data.name
                    )
                ))
            },
            Statement::Procedure(procedure_stmt) => match procedure_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling procedure {}",
                        procedure_stmt.name
                    )
                ))
            },
            Statement::Assign(assign_stmt) => match assign_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling assignment to variable {}",
                        assign_stmt.name
                    )
                ))
            },
            Statement::Function(function_stmt) => match function_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling function {}",
                        function_stmt.name
                    )
                ))
            },
            Statement::Write(write_stmt) => match write_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling WRITE statement to unit {}",
                        write_stmt.unit
                    )
                ))
            },
            Statement::FunctionCall(function_call_stmt) => match function_call_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling function call to function {}",
                        function_call_stmt.name
                    )
                ))
            },
            Statement::ProcedureCall(procedure_call_stmt) => match procedure_call_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling procedure call to procedure {}",
                        procedure_call_stmt.name
                    )
                ))
            },
            Statement::Loop(loop_stmt) => match loop_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling loop with iterator {}",
                        loop_stmt.iterator
                    )
                ))
            },
            Statement::If(if_stmt) => match if_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling IF statement"
                    )
                ))
            },
            Statement::Read(read_stmt) => match read_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling READ statement from unit {}",
                        read_stmt.unit
                    )
                ))
            }
        }
    }
}