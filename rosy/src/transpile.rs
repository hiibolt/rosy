//! # Transpilation Engine
//!
//! Core traits and context types for converting the Rosy AST into Rust source code.
//!
//! ## Key Traits
//!
//! | Trait | Purpose |
//! |-------|---------|
//! | [`Transpile`] | Converts an AST node to a Rust code string |
//! | [`TranspileableStatement`] | Represents a statement node that can be transpiled |
//! | [`TranspileableExpr`] | Represents an expression node that can be transpiled |
//!
//! ## Context
//!
//! [`TranspilationInputContext`] tracks variable scope, function/procedure
//! signatures, and closure-captured variables during transpilation.
//!
//! ## Error Handling
//!
//! Transpilation returns `Result<TranspilationOutput, Vec<Error>>` to
//! accumulate multiple errors before failing. Use `.context()` to add
//! breadcrumbs for error diagnostics.

use crate::{
    program::{expressions::Expr, statements::SourceLocation},
    resolve::{ExprRecipe, ScopeContext, TypeResolver, TypeSlot},
    rosy_lib::RosyType,
};
use anyhow::{Error, Result};
use std::collections::{BTreeSet, HashMap, HashSet};

pub enum TypeslotDeclarationResult {
    VarFuncOrProcedureDecl { result: Result<()> },
    NotAVarFuncOrProcedureDecl,
}

pub enum InferenceEdgeResult {
    HasEdges { result: Result<()> },
    NoEdges,
}

pub enum TypeHydrationResult {
    Hydrated { result: Result<()> },
    NothingToHydrate,
}

pub enum ExprFunctionCallResult {
    HasFunctionCalls { result: Result<()> },
    NoFunctionCalls,
}

pub enum ConcatExtensionResult {
    Extended,
    NotAConcatExpr,
}

pub trait TranspileableStatement: Transpile + Send + Sync {
    fn register_typeslot_declaration(
        &self,
        _resolver: &mut TypeResolver,
        _ctx: &mut ScopeContext,
        _source_location: SourceLocation,
    ) -> TypeslotDeclarationResult;
    fn wire_inference_edges(
        &self,
        _resolver: &mut TypeResolver,
        _ctx: &mut ScopeContext,
        _source_location: SourceLocation,
    ) -> InferenceEdgeResult;
    fn hydrate_resolved_types(
        &mut self,
        _resolver: &TypeResolver,
        _current_scope: &[String],
    ) -> TypeHydrationResult;
}
pub trait TranspileableExpr: Transpile + Send + Sync {
    fn type_of(&self, context: &TranspilationInputContext) -> Result<RosyType>;
    fn discover_expr_function_calls(
        &self,
        resolver: &mut TypeResolver,
        ctx: &ScopeContext,
    ) -> ExprFunctionCallResult;
    fn build_expr_recipe(
        &self,
        resolver: &TypeResolver,
        ctx: &ScopeContext,
        deps: &mut HashSet<TypeSlot>,
    ) -> ExprRecipe;
    fn extend_concat(&mut self, right: Expr) -> ConcatExtensionResult;
}
pub trait Transpile: std::fmt::Debug {
    fn transpile(
        &self,
        context: &mut TranspilationInputContext,
    ) -> Result<TranspilationOutput, Vec<Error>>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableScope {
    Local,
    Arg,
    Higher,
}
#[derive(Debug, Clone)]
pub struct VariableData {
    pub name: String,
    pub r#type: RosyType,
}
#[derive(Debug, Clone)]
pub struct ScopedVariableData {
    pub scope: VariableScope,
    pub data: VariableData,
}
#[derive(Debug, Clone)]
pub struct TranspilationInputFunctionContext {
    pub return_type: RosyType,
    pub args: Vec<VariableData>,
    pub requested_variables: BTreeSet<String>,
}
#[derive(Debug, Clone)]
pub struct TranspilationInputProcedureContext {
    pub args: Vec<VariableData>,
    pub requested_variables: BTreeSet<String>,
}
#[derive(Default, Clone)]
pub struct TranspilationInputContext {
    pub variables: HashMap<String, ScopedVariableData>,
    pub functions: HashMap<String, TranspilationInputFunctionContext>,
    pub procedures: HashMap<String, TranspilationInputProcedureContext>,
    pub in_loop: bool,
}
/// Whether an expression produces an owned value or a reference.
///
/// This drives how consumers wrap the expression:
/// - Owned + needs ref → `&expr`
/// - Ref + needs owned (Copy) → `expr` (auto-deref)
/// - Ref + needs owned (non-Copy) → `expr.clone()`
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ValueKind {
    /// A fresh value: literals, operator results, function returns.
    /// Can be moved into assignment without cloning.
    #[default]
    Owned,
    /// A reference to an existing variable (`&X` or `X` where X: &T).
    /// Must be cloned to own for non-Copy types.
    Ref,
}

#[derive(Default)]
pub struct TranspilationOutput {
    pub serialization: String,
    pub requested_variables: BTreeSet<String>,
    pub value_kind: ValueKind,
}

impl TranspilationOutput {
    /// Get this expression as an owned value for assignment or value context.
    /// - Owned → expr (move/copy)
    /// - Ref(&X) + Copy → X (strip & to get bare name)
    /// - Ref(X) + Copy → (*X) (deref &mut T)
    /// - Ref(&X) + non-Copy → X.clone()
    /// - Ref(X) + non-Copy → (*X).clone()
    pub fn as_owned(&self, ty: &RosyType) -> String {
        match self.value_kind {
            ValueKind::Owned => self.serialization.clone(),
            ValueKind::Ref => {
                if let Some(inner) = self.serialization.strip_prefix('&') {
                    if ty.is_copy() {
                        inner.to_string()
                    } else {
                        format!("{inner}.clone()")
                    }
                } else if ty.is_copy() {
                    format!("(*{})", self.serialization)
                } else {
                    format!("(*{}).clone()", self.serialization)
                }
            }
        }
    }

    /// Get this expression as a shared reference for trait method arguments.
    /// - Owned → &expr
    /// - Ref(&X) → &X (already a shared reference)
    /// - Ref(X) → &*X (deref &mut T to get &T)
    pub fn as_ref(&self) -> String {
        match self.value_kind {
            ValueKind::Owned => format!("&{}", self.serialization),
            ValueKind::Ref => {
                if self.serialization.starts_with('&') {
                    self.serialization.clone()
                } else {
                    format!("&*{}", self.serialization)
                }
            }
        }
    }

    /// Get this expression as a mutable reference for function/procedure arguments.
    /// - Owned → &mut expr
    /// - Ref(&X) → &mut X (strip & and add &mut)
    /// - Ref(X) → &mut *X (deref &mut T to get &mut T)
    pub fn as_mut_ref(&self) -> String {
        match self.value_kind {
            ValueKind::Owned => format!("&mut {}", self.serialization),
            ValueKind::Ref => {
                if let Some(inner) = self.serialization.strip_prefix('&') {
                    format!("&mut {}", inner)
                } else {
                    format!("&mut *{}", self.serialization)
                }
            }
        }
    }

    /// Get this expression as a plain value (for Copy-type arithmetic, conditions).
    /// - Owned → expr
    /// - Ref(&X) → X (strip & to deref)
    /// - Ref(X) → (*X) (deref &mut T)
    pub fn as_value(&self) -> String {
        match self.value_kind {
            ValueKind::Owned => self.serialization.clone(),
            ValueKind::Ref => {
                if let Some(inner) = self.serialization.strip_prefix('&') {
                    inner.to_string()
                } else {
                    format!("(*{})", self.serialization)
                }
            }
        }
    }
}

// helper for indenting blocks
pub fn indent(st: String) -> String {
    st.lines()
        .map(|line| format!("\t{}", line))
        .collect::<Vec<String>>()
        .join("\n")
}
// helper for adding context to a vec of  errors
pub fn add_context_to_all(arr: Vec<Error>, context: String) -> Vec<Error> {
    arr.into_iter()
        .map(|err| err.context(context.clone()))
        .collect()
}
