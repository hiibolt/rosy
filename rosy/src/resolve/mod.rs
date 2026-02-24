/// Type Resolution Module
/// 
/// This module implements a constraint-based type inference system for Rosy.
/// It runs as a pass between AST construction and transpilation, filling in
/// any `Option<RosyType>` fields that were left as `None` during parsing.
///
/// The approach:
///   1. Walk the AST to discover all "type slots" (variables, function args,
///      function return types, procedure args)
///   2. Collect constraints from assignments, operator usage, and call sites
///   3. Propagate known types through the constraint graph
///   4. Error if any slots remain unresolved after propagation

use std::collections::HashMap;
use anyhow::{anyhow, Result};
use crate::rosy_lib::RosyType;
use crate::program::Program;
use crate::program::statements::*;
use crate::program::expressions::*;
use crate::program::expressions::function_call as expr_function_call;

/// Represents an unresolved type slot with context about what the resolver tried.
#[derive(Debug)]
struct UnresolvedSlot {
    slot: TypeSlot,
    hints: Vec<String>,
}

impl std::fmt::Display for UnresolvedSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ✗ Could not determine the type of {}", self.slot)?;
        if self.hints.is_empty() {
            write!(f, "\n    No assignments or usages were found to infer the type from.")?;
        } else {
            for hint in &self.hints {
                write!(f, "\n    • {}", hint)?;
            }
        }
        // Suggestion based on slot kind
        match &self.slot {
            TypeSlot::Variable(_, name) => {
                write!(f, "\n    → Add an explicit type: VARIABLE (RE) {} ;", name)?;
            }
            TypeSlot::FunctionReturn(_, name) => {
                write!(f, "\n    → Add an explicit return type: FUNCTION (RE) {} ... ;", name)?;
            }
            TypeSlot::Argument(_, callable, arg) => {
                write!(f, "\n    → Add an explicit type to argument '{}' in '{}': {} (RE)", arg, callable, arg)?;
            }
        }
        Ok(())
    }
}

/// A unique identifier for a type slot in the constraint graph.
/// Scoped by an optional function/procedure name to handle nested scopes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeSlot {
    /// A variable declaration: (scope_path, variable_name)
    Variable(Vec<String>, String),
    /// A function return type: (scope_path, function_name)
    FunctionReturn(Vec<String>, String),
    /// A function/procedure argument: (scope_path, callable_name, arg_name)
    Argument(Vec<String>, String, String),
}

impl std::fmt::Display for TypeSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeSlot::Variable(scope, name) => {
                if scope.is_empty() {
                    write!(f, "variable '{}'", name)
                } else {
                    write!(f, "variable '{}' (in {})", name, scope.join(" > "))
                }
            }
            TypeSlot::FunctionReturn(scope, name) => {
                if scope.is_empty() {
                    write!(f, "return type of function '{}'", name)
                } else {
                    write!(f, "return type of function '{}' (in {})", name, scope.join(" > "))
                }
            }
            TypeSlot::Argument(scope, callable, arg) => {
                if scope.is_empty() {
                    write!(f, "argument '{}' of '{}'", arg, callable)
                } else {
                    write!(f, "argument '{}' of '{}' (in {})", arg, callable, scope.join(" > "))
                }
            }
        }
    }
}

/// A constraint saying "this slot must have this type".
#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub slot: TypeSlot,
    pub resolved_type: RosyType,
    pub reason: String,
}

/// Context for tracking known types during resolution.
/// Maps variable/arg names to their resolved types within a scope.
#[derive(Debug, Clone, Default)]
struct ScopeContext {
    /// Current scope path (e.g., ["RUN", "INNER_FUNC"])
    scope_path: Vec<String>,
    /// Known variable types in the current scope (name → type)
    variables: HashMap<String, RosyType>,
    /// Known function signatures (name → (return_type, arg_types))
    functions: HashMap<String, (Option<RosyType>, Vec<(String, Option<RosyType>)>)>,
    /// Known procedure signatures (name → arg_types)
    procedures: HashMap<String, Vec<(String, Option<RosyType>)>>,
}

/// The main type resolver.
pub struct TypeResolver {
    constraints: Vec<TypeConstraint>,
}

impl TypeResolver {
    pub fn new() -> Self {
        TypeResolver {
            constraints: Vec::new(),
        }
    }

    /// Run type resolution on a program. This mutates the AST in place,
    /// filling in all `None` type fields.
    ///
    /// Uses iterative constraint collection: each round applies what we know,
    /// then re-collects constraints from the (now partially-typed) AST.
    /// Stops when no new constraints are discovered (fixed point).
    pub fn resolve(program: &mut Program) -> Result<()> {
        const MAX_ITERATIONS: usize = 10;

        for iteration in 0..MAX_ITERATIONS {
            let mut resolver = TypeResolver::new();
            let mut ctx = ScopeContext::default();

            // Collect constraints from the current state of the AST
            resolver.collect_from_statements(&program.statements, &mut ctx)?;

            let num_constraints = resolver.constraints.len();

            // Apply constraints to the AST
            resolver.apply_to_statements(&mut program.statements, &ctx.scope_path)?;

            // Check if we've resolved everything
            let unresolved = resolver.validate_all_types_resolved(&program.statements, &[]);
            if unresolved.is_empty() {
                return Ok(()); // All types resolved!
            }

            // If no new constraints were found this round, we've hit the fixed point
            // and can't make further progress
            if num_constraints == 0 && iteration > 0 {
                break;
            }

            // Check if we made progress by seeing if there are still unresolved
            // slots — if constraint count didn't change, we're stuck
            if iteration > 0 {
                let prev_unresolved_count = resolver.validate_all_types_resolved(&program.statements, &[]).len();
                if prev_unresolved_count == unresolved.len() && num_constraints == 0 {
                    break;
                }
            }
        }

        // Final validation — produce error message for remaining unresolved types
        let resolver = TypeResolver::new();
        let unresolved = resolver.validate_all_types_resolved(&program.statements, &[]);
        if !unresolved.is_empty() {
            // Collect all constraints one more time for hint generation
            let mut hint_resolver = TypeResolver::new();
            let mut ctx = ScopeContext::default();
            let _ = hint_resolver.collect_from_statements(&program.statements, &mut ctx);

            let mut msg = format!(
                "\n╭─ Type Resolution Failed ─────────────────────────────────\n│\n│  {} unresolved type{} found:\n│",
                unresolved.len(),
                if unresolved.len() == 1 { "" } else { "s" }
            );
            for slot in &unresolved {
                // Use hint_resolver to gather hints (it has constraints from latest pass)
                let hints = hint_resolver.gather_hints_for_slot(&slot.slot);
                let display_slot = UnresolvedSlot { slot: slot.slot.clone(), hints };
                for line in format!("{}", display_slot).lines() {
                    msg.push_str(&format!("\n│  {}", line));
                }
                msg.push_str("\n│");
            }
            msg.push_str("\n│  The type resolver infers types from assignments, call");
            msg.push_str("\n│  sites, and operator usage. If a variable is declared");
            msg.push_str("\n│  but never assigned a value with a known type, it can't");
            msg.push_str("\n│  be resolved automatically.");
            msg.push_str("\n│");
            msg.push_str("\n╰──────────────────────────────────────────────────────────");
            return Err(anyhow!("{}", msg));
        }

        Ok(())
    }

    /// Collect type constraints from a list of statements.
    fn collect_from_statements(
        &mut self,
        statements: &[Statement],
        ctx: &mut ScopeContext,
    ) -> Result<()> {
        // First pass: register all declarations (variables, functions, procedures)
        // so we know what exists before processing assignments and calls
        for stmt in statements {
            self.register_declaration(stmt, ctx)?;
        }

        // Second pass: collect constraints from assignments and call sites
        for stmt in statements {
            self.collect_constraints_from_statement(stmt, ctx)?;
        }

        Ok(())
    }

    /// Register a declaration (variable, function, procedure) in the scope context.
    fn register_declaration(
        &mut self,
        stmt: &Statement,
        ctx: &mut ScopeContext,
    ) -> Result<()> {
        match stmt.enum_variant {
            StatementEnum::VarDecl => {
                let var_decl = stmt.inner.as_any()
                    .downcast_ref::<VarDeclStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast VarDecl statement"))?;

                // If the type is already known, record it
                if let Some(t) = &var_decl.data.r#type {
                    ctx.variables.insert(var_decl.data.name.clone(), t.clone());
                    self.constraints.push(TypeConstraint {
                        slot: TypeSlot::Variable(ctx.scope_path.clone(), var_decl.data.name.clone()),
                        resolved_type: t.clone(),
                        reason: "explicitly declared type".to_string(),
                    });
                }
            }
            StatementEnum::Function => {
                let func = stmt.inner.as_any()
                    .downcast_ref::<FunctionStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Function statement"))?;

                // Register the function signature
                let arg_types: Vec<(String, Option<RosyType>)> = func.args.iter()
                    .map(|a| (a.name.clone(), a.r#type.clone()))
                    .collect();
                ctx.functions.insert(func.name.clone(), (func.return_type.clone(), arg_types.clone()));

                // If return type is known, add constraint
                if let Some(t) = &func.return_type {
                    self.constraints.push(TypeConstraint {
                        slot: TypeSlot::FunctionReturn(ctx.scope_path.clone(), func.name.clone()),
                        resolved_type: t.clone(),
                        reason: "explicitly declared return type".to_string(),
                    });
                    // Also register the implicit return variable
                    ctx.variables.insert(func.name.clone(), t.clone());
                }

                // Register arg constraints if known
                for arg in &func.args {
                    if let Some(t) = &arg.r#type {
                        self.constraints.push(TypeConstraint {
                            slot: TypeSlot::Argument(ctx.scope_path.clone(), func.name.clone(), arg.name.clone()),
                            resolved_type: t.clone(),
                            reason: "explicitly declared argument type".to_string(),
                        });
                    }
                }

                // Recurse into the function body with a new scope
                let mut inner_ctx = ctx.clone();
                inner_ctx.scope_path.push(func.name.clone());
                // Add args to inner scope
                for arg in &func.args {
                    if let Some(t) = &arg.r#type {
                        inner_ctx.variables.insert(arg.name.clone(), t.clone());
                    }
                }
                // Add return variable to inner scope
                if let Some(t) = &func.return_type {
                    inner_ctx.variables.insert(func.name.clone(), t.clone());
                }
                self.collect_from_statements(&func.body, &mut inner_ctx)?;

                // Propagate any constraints discovered inside back to the function signature
                // (e.g., if we learned arg types from usage inside the body)
                let inferred_return: Option<RosyType> = if func.return_type.is_none() {
                    self.constraints.iter()
                        .find_map(|c| match &c.slot {
                            TypeSlot::Variable(scope, name)
                                if scope == &inner_ctx.scope_path && name == &func.name =>
                            {
                                Some(c.resolved_type.clone())
                            }
                            _ => None,
                        })
                } else {
                    None
                };
                if let Some(ret_type) = inferred_return {
                    self.constraints.push(TypeConstraint {
                        slot: TypeSlot::FunctionReturn(ctx.scope_path.clone(), func.name.clone()),
                        resolved_type: ret_type,
                        reason: format!("inferred from assignment to return variable '{}'", func.name),
                    });
                }
            }
            StatementEnum::Procedure => {
                let proc = stmt.inner.as_any()
                    .downcast_ref::<ProcedureStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Procedure statement"))?;

                // Register the procedure signature
                let arg_types: Vec<(String, Option<RosyType>)> = proc.args.iter()
                    .map(|a| (a.name.clone(), a.r#type.clone()))
                    .collect();
                ctx.procedures.insert(proc.name.clone(), arg_types);

                // Register arg constraints if known
                for arg in &proc.args {
                    if let Some(t) = &arg.r#type {
                        self.constraints.push(TypeConstraint {
                            slot: TypeSlot::Argument(ctx.scope_path.clone(), proc.name.clone(), arg.name.clone()),
                            resolved_type: t.clone(),
                            reason: "explicitly declared argument type".to_string(),
                        });
                    }
                }

                // Recurse into the procedure body with a new scope
                let mut inner_ctx = ctx.clone();
                inner_ctx.scope_path.push(proc.name.clone());
                for arg in &proc.args {
                    if let Some(t) = &arg.r#type {
                        inner_ctx.variables.insert(arg.name.clone(), t.clone());
                    }
                }
                self.collect_from_statements(&proc.body, &mut inner_ctx)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Collect constraints from a single statement's usage patterns.
    fn collect_constraints_from_statement(
        &mut self,
        stmt: &Statement,
        ctx: &mut ScopeContext,
    ) -> Result<()> {
        match stmt.enum_variant {
            StatementEnum::Assign => {
                let assign = stmt.inner.as_any()
                    .downcast_ref::<AssignStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Assign statement"))?;

                // Try to determine the type of the RHS expression
                if let Some(rhs_type) = self.try_resolve_expr_type(&assign.value, ctx) {
                    let var_name = &assign.identifier.name;

                    // If the variable doesn't have a known type, add a constraint
                    if !ctx.variables.contains_key(var_name) {
                        ctx.variables.insert(var_name.clone(), rhs_type.clone());
                        self.constraints.push(TypeConstraint {
                            slot: TypeSlot::Variable(ctx.scope_path.clone(), var_name.clone()),
                            resolved_type: rhs_type,
                            reason: "inferred from assignment".to_string(),
                        });
                    }
                }
            }
            StatementEnum::ProcedureCall => {
                let call = stmt.inner.as_any()
                    .downcast_ref::<ProcedureCallStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast ProcedureCall statement"))?;

                self.collect_constraints_from_call_site(
                    &call.name, &call.args, false, ctx
                )?;
            }
            StatementEnum::FunctionCall => {
                let call = stmt.inner.as_any()
                    .downcast_ref::<FunctionCallStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast FunctionCall statement"))?;

                self.collect_constraints_from_call_site(
                    &call.name, &call.args, true, ctx
                )?;
            }
            StatementEnum::If => {
                let if_stmt = stmt.inner.as_any()
                    .downcast_ref::<IfStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast If statement"))?;

                // Recurse into each branch
                self.collect_from_statements(&if_stmt.then_body, &mut ctx.clone())?;
                for elseif in &if_stmt.elseif_clauses {
                    self.collect_from_statements(&elseif.body, &mut ctx.clone())?;
                }
                if let Some(else_body) = &if_stmt.else_body {
                    self.collect_from_statements(else_body, &mut ctx.clone())?;
                }
            }
            StatementEnum::Loop => {
                let loop_stmt = stmt.inner.as_any()
                    .downcast_ref::<LoopStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Loop statement"))?;

                // The loop iterator is always RE
                let mut inner_ctx = ctx.clone();
                inner_ctx.variables.insert(loop_stmt.iterator.clone(), RosyType::RE());
                self.collect_from_statements(&loop_stmt.body, &mut inner_ctx)?;
            }
            StatementEnum::WhileLoop => {
                let while_stmt = stmt.inner.as_any()
                    .downcast_ref::<WhileStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast While statement"))?;

                self.collect_from_statements(&while_stmt.body, &mut ctx.clone())?;
            }
            StatementEnum::PLoop => {
                let ploop_stmt = stmt.inner.as_any()
                    .downcast_ref::<PLoopStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast PLoop statement"))?;

                let mut inner_ctx = ctx.clone();
                inner_ctx.variables.insert(ploop_stmt.iterator.clone(), RosyType::RE());
                self.collect_from_statements(&ploop_stmt.body, &mut inner_ctx)?;
            }
            StatementEnum::Fit => {
                let fit_stmt = stmt.inner.as_any()
                    .downcast_ref::<FitStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Fit statement"))?;

                self.collect_from_statements(&fit_stmt.body, &mut ctx.clone())?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Collect constraints from a function/procedure call site.
    /// If the callee has untyped parameters, we can infer their types from the arguments.
    fn collect_constraints_from_call_site(
        &mut self,
        name: &str,
        args: &[Expr],
        is_function: bool,
        ctx: &mut ScopeContext,
    ) -> Result<()> {
        let param_info: Option<Vec<(String, Option<RosyType>)>> = if is_function {
            ctx.functions.get(name).map(|(_, params)| params.clone())
        } else {
            ctx.procedures.get(name).cloned()
        };

        if let Some(params) = param_info {
            for (i, arg_expr) in args.iter().enumerate() {
                if let Some((param_name, param_type)) = params.get(i) {
                    // If the parameter doesn't have a type yet, try to infer from the argument
                    if param_type.is_none() {
                        if let Some(arg_type) = self.try_resolve_expr_type(arg_expr, ctx) {
                            self.constraints.push(TypeConstraint {
                                slot: TypeSlot::Argument(
                                    ctx.scope_path.clone(),
                                    name.to_string(),
                                    param_name.clone(),
                                ),
                                resolved_type: arg_type,
                                reason: format!("inferred from call site argument {}", i + 1),
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Try to determine the type of an expression from what we already know.
    /// Returns None if the type can't be determined yet.
    fn try_resolve_expr_type(
        &self,
        expr: &Expr,
        ctx: &ScopeContext,
    ) -> Option<RosyType> {
        match &expr.enum_variant {
            ExprEnum::Number => Some(RosyType::RE()),
            ExprEnum::String => Some(RosyType::ST()),
            ExprEnum::Boolean => Some(RosyType::LO()),
            ExprEnum::Var => {
                // Look up the variable in the scope context
                let var_expr = expr.inner.as_any()
                    .downcast_ref::<var_expr::VarExpr>()?;
                let ident = &var_expr.identifier;
                let base_type = ctx.variables.get(&ident.name)?;
                // Account for indexing reducing dimensions
                let mut result_type = base_type.clone();
                result_type.dimensions = result_type.dimensions.checked_sub(ident.indicies.len())?;
                Some(result_type)
            }
            ExprEnum::FunctionCall => {
                let func_call = expr.inner.as_any()
                    .downcast_ref::<expr_function_call::FunctionCallExpr>()?;
                // Look up the function return type
                let (return_type, _) = ctx.functions.get(&func_call.name)?;
                return_type.clone()
            }
            ExprEnum::Add => {
                let add_expr = expr.inner.as_any()
                    .downcast_ref::<add::AddExpr>()?;
                let left_type = self.try_resolve_expr_type(&add_expr.left, ctx)?;
                let right_type = self.try_resolve_expr_type(&add_expr.right, ctx)?;
                crate::rosy_lib::operators::add::get_return_type(&left_type, &right_type)
            }
            ExprEnum::Sub => {
                let sub_expr = expr.inner.as_any()
                    .downcast_ref::<sub::SubExpr>()?;
                let left_type = self.try_resolve_expr_type(&sub_expr.left, ctx)?;
                let right_type = self.try_resolve_expr_type(&sub_expr.right, ctx)?;
                crate::rosy_lib::operators::sub::get_return_type(&left_type, &right_type)
            }
            ExprEnum::Mult => {
                let mult_expr = expr.inner.as_any()
                    .downcast_ref::<mult::MultExpr>()?;
                let left_type = self.try_resolve_expr_type(&mult_expr.left, ctx)?;
                let right_type = self.try_resolve_expr_type(&mult_expr.right, ctx)?;
                crate::rosy_lib::operators::mult::get_return_type(&left_type, &right_type)
            }
            ExprEnum::Div => {
                let div_expr = expr.inner.as_any()
                    .downcast_ref::<div::DivExpr>()?;
                let left_type = self.try_resolve_expr_type(&div_expr.left, ctx)?;
                let right_type = self.try_resolve_expr_type(&div_expr.right, ctx)?;
                crate::rosy_lib::operators::div::get_return_type(&left_type, &right_type)
            }
            ExprEnum::Concat => {
                let concat_expr = expr.inner.as_any()
                    .downcast_ref::<concat::ConcatExpr>()?;
                // Concat is n-ary — fold left to right through the terms
                let mut iter = concat_expr.terms.iter();
                let first = iter.next()?;
                let mut result_type = self.try_resolve_expr_type(first, ctx)?;
                for term in iter {
                    let term_type = self.try_resolve_expr_type(term, ctx)?;
                    result_type = crate::rosy_lib::operators::concat::get_return_type(&result_type, &term_type)?;
                }
                Some(result_type)
            }
            ExprEnum::Extract => {
                let extract_expr = expr.inner.as_any()
                    .downcast_ref::<extract::ExtractExpr>()?;
                let object_type = self.try_resolve_expr_type(&extract_expr.object, ctx)?;
                let index_type = self.try_resolve_expr_type(&extract_expr.index, ctx)?;
                crate::rosy_lib::operators::extract::get_return_type(&object_type, &index_type)
            }
            ExprEnum::Eq | ExprEnum::Neq | ExprEnum::Lt | ExprEnum::Gt |
            ExprEnum::Lte | ExprEnum::Gte => {
                // Comparison operators always return LO
                Some(RosyType::LO())
            }
            ExprEnum::Not => {
                // NOT always returns LO
                Some(RosyType::LO())
            }
            ExprEnum::Complex => Some(RosyType::CM()),
            ExprEnum::StringConvert => Some(RosyType::ST()),
            ExprEnum::LogicalConvert => Some(RosyType::LO()),
            ExprEnum::DA => Some(RosyType::DA()),
            ExprEnum::Length => Some(RosyType::RE()),
            ExprEnum::Sin => {
                // SIN return type depends on input type
                let sin_expr = expr.inner.as_any()
                    .downcast_ref::<sin::SinExpr>()?;
                let input_type = self.try_resolve_expr_type(&sin_expr.expr, ctx)?;
                crate::rosy_lib::intrinsics::sin::get_return_type(&input_type)
            }
        }
    }

    /// Gather hints about what the resolver knows regarding a particular slot.
    fn gather_hints_for_slot(&self, slot: &TypeSlot) -> Vec<String> {
        let mut hints = Vec::new();

        // Check if there are any constraints that were collected for this slot
        // (there shouldn't be, since it's unresolved, but partial info is useful)
        let partial_constraints: Vec<&TypeConstraint> = self.constraints.iter()
            .filter(|c| &c.slot == slot)
            .collect();

        if !partial_constraints.is_empty() {
            // This shouldn't really happen since we applied successfully,
            // but just in case:
            for c in &partial_constraints {
                hints.push(format!("Found constraint: {} ({})", c.resolved_type, c.reason));
            }
        }

        // Give contextual guidance based on slot type
        match slot {
            TypeSlot::Variable(scope, name) => {
                if scope.is_empty() {
                    hints.push(format!(
                        "Variable '{}' is declared at global scope but is never assigned \
                         a value whose type could be determined.", name
                    ));
                } else {
                    hints.push(format!(
                        "Variable '{}' in '{}' is never assigned a value whose type \
                         could be determined.", name, scope.join(" > ")
                    ));
                }
                hints.push(
                    "Try assigning it a value (e.g. X := 0;) or adding an explicit type.".to_string()
                );
            }
            TypeSlot::FunctionReturn(_, name) => {
                hints.push(format!(
                    "Function '{}' has no explicit return type and the resolver \
                     could not determine it from the function body.", name
                ));
                hints.push(
                    "Try adding a return type to the FUNCTION declaration.".to_string()
                );
            }
            TypeSlot::Argument(_, callable, arg) => {
                hints.push(format!(
                    "Argument '{}' of '{}' has no type annotation and is never \
                     called with a value whose type could be determined.", arg, callable
                ));
                hints.push(
                    "Try adding a type after the argument name, or call the function/procedure \
                     with typed arguments.".to_string()
                );
            }
        }

        hints
    }

    /// Walk the AST after constraint application to find any remaining `None` types.
    /// Returns a list of unresolved slots with helpful context.
    fn validate_all_types_resolved(
        &self,
        statements: &[Statement],
        current_scope: &[String],
    ) -> Vec<UnresolvedSlot> {
        let mut unresolved = Vec::new();

        for stmt in statements {
            match stmt.enum_variant {
                StatementEnum::VarDecl => {
                    if let Some(var_decl) = stmt.inner.as_any()
                        .downcast_ref::<VarDeclStatement>()
                    {
                        if var_decl.data.r#type.is_none() {
                            let slot = TypeSlot::Variable(
                                current_scope.to_vec(),
                                var_decl.data.name.clone(),
                            );
                            let hints = self.gather_hints_for_slot(&slot);
                            unresolved.push(UnresolvedSlot { slot, hints });
                        }
                    }
                }
                StatementEnum::Function => {
                    if let Some(func) = stmt.inner.as_any()
                        .downcast_ref::<FunctionStatement>()
                    {
                        if func.return_type.is_none() {
                            let slot = TypeSlot::FunctionReturn(
                                current_scope.to_vec(),
                                func.name.clone(),
                            );
                            let hints = self.gather_hints_for_slot(&slot);
                            unresolved.push(UnresolvedSlot { slot, hints });
                        }
                        for arg in &func.args {
                            if arg.r#type.is_none() {
                                let slot = TypeSlot::Argument(
                                    current_scope.to_vec(),
                                    func.name.clone(),
                                    arg.name.clone(),
                                );
                                let hints = self.gather_hints_for_slot(&slot);
                                unresolved.push(UnresolvedSlot { slot, hints });
                            }
                        }
                        // Recurse into function body
                        let mut inner_scope = current_scope.to_vec();
                        inner_scope.push(func.name.clone());
                        unresolved.extend(
                            self.validate_all_types_resolved(&func.body, &inner_scope)
                        );
                    }
                }
                StatementEnum::Procedure => {
                    if let Some(proc) = stmt.inner.as_any()
                        .downcast_ref::<ProcedureStatement>()
                    {
                        for arg in &proc.args {
                            if arg.r#type.is_none() {
                                let slot = TypeSlot::Argument(
                                    current_scope.to_vec(),
                                    proc.name.clone(),
                                    arg.name.clone(),
                                );
                                let hints = self.gather_hints_for_slot(&slot);
                                unresolved.push(UnresolvedSlot { slot, hints });
                            }
                        }
                        // Recurse into procedure body
                        let mut inner_scope = current_scope.to_vec();
                        inner_scope.push(proc.name.clone());
                        unresolved.extend(
                            self.validate_all_types_resolved(&proc.body, &inner_scope)
                        );
                    }
                }
                StatementEnum::If => {
                    if let Some(if_stmt) = stmt.inner.as_any()
                        .downcast_ref::<IfStatement>()
                    {
                        unresolved.extend(
                            self.validate_all_types_resolved(&if_stmt.then_body, current_scope)
                        );
                        for elseif in &if_stmt.elseif_clauses {
                            unresolved.extend(
                                self.validate_all_types_resolved(&elseif.body, current_scope)
                            );
                        }
                        if let Some(else_body) = &if_stmt.else_body {
                            unresolved.extend(
                                self.validate_all_types_resolved(else_body, current_scope)
                            );
                        }
                    }
                }
                StatementEnum::Loop => {
                    if let Some(loop_stmt) = stmt.inner.as_any()
                        .downcast_ref::<LoopStatement>()
                    {
                        unresolved.extend(
                            self.validate_all_types_resolved(&loop_stmt.body, current_scope)
                        );
                    }
                }
                StatementEnum::WhileLoop => {
                    if let Some(while_stmt) = stmt.inner.as_any()
                        .downcast_ref::<WhileStatement>()
                    {
                        unresolved.extend(
                            self.validate_all_types_resolved(&while_stmt.body, current_scope)
                        );
                    }
                }
                StatementEnum::PLoop => {
                    if let Some(ploop_stmt) = stmt.inner.as_any()
                        .downcast_ref::<PLoopStatement>()
                    {
                        unresolved.extend(
                            self.validate_all_types_resolved(&ploop_stmt.body, current_scope)
                        );
                    }
                }
                StatementEnum::Fit => {
                    if let Some(fit_stmt) = stmt.inner.as_any()
                        .downcast_ref::<FitStatement>()
                    {
                        unresolved.extend(
                            self.validate_all_types_resolved(&fit_stmt.body, current_scope)
                        );
                    }
                }
                _ => {}
            }
        }

        unresolved
    }

    /// Apply collected constraints to the AST, filling in `None` type fields.
    fn apply_to_statements(
        &self,
        statements: &mut [Statement],
        current_scope: &[String],
    ) -> Result<()> {
        // Build a lookup map from constraints: (scope, name) → type
        let resolved: HashMap<(Vec<String>, String), RosyType> = {
            let mut map = HashMap::new();
            for constraint in &self.constraints {
                let key = match &constraint.slot {
                    TypeSlot::Variable(scope, name) => (scope.clone(), name.clone()),
                    TypeSlot::FunctionReturn(scope, name) => {
                        (scope.clone(), format!("__return__{}", name))
                    }
                    TypeSlot::Argument(scope, callable, arg) => {
                        (scope.clone(), format!("__arg__{}_{}", callable, arg))
                    }
                };

                // Check for conflicts
                if let Some(existing) = map.get(&key) {
                    if existing != &constraint.resolved_type {
                        return Err(anyhow!(
                            "Type conflict for {}: resolved as both '{}' and '{}'. \
                             Consider splitting into separate variables with explicit types.",
                            constraint.slot, existing, constraint.resolved_type
                        ));
                    }
                }

                map.insert(key, constraint.resolved_type.clone());
            }
            map
        };

        self.apply_to_statements_inner(statements, current_scope, &resolved)
    }

    fn apply_to_statements_inner(
        &self,
        statements: &mut [Statement],
        current_scope: &[String],
        resolved: &HashMap<(Vec<String>, String), RosyType>,
    ) -> Result<()> {
        for stmt in statements.iter_mut() {
            match stmt.enum_variant {
                StatementEnum::VarDecl => {
                    let var_decl = stmt.inner.as_any_mut()
                        .downcast_mut::<VarDeclStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast VarDecl statement for mutation"))?;

                    if var_decl.data.r#type.is_none() {
                        let key = (current_scope.to_vec(), var_decl.data.name.clone());
                        if let Some(resolved_type) = resolved.get(&key) {
                            var_decl.data.r#type = Some(resolved_type.clone());
                        }
                        // If still None, that's okay — transpilation will catch it
                        // with a helpful error message
                    }
                }
                StatementEnum::Function => {
                    let func = stmt.inner.as_any_mut()
                        .downcast_mut::<FunctionStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast Function statement for mutation"))?;

                    // Resolve return type
                    if func.return_type.is_none() {
                        let key = (current_scope.to_vec(), format!("__return__{}", func.name));
                        if let Some(resolved_type) = resolved.get(&key) {
                            func.return_type = Some(resolved_type.clone());
                        }
                    }

                    // Resolve argument types
                    for arg in &mut func.args {
                        if arg.r#type.is_none() {
                            let key = (current_scope.to_vec(), format!("__arg__{}_{}", func.name, arg.name));
                            if let Some(resolved_type) = resolved.get(&key) {
                                arg.r#type = Some(resolved_type.clone());
                            }
                        }
                    }

                    // Resolve the implicit return variable in the body (first statement)
                    // The function body's first stmt is a VarDecl for the return variable
                    if let Some(first_stmt) = func.body.first_mut() {
                        if let StatementEnum::VarDecl = first_stmt.enum_variant {
                            let var_decl = first_stmt.inner.as_any_mut()
                                .downcast_mut::<VarDeclStatement>()
                                .ok_or_else(|| anyhow!("Failed to downcast implicit return VarDecl"))?;
                            if var_decl.data.name == func.name && var_decl.data.r#type.is_none() {
                                var_decl.data.r#type = func.return_type.clone();
                            }
                        }
                    }

                    // Recurse into body
                    let mut inner_scope = current_scope.to_vec();
                    inner_scope.push(func.name.clone());
                    self.apply_to_statements_inner(&mut func.body, &inner_scope, resolved)?;
                }
                StatementEnum::Procedure => {
                    let proc = stmt.inner.as_any_mut()
                        .downcast_mut::<ProcedureStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast Procedure statement for mutation"))?;

                    // Resolve argument types
                    for arg in &mut proc.args {
                        if arg.r#type.is_none() {
                            let key = (current_scope.to_vec(), format!("__arg__{}_{}", proc.name, arg.name));
                            if let Some(resolved_type) = resolved.get(&key) {
                                arg.r#type = Some(resolved_type.clone());
                            }
                        }
                    }

                    // Recurse into body
                    let mut inner_scope = current_scope.to_vec();
                    inner_scope.push(proc.name.clone());
                    self.apply_to_statements_inner(&mut proc.body, &inner_scope, resolved)?;
                }
                StatementEnum::If => {
                    let if_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<IfStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast If statement for mutation"))?;

                    self.apply_to_statements_inner(&mut if_stmt.then_body, current_scope, resolved)?;
                    for elseif in &mut if_stmt.elseif_clauses {
                        self.apply_to_statements_inner(&mut elseif.body, current_scope, resolved)?;
                    }
                    if let Some(else_body) = &mut if_stmt.else_body {
                        self.apply_to_statements_inner(else_body, current_scope, resolved)?;
                    }
                }
                StatementEnum::Loop => {
                    let loop_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<LoopStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast Loop statement for mutation"))?;

                    self.apply_to_statements_inner(&mut loop_stmt.body, current_scope, resolved)?;
                }
                StatementEnum::WhileLoop => {
                    let while_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<WhileStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast While statement for mutation"))?;

                    self.apply_to_statements_inner(&mut while_stmt.body, current_scope, resolved)?;
                }
                StatementEnum::PLoop => {
                    let ploop_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<PLoopStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast PLoop statement for mutation"))?;

                    self.apply_to_statements_inner(&mut ploop_stmt.body, current_scope, resolved)?;
                }
                StatementEnum::Fit => {
                    let fit_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<FitStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast Fit statement for mutation"))?;

                    self.apply_to_statements_inner(&mut fit_stmt.body, current_scope, resolved)?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
