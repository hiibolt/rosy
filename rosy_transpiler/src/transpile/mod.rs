mod expr;
mod var_decl;
mod procedure;
mod function;
mod assign;
mod variable_data;
mod write;
mod function_call;
mod procedure_call;

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
            _ => todo!()
        }
    }
}

/*
pub struct ProgramAnalyzer {
    procedure_global_usage: HashMap<String, HashSet<String>>,
    errors: Vec<String>,
}

impl ProgramAnalyzer {
    pub fn new() -> Self {
        Self {
            procedure_global_usage: HashMap::new(),
            errors: Vec::new(),
        }
    }

    fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn analyze(&mut self, program: &Program) -> Result<()> {
        // First pass: collect global variables and procedure/function signatures
        let mut variables:  HashMap<String, (LevelsAbove, VariableData)> = HashMap::new();
        let mut functions:  HashMap<String, (VariableType, Vec<VariableData>)> = HashMap::new();
        let mut procedures: HashMap<String, Vec<VariableData>> = HashMap::new();
        for statement in &program.statements {
            match statement {
                Statement::VarDecl { data } => {
                    variables.insert(data.name.clone(), (0, data.clone()));
                }
                Statement::Procedure { name, args, .. } => {
                    procedures.insert(name.clone(), args.clone());
                }
                Statement::Function { name, args, return_type, .. } => {
                    functions.insert(name.clone(), (return_type.clone(), args.clone()));
                }
                _ => {}
            }
        }

        // Second pass: analyze procedure/function bodies for direct global variable usage
        for statement in &program.statements {
            match statement {
                Statement::Procedure { name, args, body } => {
                    self.analyze_procedure_body(name, args, body);
                }
                Statement::Function { name, args, body, .. } => {
                    self.analyze_function_body(name, args, body);
                }
                _ => {}
            }
        }

        // Third pass: propagate transitive global variable dependencies
        let mut changed = true;
        while changed {
            changed = false;
            for statement in &program.statements {
                if let Statement::Procedure { name, body, .. } = statement {
                    let original_usage = self.procedure_global_usage.get(name).cloned().unwrap_or_default();
                    let mut updated_usage = original_usage.clone();
                    
                    // Add transitive dependencies from procedure calls
                    for stmt in body {
                        self.collect_transitive_globals(stmt, &mut updated_usage);
                    }
                    
                    if updated_usage != original_usage {
                        self.procedure_global_usage.insert(name.clone(), updated_usage);
                        changed = true;
                    }
                }
            }
        }

        // Fourth pass: type checking for procedure/function/array definition bodies
        for statement in &program.statements {
            match statement {
                Statement::Procedure { name: _, args, body } => {
                    self.analyze_procedure_body_types(args, body);
                }
                Statement::Function { name: _, args, body, return_type } => {
                    self.analyze_function_body_types(args, body, return_type);
                }
                _ => {}
            }
        }

        // Fifth pass: analyze top-level statements (like procedure calls) with full type checking
        for statement in &program.statements {
            self.analyze_statement_types(statement);
        }

        Ok(())
    }

    fn collect_transitive_globals(&self, stmt: &Statement, global_usage: &mut HashSet<String>) {
        match stmt {
            Statement::ProcedureCall { name, .. } => {
                if let Some(called_proc_globals) = self.procedure_global_usage.get(name) {
                    for global_var in called_proc_globals {
                        global_usage.insert(global_var.clone());
                    }
                }
            }
            Statement::If { then_body, elseif_clauses, else_body, .. } => {
                for stmt in then_body {
                    self.collect_transitive_globals(stmt, global_usage);
                }
                for elseif_clause in elseif_clauses {
                    for stmt in &elseif_clause.body {
                        self.collect_transitive_globals(stmt, global_usage);
                    }
                }
                if let Some(else_statements) = else_body {
                    for stmt in else_statements {
                        self.collect_transitive_globals(stmt, global_usage);
                    }
                }
            }
            Statement::Loop { body, .. } => {
                for stmt in body {
                    self.collect_transitive_globals(stmt, global_usage);
                }
            }
            _ => {}
        }
    }

    fn analyze_procedure_body(&mut self, proc_name: &str, args: &[VariableData], body: &[Statement]) {
        // Create a new scope for this procedure
        let mut local_vars = HashSet::new();
        let mut global_usage = HashSet::new();
        
        // Add procedure arguments to local scope
        for arg in args {
            local_vars.insert(arg.name.clone());
        }
        
        // Analyze the procedure body
        for stmt in body {
            self.analyze_statement_for_globals(stmt, &mut local_vars, &mut global_usage);
        }
        
        // Store the global usage for this procedure
        self.procedure_global_usage.insert(proc_name.to_string(), global_usage);
    }

    fn analyze_function_body(&mut self, func_name: &str, args: &[VariableData], body: &[Statement]) {
        // Create a new scope for this function
        let mut local_vars = HashSet::new();
        let mut global_usage = HashSet::new();
        
        // Add function arguments to local scope
        for arg in args {
            local_vars.insert(arg.name.clone());
        }
        
        // Analyze the function body
        for stmt in body {
            self.analyze_statement_for_globals(stmt, &mut local_vars, &mut global_usage);
        }
        
        // Store the global usage for this function
        self.procedure_global_usage.insert(func_name.to_string(), global_usage);
    }

    fn analyze_statement_for_globals(&mut self, stmt: &Statement, local_vars: &mut HashSet<String>, global_usage: &mut HashSet<String>) {
        match stmt {
            Statement::VarDecl { data } => {
                // Local variable declaration
                local_vars.insert(data.name.clone());
            }
            Statement::Assign { name, value, indicies } => {
                // Check if we're assigning to a global variable
                if self.global_variables.contains(name) && !local_vars.contains(name) {
                    global_usage.insert(name.clone());
                }

                // Analyze the index expressions
                for expr in indicies {
                    self.analyze_expression_for_globals(expr, local_vars, global_usage);
                }

                // Analyze the expression for global variable usage
                self.analyze_expression_for_globals(value, local_vars, global_usage);
            }
            Statement::Write { exprs, .. } => {
                for expr in exprs {
                    self.analyze_expression_for_globals(expr, local_vars, global_usage);
                }
            }
            Statement::Read { name, .. } => {
                if self.global_variables.contains(name) && !local_vars.contains(name) {
                    global_usage.insert(name.clone());
                }
            }
            Statement::ProcedureCall { name, args } => {
                // Analyze arguments for global usage
                for arg in args {
                    self.analyze_expression_for_globals(arg, local_vars, global_usage);
                }
                
                // Add transitive global usage: if we call a procedure that uses globals,
                // then we also use those globals transitively
                if let Some(called_proc_globals) = self.procedure_global_usage.get(name).cloned() {
                    for global_var in called_proc_globals {
                        global_usage.insert(global_var);
                    }
                }
            }
            Statement::FunctionCall { name: _, args } => {
                for arg in args {
                    self.analyze_expression_for_globals(arg, local_vars, global_usage);
                }
            }
            Statement::If { condition, then_body, elseif_clauses, else_body } => {
                // Analyze the IF condition for global variable usage
                self.analyze_expression_for_globals(condition, local_vars, global_usage);
                
                // Analyze the THEN body
                for stmt in then_body {
                    self.analyze_statement_for_globals(stmt, local_vars, global_usage);
                }
                
                // Analyze ELSEIF clauses
                for elseif_clause in elseif_clauses {
                    self.analyze_expression_for_globals(&elseif_clause.condition, local_vars, global_usage);
                    for stmt in &elseif_clause.body {
                        self.analyze_statement_for_globals(stmt, local_vars, global_usage);
                    }
                }
                
                // Analyze ELSE body
                if let Some(else_statements) = else_body {
                    for stmt in else_statements {
                        self.analyze_statement_for_globals(stmt, local_vars, global_usage);
                    }
                }
            }
            Statement::Loop { iterator, start, end, step, body } => {
                // Add loop iterator as local variable
                local_vars.insert(iterator.clone());
                
                // Analyze loop bounds for global usage
                self.analyze_expression_for_globals(start, local_vars, global_usage);
                self.analyze_expression_for_globals(end, local_vars, global_usage);
                if let Some(step_expr) = step {
                    self.analyze_expression_for_globals(step_expr, local_vars, global_usage);
                }
                
                // Analyze loop body
                for stmt in body {
                    self.analyze_statement_for_globals(stmt, local_vars, global_usage);
                }
            }
            _ => {}
        }
    }

    fn analyze_expression_for_globals(&mut self, expr: &Expr, local_vars: &HashSet<String>, global_usage: &mut HashSet<String>) {
        match expr {
            Expr::Var(name) => {
                if self.global_variables.contains(name) && !local_vars.contains(name) {
                    global_usage.insert(name.clone());
                }
            }
            Expr::Add { left, right } => {
                self.analyze_expression_for_globals(left, local_vars, global_usage);
                self.analyze_expression_for_globals(right, local_vars, global_usage);
            }
            Expr::Concat { terms } => {
                for term in terms {
                    self.analyze_expression_for_globals(term, local_vars, global_usage);
                }
            }
            Expr::FunctionCall { name: _, args } => {
                for arg in args {
                    self.analyze_expression_for_globals(arg, local_vars, global_usage);
                }
            }
            Expr::Exp { expr: inner } => {
                self.analyze_expression_for_globals(inner, local_vars, global_usage);
            }
            Expr::Complex { expr: inner } => {
                self.analyze_expression_for_globals(inner, local_vars, global_usage);
            }
            Expr::Extract { object, index } => {
                self.analyze_expression_for_globals(object, local_vars, global_usage);
                self.analyze_expression_for_globals(index, local_vars, global_usage);
            },
            Expr::StringConvert { expr } => {
                self.analyze_expression_for_globals(expr, local_vars, global_usage);
            },
            _ => {} // Number, String, Boolean don't reference variables
        }
    }

    // Get the global variables used by a procedure
    pub fn get_procedure_globals(&self, proc_name: &str) -> Vec<String> {
        self.procedure_global_usage.get(proc_name)
            .map(|set| {
                let mut vars: Vec<String> = set.iter().cloned().collect();
                vars.sort(); // For consistent ordering
                vars
            })
            .unwrap_or_default()
    }
    fn is_variable_defined(&self, name: &str) -> bool {
        self.variable_types.contains_key(name)
    }

    fn get_variable_type(&self, name: &str) -> Option<RosyType> {
        self.variable_types.get(name).map(|data| data.r#type.clone())
    }

    fn analyze_procedure_call(&mut self, name: &str, args: &[Expr]) {
        // Check if procedure exists
        if !self.procedure_signatures.contains_key(name) {
            self.add_error(format!("Procedure '{}' is not defined", name));
            return;
        }

        // Get expected arguments
        let _expected_args = &self.procedure_signatures[name];
        
        // Note: We don't check argument count here because we'll be automatically
        // adding global variable references, so the count will change
        
        // Analyze each argument expression
        for arg in args {
            self.analyze_expression(arg);
        }
    }

    fn analyze_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) => {},
            Expr::Var(name) => {
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined in expression", name));
                }
            }
            Expr::Add { left, right } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            }
            Expr::Concat { terms } => {
                for term in terms {
                    self.analyze_expression(term);
                }
            }
            Expr::FunctionCall { name, args } => {
                if !self.function_signatures.contains_key(name) {
                    self.add_error(format!("Function '{}' is not defined", name));
                }
                for arg in args {
                    self.analyze_expression(arg);
                }
            }
            Expr::Exp { expr: inner } => {
                self.analyze_expression(inner);
            }
            Expr::Complex { expr: inner } => {
                self.analyze_expression(inner);
            }
            Expr::Extract { object, index } => {
                self.analyze_expression(object);
                self.analyze_expression(index);
            }
            Expr::StringConvert { expr } => {
                self.analyze_expression(expr);
            },
            Expr::VarIndexing { name, indices } => {
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined in expression", name));
                }
                for index in indices {
                    self.analyze_expression(index);
                }
            }
        }
    }

    /// Recursively determine the type of an expression
    fn get_expression_type(&mut self, expr: &Expr) -> Option<RosyType> {
        match expr {
            Expr::Number(_) => Some(RosyType::RE),
            Expr::String(_) => Some(RosyType::ST),
            Expr::Boolean(_) => Some(RosyType::LO),
            Expr::Var(name) => self.get_variable_type(name),
            Expr::Exp { expr: _inner } => {
                Some(RosyType::CM) // EXP returns complex
            },
            Expr::Complex { expr: _inner } => {
                Some(RosyType::CM) // CM returns complex
            },
            Expr::Add { left: _left, right: _right } => {
                // For now, assume addition returns RE
                Some(RosyType::RE)
            },
            Expr::Concat { terms } => {
                // Check if all terms are strings - if so, return ST, otherwise VE
                let all_strings = terms.iter().all(|term| {
                    if let Some(term_type) = self.get_expression_type(term) {
                        term_type == RosyType::ST
                    } else {
                        false
                    }
                });
                
                if all_strings {
                    Some(RosyType::ST) // String concatenation
                } else {
                    Some(RosyType::VE) // Vector concatenation
                }
            },
            Expr::FunctionCall { name, args: _args } => {
                // Look up the function's return type
                if let Some((return_type, _)) = self.function_signatures.get(name) {
                    Some(return_type.clone())
                } else {
                    None
                }
            },
            Expr::Extract { object, index: _ } => {
                // Extract operation returns different types based on the object type
                if let Some(object_type) = self.get_expression_type(object) {
                    match object_type {
                        RosyType::ST => Some(RosyType::ST), // String extraction returns string
                        RosyType::VE => Some(RosyType::RE), // Vector extraction returns real
                        RosyType::CM => Some(RosyType::RE), // Complex extraction returns real (component)
                        _ => None, // Other types don't support extraction
                    }
                } else {
                    None
                }
            },
            Expr::StringConvert { expr: _ } => {
                // ST() always returns string type regardless of input
                Some(RosyType::ST)
            },
            Expr::VarIndexing { name, .. } => {
                self.get_variable_type(name)
            }
        }
    }

    // Type checking methods for procedure/function bodies
    fn analyze_procedure_body_types(&mut self, args: &[VariableData], body: &[Statement]) {
        // Create local scope with procedure arguments
        let original_vars = self.variable_types.clone();
        
        // Add procedure arguments to scope
        for arg in args {
            self.define_variable(&arg.name, arg.clone());
        }
        
        // Analyze each statement in the procedure body
        for stmt in body {
            self.analyze_statement_types(stmt);
        }
        
        // Restore original variable scope
        self.variable_types = original_vars;
    }

    fn analyze_function_body_types(&mut self, args: &[VariableData], body: &[Statement], _return_type: &RosyType) {
        // Create local scope with function arguments
        let original_vars = self.variable_types.clone();
        
        // Add function arguments to scope
        for arg in args {
            self.define_variable(&arg.name, arg.clone());
        }
        
        // Analyze each statement in the function body
        for stmt in body {
            self.analyze_statement_types(stmt);
        }
        
        // Check for return statements and validate return type
        /* i lowkey think i superceded this with improvements, but should test later. todo!();
        for stmt in body {
            if let Statement::Assign { name, value, indicies } = stmt {
                // In ROSY, functions return by assigning to the function name
                if let Some((_func_return_type, _)) = self.function_signatures.iter()
                    .find(|(_, (_, func_args))| func_args == args)
                    .map(|(_, (ret_type, func_args))| (ret_type, func_args))
                {
                    if self.function_signatures.iter().any(|(func_name, (_, _))| func_name == name) {
                        // Check that return value type matches function return type
                        if let Some(value_type) = self.get_expression_type(value) {
                            if value_type != *return_type {
                                self.add_error(format!(
                                    "Function return type mismatch: expected {:?}, found {:?}",
                                    return_type, value_type
                                ));
                            }
                        }
                    }
                }
            }
        } */
        
        // Note: We don't require explicit return statements in ROSY functions
        // as they can return by assigning to the function name
        
        // Restore original variable scope
        self.variable_types = original_vars;
    }

    fn analyze_statement_types(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VarDecl { data } => {
                self.define_variable(&data.name, data.clone());
            }
            Statement::Write { exprs, .. } => {
                for expr in exprs {
                    self.analyze_expression(expr);
                }
            }
            Statement::Read { name, .. } => {
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined in READ statement", name));
                }
            }
            Statement::Assign { name, value, indicies } => {
                // Check if variable is defined
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined in assignment", name));
                    return;
                }

                // Check that each index expression is an RE
                for index_expr in indicies {
                    if let Some(index_type) = self.get_expression_type(index_expr) {
                        if index_type != RosyType::RE {
                            self.add_error(format!("Array index must be of type RE, found {:?}", index_type));
                        }
                    }
                }
                
                // Check type compatibility
                if let Some(var_type) = self.get_variable_type(name) {
                    if let Some(expr_type) = self.get_expression_type(value) {
                        if var_type != expr_type {
                            self.add_error(format!(
                                "Type mismatch in assignment to '{}': expected {:?}, found {:?}",
                                name, var_type, expr_type
                            ));
                        }
                    }
                }

                
                self.analyze_expression(value);
            }
            Statement::ProcedureCall { name, args } => {
                self.analyze_procedure_call(name, args);
            }
            Statement::FunctionCall { name, args } => {
                if !self.function_signatures.contains_key(name) {
                    self.add_error(format!("Function '{}' is not defined", name));
                }
                for arg in args {
                    self.analyze_expression(arg);
                }
            }
            Statement::If { condition, then_body, elseif_clauses, else_body } => {
                // Check that IF condition is boolean
                if let Some(condition_type) = self.get_expression_type(condition) {
                    if condition_type != RosyType::LO {
                        self.add_error(format!("IF condition must be of type LO (boolean), found {:?}", condition_type));
                    }
                }
                self.analyze_expression(condition);
                
                // Analyze IF body
                for stmt in then_body {
                    self.analyze_statement_types(stmt);
                }
                
                // Analyze ELSEIF clauses
                for elseif_clause in elseif_clauses {
                    if let Some(condition_type) = self.get_expression_type(&elseif_clause.condition) {
                        if condition_type != RosyType::LO {
                            self.add_error(format!("ELSEIF condition must be of type LO (boolean), found {:?}", condition_type));
                        }
                    }
                    self.analyze_expression(&elseif_clause.condition);
                    
                    for stmt in &elseif_clause.body {
                        self.analyze_statement_types(stmt);
                    }
                }
                
                // Analyze ELSE body
                if let Some(else_statements) = else_body {
                    for stmt in else_statements {
                        self.analyze_statement_types(stmt);
                    }
                }
            }
            Statement::Loop { iterator, start, end, step, body } => {
                // Loop iterator should be RE type
                self.define_variable(iterator, VariableData { 
                    name: iterator.clone(), 
                    r#type: RosyType::RE, 
                    dimensions: vec![]
                });
                
                // Start, end, and step should be RE type
                if let Some(start_type) = self.get_expression_type(start) {
                    if start_type != RosyType::RE {
                        self.add_error(format!("LOOP start value must be of type RE, found {:?}", start_type));
                    }
                }
                if let Some(end_type) = self.get_expression_type(end) {
                    if end_type != RosyType::RE {
                        self.add_error(format!("LOOP end value must be of type RE, found {:?}", end_type));
                    }
                }
                if let Some(step_expr) = step {
                    if let Some(step_type) = self.get_expression_type(step_expr) {
                        if step_type != RosyType::RE {
                            self.add_error(format!("LOOP step value must be of type RE, found {:?}", step_type));
                        }
                    }
                }
                
                self.analyze_expression(start);
                self.analyze_expression(end);
                if let Some(step_expr) = step {
                    self.analyze_expression(step_expr);
                }
                
                // Analyze loop body
                for stmt in body {
                    self.analyze_statement_types(stmt);
                }
            }
            _ => {}
        }
    }
} */