mod expr;
mod core;
mod statements;
mod shared;

use crate::ast::*;
use std::{any::Any, collections::{BTreeSet, HashMap}};
use anyhow::{Result, Error};
use crate::rosy_lib::RosyType;

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

pub trait TranspileWithType: Transpile + TypeOf + Send + Sync + std::fmt::Debug + Any + 'static {}
pub trait TypeOf {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType>;
}
impl TypeOf for VariableIdentifier {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        let var_data = context.variables.get(&self.name)
            .ok_or(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.name))?;

        let mut var_type = var_data.data.r#type.clone();
        var_type.dimensions = var_type.dimensions
            .checked_sub(self.indicies.len())
            .ok_or(anyhow::anyhow!(
                "Variable '{}' does not have enough dimensions to index into it (tried to index {} times, but it only has {} dimensions)!",
                self.name, self.indicies.len(), var_type.dimensions
            ))?;

        Ok(var_type)
    }
}
impl TypeOf for Expr {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType> {
        self.inner.type_of(context)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableScope {
    Local,
    Arg,
    Higher
}
#[derive(Debug, Clone)]
pub struct VariableData {
    pub name: String,
    pub r#type: RosyType
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
#[derive(Default, Clone)]
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
            Statement::DAInit(da_init_stmt) => match da_init_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling DA initialization statement"
                    )
                ))
            },
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
                        assign_stmt.identifier.name
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
            },
            Statement::PLoop(ploop_stmt) => match ploop_stmt.transpile(context) {
                Ok(output) => Ok(output),
                Err(vec_err) => Err(add_context_to_all(
                    vec_err,
                    format!(
                        "...while transpiling parallel loop with iterator {}",
                        ploop_stmt.iterator
                    )
                ))
            }
        }
    }
}
impl Transpile for VariableIdentifier {
    fn transpile ( &self, context: &mut TranspilationInputContext ) -> Result<TranspilationOutput, Vec<Error>> {
        // Check that the variable exists and that the 
        //  dimensions are correct
        //
        // Cheeky trick to reuse code :3
        let _ = self.type_of(context)
            .map_err(|err| {
                vec!(err.context(format!("...while checking the type of variable {}", self.name)))
            })?;

        // Serialize the indicies
        let mut serialized_indicies = String::new();
        let mut requested_variables = BTreeSet::new();
        let mut errors = Vec::new();
        for (i, index_expr) in self.indicies.iter().enumerate() {
            let i = i + 1;
            let name = &self.name;

            // Check that the type is RE
            let index_expr_type = index_expr.type_of(context)
                .map_err(|err| {
                    vec!(err.context(format!("...while checking the type for index expression {i} of {name}")))
                })?;
            let expected_type = RosyType::RE();
            if index_expr_type != expected_type {
                return Err(vec!(anyhow::anyhow!("Indexing expression {i} when indexing {name} was {index_expr_type}, when it should be {expected_type}!")));
            }

            // Transpile it
            match index_expr.transpile(context) {
                Ok(output) => {
                    serialized_indicies.push_str(&format!("[(({}).to_owned() - 1.0f64) as usize]", output.serialization));
                    requested_variables.extend(output.requested_variables);
                },
                Err(vec_err) => {
                    for err in vec_err {
                        errors.push(err.context(format!(
                            "...while transpiling index expression to {}", self.name
                        )));
                    }
                }
            }
        }

        // Finally, serialize the entire variable
        if VariableScope::Higher == context.variables.get(&self.name)
            .ok_or(vec!(anyhow::anyhow!("Variable '{}' is not defined in this scope!", self.name)))? 
            .scope
        {
            requested_variables.insert(self.name.clone());
        }
        let serialization = format!(
            "{}{}",
            self.name,
            serialized_indicies
        );
        if errors.is_empty() {
            Ok(TranspilationOutput {
                serialization,
                requested_variables
            })
        } else {
            Err(errors)
        }
    }
}