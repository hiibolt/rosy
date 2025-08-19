use anyhow::{Result, Context};

use crate::ast::{Expr, Program, Statement};

pub trait Transpile {
    fn transpile(&self) -> Result<String>;
}
impl Transpile for Program {
    fn transpile(&self) -> Result<String> {
        let cosy_lib = std::fs::read_to_string("assets/cosy_lib/cosy.cpp")
        .context("Failed to read cosy_lib.cpp!")?;
        let mut output = cosy_lib + "\n\n\n\n/// Automatically generated\n";

        // Transpile the AST to C++
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
    fn transpile(&self) -> Result<String> {
        let res = match self {
            Expr::Number(n) => {
                format!("Cosy({})", n)
            },
            Expr::Var(name) => name.to_string(),
            Expr::Exp { expr } => {
                let sub_expr: String = (*expr).transpile()
                    .context("Failed to convert sub-expression to string!")?;
                format!("Cosy(e^({}))", sub_expr)
            },
            Expr::Add { left, right } => {
                let left_str: String = (*left).transpile()
                    .context("Failed to convert left expression to string!")?;
                let right_str: String = (*right).transpile()
                    .context("Failed to convert right expression to string!")?;
                format!("({} + {})", left_str, right_str)
            }
        };

        Ok(res)
    }
}
impl Transpile for Statement {
    fn transpile( &self ) -> Result<String> {
        match self {
            Statement::VarDecl { name, .. } => {
                Ok(format!("Cosy {};", name))
            },
            Statement::Write { exprs } => {
                let mut exprs_sts = Vec::new();
                for expr in exprs {
                    let expr_st: String = expr.transpile()
                        .context("Failed to convert expression to string!")?;
                    exprs_sts.push(expr_st);
                }

                Ok(format!("std::cout << {} << std::endl;", exprs_sts.join(" << ")))
            }
            Statement::Assign { name, value } => {
                let value_st: String = value.transpile()
                    .context("Failed to convert value expression to string!")?;
                Ok(format!("{} = {};", name, value_st))
            },
            Statement::Procedure {
                name,
                args,
                body
            } => {
                let fn_type = if name == "RUN" { "int" } else { "void" };
                let fn_name = if name == "RUN" { "main" } else { &name };

                let mut body_sts = Vec::new();
                for stmt in body {
                    let mut stmt_st: String = stmt.transpile()
                        .context("Failed to convert statement to string!")?;
                    stmt_st.insert(0, '\t'); // Indent the body statements
                    body_sts.push(stmt_st);
                }

                Ok(format!("{} {}({}) {{\n{}\n}}", fn_type, fn_name, args.join(", "), body_sts.join("\n")))
            },
            Statement::ProcedureCall { name, args } => {
                let mut arg_strs = Vec::new();
                for arg in args {
                    let arg_st: String = arg.transpile()
                        .context("Failed to convert argument expression to string!")?;
                    arg_strs.push(arg_st);
                }
                
                Ok(format!("{}({});", name, arg_strs.join(", ")))
            }
        }
    }
}