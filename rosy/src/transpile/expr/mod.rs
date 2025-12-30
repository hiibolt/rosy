mod var_expr;
mod concat;
mod complex;
mod extract;
mod add;
mod sub;
mod mult;
mod div;
mod string_convert;
mod logical_convert;
mod function_call;
mod da;
mod length;
mod sin;

use crate::ast::*;
use super::{Transpile, TranspilationInputContext, TranspilationOutput};
use std::collections::BTreeSet;
use anyhow::{Result, Error};


impl Transpile for Expr {
    fn transpile (
        &self, context: &mut TranspilationInputContext
    ) -> Result<TranspilationOutput, Vec<Error>> {
        match self {
            Expr::Number(n) => Ok(TranspilationOutput {
                serialization: format!("&mut ({n}f64)"),
                requested_variables: BTreeSet::new(),
            }),
            Expr::String(s) => Ok(TranspilationOutput {
                serialization: format!("&mut \"{}\".to_string()", s.replace('"', "\\\"")),
                requested_variables: BTreeSet::new(),
            }),
            Expr::Boolean(b) => Ok(TranspilationOutput {
                serialization: format!("&mut {}", if *b { "true" } else { "false" }),
                requested_variables: BTreeSet::new(),
            }),
            Expr::Var(var_expr) => var_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context(format!(
                        "...while transpiling variable expression for variable '{}'", var_expr.identifier.name
                    ))
                }).collect::<Vec<Error>>()),
            Expr::Add(add_expr) => add_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling addition expression")
                }).collect::<Vec<Error>>()),
            Expr::Sub(sub_expr) => sub_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling subtraction expression")
                }).collect::<Vec<Error>>()),
            Expr::Mult(mult_expr) => mult_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling multiplication expression")
                }).collect::<Vec<Error>>()),
            Expr::Div(div_expr) => div_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling division expression")
                }).collect::<Vec<Error>>()),
            Expr::StringConvert(string_convert_expr) => string_convert_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling string conversion expression")
                }).collect::<Vec<Error>>()),
            Expr::LogicalConvert(logical_convert_expr) => logical_convert_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling logical conversion expression")
                }).collect::<Vec<Error>>()),
            Expr::FunctionCall(function_call_expr) => function_call_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling function call expression")
                }).collect::<Vec<Error>>()),
            Expr::Concat(concat_expr) => concat_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling concatenation expression")
                }).collect::<Vec<Error>>()),
            Expr::Extract(extract_expr) => extract_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling extract expression")
                }).collect::<Vec<Error>>()),
            Expr::Complex(complex_expr) => complex_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling complex expression")
                }).collect::<Vec<Error>>()),
            Expr::DA(da_expr) => da_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling DA expression")
                }).collect::<Vec<Error>>()),
            Expr::Length(length_expr) => length_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling LENGTH expression")
                }).collect::<Vec<Error>>()),
            Expr::Sin(sin_expr) => sin_expr.transpile(context)
                .map_err(|e| e.into_iter().map(|err| {
                    err.context("...while transpiling SIN expression")
                }).collect::<Vec<Error>>()),
        }
    }
}