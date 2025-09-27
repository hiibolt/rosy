use anyhow::{ensure, Context, Result};
use std::collections::HashSet;

use crate::ast::{Expr, Program, Statement};

#[derive(Debug, Default, Clone)]
pub struct TranspileContext {
    /// Variables that are function/procedure arguments (already references)
    function_args: HashSet<String>,
}

impl TranspileContext {
    fn new() -> Self {
        Self::default()
    }
    
    fn with_args(args: &[String]) -> Self {
        Self {
            function_args: args.iter().cloned().collect(),
        }
    }
    
    fn is_function_arg(&self, name: &str) -> bool {
        self.function_args.contains(name)
    }
}

pub trait Transpile {
    fn transpile(&self) -> Result<String> {
        self.transpile_with_context(&TranspileContext::new())
    }
    
    fn transpile_with_context(&self, context: &TranspileContext) -> Result<String>;
}
impl Transpile for Program {
    fn transpile_with_context(&self, _context: &TranspileContext) -> Result<String> {
        let mainfile = std::fs::read_to_string("assets/rust/src/main.rs")
            .context("Failed to read main.rs template file!")?;
        let mut output = mainfile + "\n\n\n\n/// Automatically generated\n";

        // Transpile the AST to Rust
        for statement in &self.statements {
            let statement_st: String = statement.transpile()
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
                format!("Cosy::Real({}f64)", n)
            },
            Expr::String(s) => {
                format!("Cosy::String(String::from(\"{}\"))", s)
            },
            Expr::Var(name) => {
                // If it's a function argument, it's already a reference, so just use the name
                // If it's a regular variable, we'll add the reference when needed in operations
                name.to_string()
            },
            Expr::Exp { expr } => {
                let sub_expr: String = (*expr).transpile_with_context(context)
                    .context("Failed to convert sub-expression to string!")?;
                format!("Cosy::Real(todo!({}))", sub_expr)
            },
            Expr::Complex { expr } => {
                let sub_expr: String = (*expr).transpile_with_context(context)
                    .context("Failed to convert complex expression to string!")?;
                format!("{}.into_complex()", sub_expr)
            },
            Expr::Add { left, right } => {
                let left_str: String = (*left).transpile_with_context(context)
                    .context("Failed to convert left expression to string!")?;
                let right_str: String = (*right).transpile_with_context(context)
                    .context("Failed to convert right expression to string!")?;
                
                // Add reference prefix only if the operand is not already a function argument
                let left_ref = if let Expr::Var(name) = left.as_ref() {
                    if context.is_function_arg(name) {
                        left_str
                    } else {
                        format!("&{}", left_str)
                    }
                } else {
                    format!("&{}", left_str)
                };
                
                let right_ref = if let Expr::Var(name) = right.as_ref() {
                    if context.is_function_arg(name) {
                        right_str
                    } else {
                        format!("&{}", right_str)
                    }
                } else {
                    format!("&{}", right_str)
                };
                
                format!("({} + {})", left_ref, right_ref)
            },
            Expr::Concat { terms } => {
                let term_strs: Result<Vec<String>> = terms.iter()
                    .map(|term| {
                        let term_str = term.transpile_with_context(context)?;
                        // Apply same reference logic as in Add
                        if let Expr::Var(name) = term.as_ref() {
                            if context.is_function_arg(name) {
                                Ok(term_str)
                            } else {
                                Ok(format!("&{}", term_str))
                            }
                        } else {
                            Ok(format!("&{}", term_str))
                        }
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
                
                format!("{}({})", name, arg_strs.join(", "))
            }
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
                    let mut body = format!("{start}.into_usize()..={end}.into_usize()");

                    if let Some(step_expr) = step {
                        let step = step_expr.transpile_with_context(context)
                            .context("Failed to convert loop step expression to string!")?;
                        body = format!("({}).step_by({}.into_usize())", body, step);
                    }

                    body
                };
                let body_stmts = {
                    let mut stmts = Vec::new();
                    for stmt in body {
                        let mut stmt_st: String = stmt.transpile_with_context(context)
                            .context("Failed to convert loop body statement to string!")?;
                        stmt_st.insert(0, '\t');
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
            Statement::VarDecl { name, .. } => {
                Ok(format!("let mut {} = Cosy::Real(0f64);", name))
            },
            Statement::Write { unit, exprs } => {
                let mut exprs_sts = Vec::new();

                ensure!(*unit == 6, "Only WRITE with unit 6 (console) is supported so far!");

                for expr in exprs {
                    let expr_st: String = expr.transpile_with_context(context)
                        .context("Failed to convert expression to string!")?;
                    exprs_sts.push(expr_st);
                }

                Ok(format!(
                    "println!(\"{}\", {});",
                    exprs_sts.iter().map(|_| "{}").collect::<Vec<_>>().join(""),
                    exprs_sts.join(", ")
                ))
            },
            Statement::Read { unit, name } => {
                ensure!(*unit == 5, "Only READ with unit 5 (stdin) is supported so far!");
                Ok(format!("{} = Cosy::from_stdin();", name))
            },
            Statement::Assign { name, value } => {
                let value_st: String = value.transpile_with_context(context)
                    .context("Failed to convert value expression to string!")?;
                Ok(format!("{} = {}.to_owned();", name, value_st))
            },
            Statement::Procedure {
                name,
                args,
                body
            } => {
                let fn_name = if name == "RUN" { "main" } else { &name };

                // Create context for procedure body that knows about the arguments
                let body_context = TranspileContext::with_args(args);
                
                let mut body_sts = Vec::new();
                for stmt in body {
                    let mut stmt_st: String = stmt.transpile_with_context(&body_context)
                        .context("Failed to convert statement to string!")?;
                    stmt_st.insert(0, '\t'); // Indent the body statements
                    body_sts.push(stmt_st);
                }

                // Add type annotations for procedure arguments just like functions
                let args_with_types: Vec<String> = if fn_name == "main" {
                    // main function should have no parameters
                    Vec::new()
                } else {
                    args.iter()
                        .map(|arg| format!("{}: &Cosy", arg))
                        .collect()
                };

                Ok(format!("fn {} ( {} ) {{\n{}\n}}", fn_name, args_with_types.join(", "), body_sts.join("\n")))
            },
            Statement::ProcedureCall { name, args } => {
                let mut arg_strs = Vec::new();
                for arg in args {
                    let arg_st: String = arg.transpile_with_context(context)
                        .context("Failed to convert argument expression to string!")?;
                    // Add reference for procedure call arguments since procedures expect &Cosy
                    arg_strs.push(format!("&{}", arg_st));
                }
                
                Ok(format!("{}({});", name, arg_strs.join(", ")))
            },
            Statement::Function {
                name,
                args,
                body
            } => {
                let fn_name = if name == "RUN" { "main" } else { &name };

                // Create context for function body that knows about the arguments
                let body_context = TranspileContext::with_args(args);

                let mut body_sts = Vec::new();
                for stmt in body {
                    let mut stmt_st: String = stmt.transpile_with_context(&body_context)
                        .context("Failed to convert statement to string!")?;
                    stmt_st.insert(0, '\t'); // Indent the body statements
                    body_sts.push(stmt_st);
                }

                Ok(format!("fn {} ( {} ) -> Cosy {{\n{}\n{}}}",
                    fn_name,
                    args.into_iter()
                        .map(|st| format!("{st}: &Cosy"))
                        .collect::<Vec<String>>()
                        .join(", "),
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
                
                Ok(format!("{}({})", name, arg_strs.join(", ")))
            }
        }
    }
}