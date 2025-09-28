use crate::ast::*;
use crate::analysis::ProgramAnalyzer;
use anyhow::{Result, Context, ensure};
use std::collections::{HashMap, HashSet};

use rosy_lib::RosyType;

#[derive(Default, Clone)]
pub struct TranspileContext {
    args: HashMap<String, RosyType>,
    // Global variable analysis results
    global_variables: HashSet<String>,
    global_variable_types: HashMap<String, RosyType>,
    procedure_global_usage: HashMap<String, Vec<String>>,
}

impl TranspileContext {
    pub fn with_args(args: &[VariableData]) -> Self {
        let mut arg_map = HashMap::new();
        for arg in args {
            arg_map.insert(arg.name.clone(), arg.r#type.clone());
        }
        Self { 
            args: arg_map,
            ..Default::default()
        }
    }

    pub fn with_global_analysis(
        global_vars: HashSet<String>, 
        global_types: HashMap<String, RosyType>,
        proc_global_usage: HashMap<String, Vec<String>>
    ) -> Self {
        Self {
            global_variables: global_vars,
            global_variable_types: global_types,
            procedure_global_usage: proc_global_usage,
            ..Default::default()
        }
    }

    pub fn get_procedure_globals(&self, proc_name: &str) -> Vec<String> {
        self.procedure_global_usage.get(proc_name).cloned().unwrap_or_default()
    }

    pub fn get_global_type(&self, var_name: &str) -> Option<&RosyType> {
        self.global_variable_types.get(var_name)
    }

    /// Smart reference conversion - converts &mut T to &T when needed for trait calls
    pub fn smart_reference(&self, expr: &Expr, expr_str: &str) -> String {
        match expr {
            Expr::Var(name) => {
                if self.args.contains_key(name) {
                    // This is a procedure parameter (&mut T), convert to &T
                    format!("&*{}", expr_str)
                } else {
                    // This is a local/global variable, just add &
                    format!("&{}", expr_str)
                }
            },
            // For operations that return owned values (like Extract, StringConvert), just add &
            Expr::Extract { .. } | Expr::StringConvert { .. } => {
                format!("(&{})", expr_str)
            },
            _ => {
                // For other complex expressions, just wrap in parentheses and add &
                format!("(&{})", expr_str)
            }
        }
    }
}

pub trait Transpile {
    fn transpile(&self) -> Result<String> {
        self.transpile_with_context(&TranspileContext::default())
    }

    fn transpile_with_context(&self, context: &TranspileContext) -> Result<String>;
}

impl Transpile for Program {
    fn transpile_with_context(&self, _context: &TranspileContext) -> Result<String> {
        // First, perform analysis to collect global variable usage
        let mut analyzer = ProgramAnalyzer::new();
        analyzer.analyze(self)?;
        
        // Extract analysis results
        let mut global_vars = HashSet::new();
        let mut global_types = HashMap::new();
        let mut proc_global_usage = HashMap::new();
        
        // Collect global variables and their types
        for statement in &self.statements {
            if let Statement::VarDecl { data } = statement {
                global_vars.insert(data.name.clone());
                global_types.insert(data.name.clone(), data.r#type.clone());
            }
        }
        
        // Create a new analyzer to get global usage info
        let mut temp_analyzer = ProgramAnalyzer::new();
        temp_analyzer.analyze(self)?;
        
        // Build procedure global usage map
        for statement in &self.statements {
            match statement {
                Statement::Procedure { name, .. } | Statement::Function { name, .. } => {
                    let globals = temp_analyzer.get_procedure_globals(name);
                    proc_global_usage.insert(name.clone(), globals);
                }
                _ => {}
            }
        }
        
        // Create context with global analysis
        let context = TranspileContext::with_global_analysis(
            global_vars,
            global_types,
            proc_global_usage
        );

        let mut output = String::new();

        // Transpile all statements (global variables and functions/procedures)
        for statement in &self.statements {
            let statement_st: String = statement.transpile_with_context(&context)
                .context("Failed to convert statement to string!")?;
            output.push_str(&statement_st);
            output.push('\n');
        }

        Ok(output)
    }
}

impl Transpile for Expr {
    fn transpile_with_context(&self, context: &TranspileContext) -> Result<String> {
        let res = match self {
            Expr::Number(n) => {
                format!("{}f64", n)
            },
            Expr::String(s) => {
                format!("String::from(\"{}\")", s)
            },
            Expr::Boolean(b) => {
                format!("{}", b)
            },
            Expr::Var(name) => {
                name.to_string()
            },
            Expr::Exp { expr } => {
                let _sub_expr: String = (*expr).transpile_with_context(context)
                    .context("Failed to convert sub-expression to string!")?;
                todo!();
            },
            Expr::Complex { expr } => {
                let sub_expr: String = (*expr).transpile_with_context(context)
                    .context("Failed to convert complex expression to string!")?;
                format!("{}.cm().context(\"...while trying to convert to a CM!\")?", sub_expr)
            },
            Expr::Add { left, right } => {
                let left_str: String = (*left).transpile_with_context(context)
                    .context("Failed to convert left expression to string!")?;
                let right_str: String = (*right).transpile_with_context(context)
                    .context("Failed to convert right expression to string!")?;
                
                // For procedure parameters, use &* to convert from &mut to &
                let left_ref = context.smart_reference(left, &left_str);
                let right_ref = context.smart_reference(right, &right_str);
                
                format!("({}.rosy_add({}))", left_ref, right_ref)
            },
            Expr::Concat { terms } => {
                let term_strs: Result<Vec<String>> = terms.iter()
                    .map(|term| {
                        let term_str = term.transpile_with_context(context)?;
                        // For procedure parameters, use &* to convert from &mut to &
                        Ok(context.smart_reference(term, &term_str))
                    })
                    .collect();
                
                let term_strs = term_strs?;
                format!(
                    "({})",
                    term_strs.join(").concat(")
                )
            },
            Expr::FunctionCall { name, args } => {
                let mut arg_strs = Vec::new();
                for arg in args {
                    let arg_st: String = arg.transpile_with_context(context)
                        .context("Failed to convert argument expression to string!")?;
                    // Add reference for function call arguments since functions expect &Cosy
                    arg_strs.push(format!("&{}", arg_st));
                }
                
                format!("{}({}).with_context(|| format!(\"...while trying to call function {}!\"))?", name, arg_strs.join(", "), name)
            },
            Expr::Extract { object, index } => {
                let object_str = object.transpile_with_context(context)
                    .context("Failed to convert extract object to string!")?;
                let index_str = index.transpile_with_context(context)
                    .context("Failed to convert extract index to string!")?;
                
                // Always use parentheses and smart reference conversion for object
                let object_ref = match object.as_ref() {
                    Expr::Var(name) => {
                        if context.args.contains_key(name) {
                            format!("(&*{})", name)
                        } else {
                            format!("(&{})", name)
                        }
                    },
                    _ => {
                        format!("({})", object_str)
                    }
                };
                
                // Smart reference conversion for index
                let index_ref = context.smart_reference(index, &index_str);
                
                format!("{}.rosy_extract({}).context(\"...while trying to extract component!\")?", object_ref, index_ref)
            },
            Expr::StringConvert { expr } => {
                let sub_expr: String = (*expr).transpile_with_context(context)
                    .context("Failed to convert string conversion expression to string!")?;
                
                // Always use parentheses and smart reference conversion
                let expr_ref = match expr.as_ref() {
                    Expr::Var(name) => {
                        if context.args.contains_key(name) {
                            format!("(&*{})", name)
                        } else {
                            format!("(&{})", name)
                        }
                    },
                    _ => {
                        format!("({})", sub_expr)
                    }
                };
                
                format!("{}.rosy_to_string().context(\"...while trying to convert to string!\")?", expr_ref)
            },
        };

        Ok(res)
    }
}
impl Transpile for Statement {
    fn transpile_with_context(&self, context: &TranspileContext) -> Result<String> {
        match self {
            Statement::Loop { 
                iterator, start, end, step,
                body 
            } => {
                let loop_iterator = {
                    let start = start.transpile_with_context(context)
                        .context("Failed to convert loop start expression to string!")?;
                    let end = end.transpile_with_context(context)
                        .context("Failed to convert loop end expression to string!")?;
                    let mut body = format!("({start} as usize)..=({end} as usize)");

                    if let Some(step_expr) = step {
                        let step = step_expr.transpile_with_context(context)
                            .context("Failed to convert loop step expression to string!")?;
                        body = format!("({}).step_by({} as usize)", body, step);
                    }

                    body
                };
                let body_stmts = {
                    let mut stmts = Vec::new();
                    for stmt in body {
                        let stmt_st: String = stmt.transpile_with_context(context)
                            .context("Failed to convert loop body statement to string!")?
                            .lines()
                            .map(|line| format!("\t{}", line))
                            .collect::<Vec<String>>()
                            .join("\n");
                        stmts.push(stmt_st);
                    }
                    stmts
                };
                
                Ok(format!(
                    "for {} in {} {{\n{}\n}}",
                    iterator,
                    loop_iterator,
                    body_stmts.join("\n")
                ))
            },
            Statement::VarDecl { data, .. } => {
                let rust_type = data.r#type.as_rust_type();

                let default_init = match data.r#type {
                    RosyType::VE => " = vec!()",
                    RosyType::RE => " = 0.0",
                    RosyType::ST => " = String::new()",
                    RosyType::LO => " = false",
                    RosyType::CM => " = (0.0, 0.0)",
                };

                Ok(format!("let mut {}: {}{};", data.name, rust_type, default_init))
            },
            Statement::Write { unit, exprs } => {
                let mut exprs_sts = Vec::new();

                ensure!(*unit == 6, "Only WRITE with unit 6 (console) is supported so far!");

                for expr in exprs {
                    let expr_st: String = expr.transpile_with_context(context)
                        .context("Failed to convert expression to string!")?;
                    
                    // Use smart reference conversion for all expressions
                    let display_expr = match expr {
                        Expr::Var(name) => {
                            // For simple variables that are procedure parameters, use &* 
                            if context.args.contains_key(name) {
                                format!("(&*{}).rosy_display()", name)
                            } else {
                                format!("(&{}).rosy_display()", expr_st)
                            }
                        },
                        _ => {
                            // For complex expressions, call rosy_display directly since 
                            // they already return owned values or handle their own referencing
                            format!("({}).rosy_display()", expr_st)
                        }
                    };
                    
                    exprs_sts.push(display_expr);
                }

                Ok(format!(
                    "println!(\"{}\", {});",
                    exprs_sts.iter().map(|_| "{}").collect::<Vec<_>>().join(""),
                    exprs_sts.join(", ")
                ))
            },
            Statement::Read { unit, name } => {
                ensure!(*unit == 5, "Only READ with unit 5 (stdin) is supported so far!");
                Ok(format!("{} = from_stdin().context(\"...while trying to read from stdin!\")?;", name))
            },
            Statement::Assign { name, value } => {
                let value_st = value.transpile_with_context(context)
                    .context("Failed to convert assignment value to string!")?;
                
                // Check if this is a procedure parameter (already a mutable reference)
                if context.args.contains_key(name) {
                    Ok(format!("*{} = {}.to_owned();", name, value_st))
                } else {
                    // Local variable or global variable from main scope
                    Ok(format!("{} = {}.to_owned();", name, value_st))
                }
            },
            Statement::Procedure {
                name,
                args,
                body
            } => {
                // Get the global variables this procedure uses
                let global_vars = context.get_procedure_globals(name);
                
                // Create context for procedure body that knows about the arguments AND globals
                let mut body_context = TranspileContext::with_args(args);
                
                // Add global variables as procedure parameters to the context
                for global_var in &global_vars {
                    if let Some(global_type) = context.get_global_type(global_var) {
                        body_context.args.insert(global_var.clone(), global_type.clone());
                    }
                }
                
                body_context.global_variables = context.global_variables.clone();
                body_context.global_variable_types = context.global_variable_types.clone();
                body_context.procedure_global_usage = context.procedure_global_usage.clone();
                
                let mut body_sts = Vec::new();
                for stmt in body {
                    let stmt_st: String = stmt.transpile_with_context(&body_context)
                        .context("Failed to convert statement to string!")?
                        .lines()
                        .map(|line| format!("\t{}", line))
                        .collect::<Vec<String>>()
                        .join("\n");
                    body_sts.push(stmt_st);
                }

                // Add type annotations for procedure arguments
                let mut args_with_types: Vec<String> = args.iter()
                    .map(|arg| {
                        let rust_type = arg.r#type.as_rust_type();
                        format!("{}: &{}", arg.name, rust_type)
                    })
                    .collect();

                // Add global variables as mutable reference parameters
                for global_var in &global_vars {
                    if let Some(global_type) = context.get_global_type(global_var) {
                        let rust_type = global_type.as_rust_type();
                        args_with_types.push(format!("{}: &mut {}", global_var, rust_type));
                    }
                }

                Ok(format!(
                    "fn {} ( {} ) -> Result<()> {{\n{}\n\tOk(())\n}}",
                    name,
                    args_with_types.join(", "),
                    body_sts.join("\n")
                ))
            },
            Statement::ProcedureCall { name, args } => {
                let mut arg_strs = Vec::new();
                
                // Add the explicit arguments first
                for arg in args {
                    let arg_st: String = arg.transpile_with_context(context)
                        .context("Failed to convert argument expression to string!")?;
                    // Add reference for procedure call arguments since procedures expect &Cosy
                    arg_strs.push(format!("&{}", arg_st));
                }
                
                // Add the required global variables as references
                let global_vars = context.get_procedure_globals(name);
                for global_var in &global_vars {
                    // If we're inside a procedure that already has this as a parameter, pass it directly
                    if context.args.contains_key(global_var) {
                        arg_strs.push(global_var.clone());
                    } else {
                        // Otherwise, pass a mutable reference to the global variable
                        arg_strs.push(format!("&mut {}", global_var));
                    }
                }
                
                Ok(format!("{}({}).with_context(|| format!(\"...while trying to call procedure {}!\"))?;", name, arg_strs.join(", "), name))
            },
            Statement::Function {
                name,
                args,
                return_type,
                body
            } => {
                // Create context for function body that knows about the arguments
                let body_context = TranspileContext::with_args(args);

                let mut body_sts = Vec::new();
                for stmt in body {
                    let stmt_st: String = stmt.transpile_with_context(&body_context)
                        .context("Failed to convert statement to string!")?
                        .lines()
                        .map(|line| format!("\t{}", line))
                        .collect::<Vec<String>>()
                        .join("\n");
                    body_sts.push(stmt_st);
                }

                Ok(format!("fn {} ( {} ) -> Result<{}> {{\n{}\n\tOk({})\n}}",
                    name,
                    args.into_iter()
                        .map(|var_data| {
                            let rust_type = var_data.r#type.as_rust_type();
                            format!("{}: &{rust_type}", var_data.name)
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                    return_type.as_rust_type(),
                    body_sts.join("\n"),
                    name
                ))
            },
            Statement::FunctionCall { name, args } => {
                let mut arg_strs = Vec::new();
                for arg in args {
                    let arg_st: String = arg.transpile_with_context(context)
                        .context("Failed to convert argument expression to string!")?;
                    // Add reference for function call arguments since functions expect &Cosy
                    arg_strs.push(format!("&{}", arg_st));
                }
                
                Ok(format!("{}({}).with_context(|| format!(\"...while trying to call function {}!\"))?", name, arg_strs.join(", "), name))
            },
            Statement::If { condition, then_body, elseif_clauses, else_body } => {
                let mut condition_st = condition.transpile_with_context(context)
                    .context("Failed to convert IF condition to string!")?;
                
                // If the condition is a boolean variable that's a procedure parameter, dereference it
                if let Expr::Var(name) = condition {
                    if context.args.contains_key(name) {
                        condition_st = format!("*{}", condition_st);
                    }
                }
                
                let mut result = format!("if {} {{", condition_st);
                
                // Add THEN body
                for stmt in then_body {
                    let stmt_st = stmt.transpile_with_context(context)
                        .context("Failed to convert IF body statement to string!")?
                        .lines()
                        .map(|line| format!("\t{}", line))
                        .collect::<Vec<String>>()
                        .join("\n");
                    result.push('\n');
                    result.push_str(&stmt_st);
                }
                
                // Add ELSEIF clauses
                for elseif_clause in elseif_clauses {
                    let mut elseif_condition_st = elseif_clause.condition.transpile_with_context(context)
                        .context("Failed to convert ELSEIF condition to string!")?;
                    
                    // Handle dereferencing for ELSEIF conditions too
                    if let Expr::Var(name) = &elseif_clause.condition {
                        if context.args.contains_key(name) {
                            elseif_condition_st = format!("*{}", elseif_condition_st);
                        }
                    }
                    
                    result.push_str(&format!("\n}} else if {} {{", elseif_condition_st));
                    
                    for stmt in &elseif_clause.body {
                        let stmt_st = stmt.transpile_with_context(context)
                            .context("Failed to convert ELSEIF body statement to string!")?
                            .lines()
                            .map(|line| format!("\t{}", line))
                            .collect::<Vec<String>>()
                            .join("\n");
                        result.push('\n');
                        result.push_str(&stmt_st);
                    }
                }
                
                // Add ELSE clause if present
                if let Some(else_statements) = else_body {
                    result.push_str("\n} else {");
                    
                    for stmt in else_statements {
                        let stmt_st = stmt.transpile_with_context(context)
                            .context("Failed to convert ELSE body statement to string!")?
                            .lines()
                            .map(|line| format!("\t{}", line))
                            .collect::<Vec<String>>()
                            .join("\n");
                        result.push('\n');
                        result.push_str(&stmt_st);
                    }
                }
                
                result.push_str("\n}");
                Ok(result)
            }
        }
    }
}