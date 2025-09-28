use std::collections::{HashMap, HashSet};
use anyhow::{Result, bail};
use crate::ast::{Program, Statement, Expr};
use rosy_lib::RosyType;

#[derive(Debug, Default)]
struct Scope {
    variables:  HashMap<String, RosyType>, // Changed from HashSet to HashMap to track types
    procedures: HashMap<String, ProcedureInfo>,
    functions:  HashMap<String, FunctionInfo>,
    // Track function/procedure arguments that cannot be modified
    immutable_arguments: HashSet<String>,
}

#[derive(Debug, Clone)]
struct ProcedureInfo {
    arg_count: usize,
    arg_types: Vec<RosyType>, // Add argument types
}

#[derive(Debug, Clone)]
struct FunctionInfo {
    arg_count: usize,
    arg_types: Vec<RosyType>, // Add argument types
    return_type: RosyType,    // Add return type
}

#[derive(Debug)]
struct StaticAnalyzer {
    scope_stack: Vec<Scope>,
    errors: Vec<String>,
}

impl StaticAnalyzer {
    fn new() -> Self {
        Self {
            scope_stack: vec![Scope::default()],
            errors: Vec::new(),
        }
    }

    fn analyze(&mut self, program: &Program) -> Result<()> {
        self.collect_procedures(program);
        self.collect_functions(program);
        
        for statement in &program.statements {
            self.analyze_statement(statement);
        }

        if !self.errors.is_empty() {
            bail!("Static analysis failed with {} error(s):\n{}", 
                  self.errors.len(), 
                  self.errors.join("\n"));
        }

        Ok(())
    }

    fn collect_procedures(&mut self, program: &Program) {
        for statement in &program.statements {
            if let Statement::Procedure { name, args, .. } = statement {
                let proc_info = ProcedureInfo {
                    arg_count: args.len(),
                    arg_types: args.iter().map(|arg| arg.r#type).collect(),
                };
                
                if let Some(scope) = self.current_scope() {
                    if scope.procedures.contains_key(name) {
                        self.add_error(format!("Procedure '{}' is already defined", name));
                    } else if let Some(scope_mut) = self.current_scope_mut() {
                        scope_mut.procedures.insert(name.clone(), proc_info);
                    }
                }
            }
        }
    }
    
    fn collect_functions(&mut self, program: &Program) {
        for statement in &program.statements {
            if let Statement::Function { name, args, return_type, .. } = statement {
                let func_info = FunctionInfo {
                    arg_count: args.len(),
                    arg_types: args.iter().map(|arg| arg.r#type).collect(),
                    return_type: *return_type,
                };
                
                if let Some(scope) = self.current_scope() {
                    if scope.functions.contains_key(name) {
                        self.add_error(format!("Function '{}' is already defined", name));
                    } else if let Some(scope_mut) = self.current_scope_mut() {
                        scope_mut.functions.insert(name.clone(), func_info);
                    }
                }
            }
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Loop { iterator, start, end, step, body } => {
                // Check that loop bounds are numeric (RE type)
                if let Some(start_type) = self.get_expression_type(start) {
                    if start_type != RosyType::RE {
                        self.add_error(format!("Loop start expression must be of type RE, found {:?}", start_type));
                    }
                } else {
                    self.analyze_expression(start);
                }
                
                if let Some(end_type) = self.get_expression_type(end) {
                    if end_type != RosyType::RE {
                        self.add_error(format!("Loop end expression must be of type RE, found {:?}", end_type));
                    }
                } else {
                    self.analyze_expression(end);
                }
                
                if let Some(step_expr) = step {
                    if let Some(step_type) = self.get_expression_type(step_expr) {
                        if step_type != RosyType::RE {
                            self.add_error(format!("Loop step expression must be of type RE, found {:?}", step_type));
                        }
                    } else {
                        self.analyze_expression(step_expr);
                    }
                }
                
                self.push_scope();
                
                // The loop iterator is a RE variable in the new scope
                if let Some(scope_mut) = self.current_scope_mut() {
                    scope_mut.variables.insert(iterator.clone(), RosyType::RE);
                }
                
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                
                self.pop_scope();
            },
            Statement::VarDecl { data, .. } => {
                if let Some(scope) = self.current_scope() {
                    if scope.variables.contains_key(&data.name) {
                        self.add_error(format!("Variable '{}' is already declared", data.name));
                    } else if let Some(scope_mut) = self.current_scope_mut() {
                        scope_mut.variables.insert(data.name.clone(), data.r#type);
                    }
                }
            },
            Statement::Write { exprs, .. } => {
                for expr in exprs {
                    self.analyze_expression(expr);
                }
            },
            Statement::Read { name, .. } => {
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined during READ", name));
                }
            },
            Statement::Assign { name, value } => {
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined during assignment", name));
                } else {
                    // Type checking for assignment
                    if let Some(var_type) = self.get_variable_type(name) {
                        if let Some(expr_type) = self.get_expression_type(value) {
                            if var_type != expr_type {
                                self.add_error(format!(
                                    "Type mismatch in assignment to '{}': expected {:?}, found {:?}",
                                    name, var_type, expr_type
                                ));
                            }
                        } else {
                            // Still analyze the expression for other errors
                            self.analyze_expression(value);
                        }
                    }
                }
                
                // Check if trying to modify an immutable argument
                if self.is_immutable_argument(name) {
                    self.add_error(format!("Cannot modify argument '{}' - arguments are immutable in functions and procedures", name));
                }
            },
            Statement::Procedure { args, body, .. } |
            Statement::Function { args, body, .. } => {
                self.push_scope();
                
                if let Some(scope_mut) = self.current_scope_mut() {
                    for arg in args {
                        // Arguments are both variables and immutable
                        scope_mut.variables.insert(arg.name.clone(), arg.r#type);
                        scope_mut.immutable_arguments.insert(arg.name.clone());
                    }
                }
                
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                
                self.pop_scope();
            },
            Statement::ProcedureCall { name, args } => {
                // First collect expected types and arg count
                let (expected_arg_count, expected_arg_types) = if let Some(proc_info) = self.find_procedure(name) {
                    (proc_info.arg_count, proc_info.arg_types.clone())
                } else {
                    self.add_error(format!("Procedure '{}' is not defined", name));
                    return;
                };

                if args.len() != expected_arg_count {
                    self.add_error(
                        format!("Procedure '{}' expects {} argument(s), but {} were provided", 
                               name, expected_arg_count, args.len())
                    );
                } else {
                    // Check argument types
                    for (i, (arg_expr, expected_type)) in args.iter().zip(expected_arg_types.iter()).enumerate() {
                        if let Some(actual_type) = self.get_expression_type(arg_expr) {
                            if actual_type != *expected_type {
                                self.add_error(format!(
                                    "Procedure '{}' argument {} type mismatch: expected {:?}, found {:?}",
                                    name, i + 1, expected_type, actual_type
                                ));
                            }
                        } else {
                            self.analyze_expression(arg_expr);
                        }
                    }
                }
            },
            Statement::FunctionCall { name, args }=> {
                // First collect expected types and arg count
                let (expected_arg_count, expected_arg_types) = if let Some(func_info) = self.find_function(name) {
                    (func_info.arg_count, func_info.arg_types.clone())
                } else {
                    self.add_error(format!("Function '{}' is not defined", name));
                    return;
                };

                if args.len() != expected_arg_count {
                    self.add_error(
                        format!("Function '{}' expects {} argument(s), but {} were provided", 
                               name, expected_arg_count, args.len())
                    );
                } else {
                    // Check argument types
                    for (i, (arg_expr, expected_type)) in args.iter().zip(expected_arg_types.iter()).enumerate() {
                        if let Some(actual_type) = self.get_expression_type(arg_expr) {
                            if actual_type != *expected_type {
                                self.add_error(format!(
                                    "Function '{}' argument {} type mismatch: expected {:?}, found {:?}",
                                    name, i + 1, expected_type, actual_type
                                ));
                            }
                        } else {
                            self.analyze_expression(arg_expr);
                        }
                    }
                }
            },
            Statement::If { condition, then_body, elseif_clauses, else_body } => {
                // Check that the main IF condition is of type LO (boolean)
                if let Some(condition_type) = self.get_expression_type(condition) {
                    if condition_type != RosyType::LO {
                        self.add_error(format!("IF condition must be of type LO (boolean), found {:?}", condition_type));
                    }
                } else {
                    self.analyze_expression(condition);
                }
                
                // Analyze statements in the THEN body
                for stmt in then_body {
                    self.analyze_statement(stmt);
                }
                
                // Analyze ELSEIF clauses
                for elseif_clause in elseif_clauses {
                    // Check that each ELSEIF condition is of type LO (boolean)
                    if let Some(condition_type) = self.get_expression_type(&elseif_clause.condition) {
                        if condition_type != RosyType::LO {
                            self.add_error(format!("ELSEIF condition must be of type LO (boolean), found {:?}", condition_type));
                        }
                    } else {
                        self.analyze_expression(&elseif_clause.condition);
                    }
                    
                    // Analyze statements in the ELSEIF body
                    for stmt in &elseif_clause.body {
                        self.analyze_statement(stmt);
                    }
                }
                
                // Analyze ELSE body if present
                if let Some(else_statements) = else_body {
                    for stmt in else_statements {
                        self.analyze_statement(stmt);
                    }
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
                todo!();
            },
            Expr::Complex { expr: inner } => {
                let inner_type = if let Some(inner_type) = self.get_expression_type(inner) {
                    inner_type
                } else {
                    self.analyze_expression(inner);
                    return None;
                };

                match inner_type.cm_intrinsic_result() {
                    Some(result_type) => Some(result_type),
                    None => {
                        self.add_error(format!("Cannot apply EXP to type {:?}", inner_type));
                        None
                    }
                }
            },
            Expr::Add { left, right } => {
                let left_type = if let Some(left_type) = self.get_expression_type(left) {
                    left_type
                } else {
                    self.analyze_expression(left);
                    return None;
                };
                let right_type = if let Some(right_type) = self.get_expression_type(right) {
                    right_type
                } else {
                    self.analyze_expression(right);
                    return None;
                };
                
                if let Some(result_type) = left_type.add_operation_result(&right_type) {
                    Some(result_type)
                } else {
                    self.add_error(format!("Invalid addition operation between {:?} and {:?}", left_type, right_type));
                    None
                }
            },
            Expr::Concat { terms } => {
                if terms.is_empty() {
                    self.add_error("Concatenation requires at least one term".to_string());
                    return None;
                }
                
                let first_type = if let Some(first_type) = self.get_expression_type(&terms[0]) {
                    first_type
                } else {
                    self.analyze_expression(&terms[0]);
                    return None;
                };
                
                let mut current_type = first_type;
                
                for term in &terms[1..] {
                    let term_type = if let Some(term_type) = self.get_expression_type(term) {
                        term_type
                    } else {
                        self.analyze_expression(term);
                        return None;
                    };
                    
                    if let Some(result_type) = current_type.concat_operation_result(&term_type) {
                        current_type = result_type;
                    } else {
                        self.add_error(format!("Invalid concatenation operation between {:?} and {:?}", current_type, term_type));
                        return None;
                    }
                }
                
                Some(current_type)
            },
            Expr::FunctionCall { name, args } => {
                // First collect expected types and return type
                let (expected_arg_count, expected_arg_types, return_type) = if let Some(func_info) = self.find_function(name) {
                    (func_info.arg_count, func_info.arg_types.clone(), func_info.return_type)
                } else {
                    self.add_error(format!("Function '{}' is not defined", name));
                    return None;
                };

                if args.len() != expected_arg_count {
                    self.add_error(
                        format!("Function '{}' expects {} argument(s), but {} were provided", 
                               name, expected_arg_count, args.len())
                    );
                } else {
                    // Check argument types
                    for (i, (arg_expr, expected_type)) in args.iter().zip(expected_arg_types.iter()).enumerate() {
                        if let Some(actual_type) = self.get_expression_type(arg_expr) {
                            if actual_type != *expected_type {
                                self.add_error(format!(
                                    "Function '{}' argument {} type mismatch: expected {:?}, found {:?}",
                                    name, i + 1, expected_type, actual_type
                                ));
                            }
                        } else {
                            self.analyze_expression(arg_expr);
                        }
                    }
                }
                
                Some(return_type)
            },
        }
    }

    fn analyze_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(_) => { },
            Expr::String(_) => { },
            Expr::Boolean(_) => { },
            Expr::Var(name) => {
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined in expression", name));
                }
            },
            Expr::Exp { expr } => {
                self.analyze_expression(expr);
            },
            Expr::Complex { expr } => {
                self.analyze_expression(expr);
            },
            Expr::Add { left, right } => {
                self.analyze_expression(left);
                self.analyze_expression(right);
            },
            Expr::Concat { terms } => {
                for term in terms {
                    self.analyze_expression(term);
                }
            },
            Expr::FunctionCall { name, args } => {
                if let Some(func_info) = self.find_function(name) {
                    if args.len() != func_info.arg_count {
                        self.add_error(
                            format!("Function '{}' expects {} argument(s), but {} were provided", 
                                   name, func_info.arg_count, args.len())
                        );
                    }
                    
                    for arg in args {
                        self.analyze_expression(arg);
                    }
                } else {
                    self.add_error(format!("Function '{}' is not defined", name));
                }
            },
        }
    }

    fn is_variable_defined(&self, name: &str) -> bool {
        for scope in self.scope_stack.iter().rev() {
            if scope.variables.contains_key(name) {
                return true;
            }
        }
        false
    }
    
    fn get_variable_type(&self, name: &str) -> Option<RosyType> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(var_type) = scope.variables.get(name) {
                return Some(*var_type);
            }
        }
        None
    }

    fn is_immutable_argument(&self, name: &str) -> bool {
        for scope in self.scope_stack.iter().rev() {
            if scope.immutable_arguments.contains(name) {
                return true;
            }
        }
        false
    }

    fn find_procedure(&self, name: &str) -> Option<&ProcedureInfo> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(proc_info) = scope.procedures.get(name) {
                return Some(proc_info);
            }
        }
        None
    }
    fn find_function(&self, name: &str) -> Option<&FunctionInfo> {
        for scope in self.scope_stack.iter().rev() {
            if let Some(func_info) = scope.functions.get(name) {
                return Some(func_info);
            }
        }
        None
    }

    fn current_scope(&self) -> Option<&Scope> {
        self.scope_stack.last()
    }

    fn current_scope_mut(&mut self) -> Option<&mut Scope> {
        self.scope_stack.last_mut()
    }

    fn push_scope(&mut self) {
        self.scope_stack.push(Scope::default());
    }

    fn pop_scope(&mut self) {
        if self.scope_stack.len() > 1 {
            self.scope_stack.pop();
        }
    }

    fn add_error(&mut self, message: String) {
        self.errors.push(message);
    }
}

/// Convenience function to perform static analysis on a program
pub fn analyze_program(program: &Program) -> Result<()> {
    let mut analyzer = StaticAnalyzer::new();
    analyzer.analyze(program)
}