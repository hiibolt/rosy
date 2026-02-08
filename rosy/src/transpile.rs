use std::{any::Any, collections::{BTreeSet, HashMap}};
use anyhow::{Result, Error};
use crate::rosy_lib::RosyType;

pub trait TranspileWithType: Transpile + TypeOf + Send + Sync + std::fmt::Debug + Any + 'static {}
pub trait TypeOf {
    fn type_of ( &self, context: &TranspilationInputContext ) -> Result<RosyType>;
}
pub trait Transpile: std::fmt::Debug {
    fn transpile ( 
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>>;
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
    pub procedures: HashMap<String, TranspilationInputProcedureContext>,
    pub in_loop: bool,
}
#[derive(Default)]
pub struct TranspilationOutput {
    pub serialization: String,
    pub requested_variables: BTreeSet<String>
}


// helper for indenting blocks
pub fn indent ( st: String ) -> String {
    st.lines()
        .map(|line| format!("\t{}", line))
        .collect::<Vec<String>>()
        .join("\n")
}
// helper for adding context to a vec of  errors
pub fn add_context_to_all ( arr: Vec<Error>, context: String ) -> Vec<Error> {
    arr.into_iter()
        .map(|err| err.context(context.clone()))
        .collect()
}
