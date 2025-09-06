use std::collections::{HashMap, HashSet};
use anyhow::{Result, bail};
use crate::ast::{Program, Statement, Expr};

#[derive(Debug, Default)]
struct Scope {
    variables:  HashSet<String>,
    procedures: HashMap<String, ProcedureInfo>,
    functions:  HashMap<String, FunctionInfo>,
    // Track function/procedure arguments that cannot be modified
    immutable_arguments: HashSet<String>,
}

#[derive(Debug, Clone)]
struct ProcedureInfo {
    arg_count: usize,
}
#[derive(Debug, Clone)]
struct FunctionInfo {
    arg_count: usize,
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
            if let Statement::Function { name, args, .. } = statement {
                let func_info = FunctionInfo {
                    arg_count: args.len(),
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
                self.analyze_expression(start);
                self.analyze_expression(end);
                if let Some(step_expr) = step {
                    self.analyze_expression(step_expr);
                }
                
                self.push_scope();
                
                // The loop iterator is a variable in the new scope
                if let Some(scope_mut) = self.current_scope_mut() {
                    scope_mut.variables.insert(iterator.clone());
                }
                
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                
                self.pop_scope();
            }
            Statement::VarDecl { name, .. } => {
                if let Some(scope) = self.current_scope() {
                    if scope.variables.contains(name) {
                        self.add_error(format!("Variable '{}' is already declared", name));
                    } else if let Some(scope_mut) = self.current_scope_mut() {
                        scope_mut.variables.insert(name.clone());
                    }
                }
            }
            Statement::Write { exprs } => {
                for expr in exprs {
                    self.analyze_expression(expr);
                }
            }
            Statement::Assign { name, value } => {
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined during assignment", name));
                }
                
                // Check if trying to modify an immutable argument
                if self.is_immutable_argument(name) {
                    self.add_error(format!("Cannot modify argument '{}' - arguments are immutable in functions and procedures", name));
                }
                
                self.analyze_expression(value);
            },
            Statement::Procedure { args, body, .. } |
            Statement::Function { args, body, .. } => {
                self.push_scope();
                
                if let Some(scope_mut) = self.current_scope_mut() {
                    for arg in args {
                        // Arguments are both variables and immutable
                        scope_mut.variables.insert(arg.clone());
                        scope_mut.immutable_arguments.insert(arg.clone());
                    }
                }
                
                for stmt in body {
                    self.analyze_statement(stmt);
                }
                
                self.pop_scope();
            },
            Statement::ProcedureCall { name, args } => {
                if let Some(proc_info) = self.find_procedure(name) {
                    if args.len() != proc_info.arg_count {
                        self.add_error(
                            format!("Procedure '{}' expects {} argument(s), but {} were provided", 
                                   name, proc_info.arg_count, args.len())
                        );
                    }
                    
                    for arg in args {
                        self.analyze_expression(arg);
                    }
                } else {
                    self.add_error(format!("Procedure '{}' is not defined", name));
                }
            },
            Statement::FunctionCall { name, args }=> {
                if let Some(proc_info) = self.find_function(name) {
                    if args.len() != proc_info.arg_count {
                        self.add_error(
                            format!("Function '{}' expects {} argument(s), but {} were provided", 
                                   name, proc_info.arg_count, args.len())
                        );
                    }
                    
                    for arg in args {
                        self.analyze_expression(arg);
                    }
                } else {
                    self.add_error(format!("Procedure '{}' is not defined", name));
                }
            }
        }
    }

    fn analyze_expression(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(_) => { },
            Expr::String(_) => { },
            Expr::Var(name) => {
                if !self.is_variable_defined(name) {
                    self.add_error(format!("Variable '{}' is not defined in expression `{expr:?}`", name));
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
            if scope.variables.contains(name) {
                return true;
            }
        }
        false
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