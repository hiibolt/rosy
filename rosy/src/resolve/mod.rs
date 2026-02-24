/// Type Resolution Module
///
/// This module implements a **dependency-graph-based** type inference system for Rosy.
/// It runs as a pass between AST construction and transpilation, filling in
/// any `Option<RosyType>` fields that were left as `None` during parsing.
///
/// The approach:
///   1. Walk the AST to discover all "type slots" (variables, function args,
///      function return types, procedure args). Slots with explicit types are
///      immediately resolved; others become unresolved nodes.
///   2. Build a dependency graph: for each unresolved slot, determine which
///      other slots must be resolved first (edges). E.g. `X := Y + Z` means
///      the slot for `X` depends on the slots for `Y` and `Z`.
///   3. Topologically sort the graph (Kahn's algorithm) and resolve slots
///      from leaves inward — each slot is resolved exactly once.
///   4. If unresolved slots remain, they form a cycle — report a clear error.

use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::{anyhow, Result};
use crate::rosy_lib::RosyType;
use crate::program::Program;
use crate::program::statements::*;
use crate::program::expressions::*;
use crate::program::expressions::function_call as expr_function_call;

// ─── Type Slot ──────────────────────────────────────────────────────────────

/// A unique identifier for a type slot in the dependency graph.
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

// ─── Resolution Rule ────────────────────────────────────────────────────────

/// Describes *how* to compute a slot's type once all its dependencies are resolved.
#[derive(Debug, Clone)]
enum ResolutionRule {
    /// The type is already known from an explicit annotation.
    Explicit(RosyType),
    /// Inferred from an assignment RHS or call-site argument expression.
    InferredFrom {
        recipe: ExprRecipe,
        reason: String,
    },
    /// Mirrors another slot exactly (e.g., return type from implicit return var).
    Mirror {
        source: TypeSlot,
        reason: String,
    },
    /// No rule has been established yet — the slot is truly unknown.
    /// Will remain unresolved and trigger an error if not replaced.
    Unresolved,
}

/// A lightweight "recipe" for computing the type of an expression.
/// Stores just enough info to re-derive the type once dependencies are resolved.
#[derive(Debug, Clone)]
enum ExprRecipe {
    /// A literal type — always known.
    Literal(RosyType),
    /// A variable reference — look up its slot.
    Variable(TypeSlot),
    /// A binary operator applied to two sub-recipes.
    BinaryOp { op: BinaryOpKind, left: Box<ExprRecipe>, right: Box<ExprRecipe> },
    /// An n-ary concat of sub-recipes.
    Concat(Vec<ExprRecipe>),
    /// A function call — result is the function's return type slot.
    FunctionCall(TypeSlot),
    /// SIN intrinsic — result depends on input type.
    Sin(Box<ExprRecipe>),
    /// Expression whose type couldn't be determined statically.
    Unknown,
}

#[derive(Debug, Clone, Copy)]
enum BinaryOpKind {
    Add, Sub, Mult, Div, Extract,
}

// ─── Dependency Graph Node ──────────────────────────────────────────────────

#[derive(Debug)]
struct GraphNode {
    slot: TypeSlot,
    /// How to compute this slot's type once dependencies are met.
    rule: ResolutionRule,
    /// Slots that this node depends on (must be resolved first).
    depends_on: HashSet<TypeSlot>,
    /// The resolved type (filled in during topological traversal).
    resolved: Option<RosyType>,
}

// ─── Scope Context (used during graph construction) ─────────────────────────

/// Tracks what's been declared so far in a scope during the discovery walk.
#[derive(Debug, Clone, Default)]
struct ScopeContext {
    scope_path: Vec<String>,
    /// Maps variable name → its TypeSlot.
    variables: HashMap<String, TypeSlot>,
    /// Maps function name → (return_type_slot, vec of (arg_name, arg_slot)).
    functions: HashMap<String, (TypeSlot, Vec<(String, TypeSlot)>)>,
    /// Maps procedure name → vec of (arg_name, arg_slot).
    procedures: HashMap<String, Vec<(String, TypeSlot)>>,
}

// ─── Type Resolver ──────────────────────────────────────────────────────────

pub struct TypeResolver {
    /// All nodes in the dependency graph, keyed by their slot.
    nodes: HashMap<TypeSlot, GraphNode>,
}

impl TypeResolver {
    pub fn new() -> Self {
        TypeResolver {
            nodes: HashMap::new(),
        }
    }

    // ─── Public entry point ─────────────────────────────────────────────

    /// Run type resolution on a program. Mutates the AST in place.
    pub fn resolve(program: &mut Program) -> Result<()> {
        let mut resolver = TypeResolver::new();
        let mut ctx = ScopeContext::default();

        // Phase 1: Walk AST, discover all slots and build dependency graph
        resolver.discover_slots(&program.statements, &mut ctx)?;

        // Phase 2: Topological sort + resolve
        resolver.topological_resolve()?;

        // Phase 3: Apply resolved types back to the AST
        resolver.apply_to_ast(&mut program.statements, &[])?;

        Ok(())
    }

    // ─── Phase 1: Discovery ─────────────────────────────────────────────

    /// Walk the AST, creating graph nodes for every type slot and recording
    /// their dependencies.
    fn discover_slots(
        &mut self,
        statements: &[Statement],
        ctx: &mut ScopeContext,
    ) -> Result<()> {
        // First pass: register all declarations so we know what exists
        for stmt in statements {
            self.register_declaration(stmt, ctx)?;
        }

        // Second pass: discover dependencies from assignments and call sites
        for stmt in statements {
            self.discover_dependencies(stmt, ctx)?;
        }

        Ok(())
    }

    /// Insert a node for a slot. If it has an explicit type, mark it resolved.
    fn insert_slot(&mut self, slot: TypeSlot, explicit_type: Option<&RosyType>) {
        if let Some(t) = explicit_type {
            self.nodes.insert(slot.clone(), GraphNode {
                slot,
                rule: ResolutionRule::Explicit(t.clone()),
                depends_on: HashSet::new(),
                resolved: Some(t.clone()),
            });
        } else {
            // Placeholder — rule and deps will be set by discover_dependencies
            self.nodes.entry(slot.clone()).or_insert_with(|| GraphNode {
                slot,
                rule: ResolutionRule::Unresolved,
                depends_on: HashSet::new(),
                resolved: None,
            });
        }
    }

    /// Register a declaration, creating graph nodes for its type slots.
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

                let slot = TypeSlot::Variable(
                    ctx.scope_path.clone(),
                    var_decl.data.name.clone(),
                );
                self.insert_slot(slot.clone(), var_decl.data.r#type.as_ref());
                ctx.variables.insert(var_decl.data.name.clone(), slot);
            }
            StatementEnum::Function => {
                let func = stmt.inner.as_any()
                    .downcast_ref::<FunctionStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Function statement"))?;

                // Return type slot
                let ret_slot = TypeSlot::FunctionReturn(
                    ctx.scope_path.clone(),
                    func.name.clone(),
                );
                self.insert_slot(ret_slot.clone(), func.return_type.as_ref());

                // Argument slots
                let mut arg_slots = Vec::new();
                for arg in &func.args {
                    let arg_slot = TypeSlot::Argument(
                        ctx.scope_path.clone(),
                        func.name.clone(),
                        arg.name.clone(),
                    );
                    self.insert_slot(arg_slot.clone(), arg.r#type.as_ref());
                    arg_slots.push((arg.name.clone(), arg_slot));
                }

                ctx.functions.insert(
                    func.name.clone(),
                    (ret_slot.clone(), arg_slots),
                );

                // Recurse into function body with inner scope
                let mut inner_ctx = ScopeContext {
                    scope_path: {
                        let mut p = ctx.scope_path.clone();
                        p.push(func.name.clone());
                        p
                    },
                    // Inner scope inherits outer declarations
                    variables: ctx.variables.clone(),
                    functions: ctx.functions.clone(),
                    procedures: ctx.procedures.clone(),
                };

                // Add args to inner scope as variable references
                for arg in &func.args {
                    let arg_slot = TypeSlot::Argument(
                        ctx.scope_path.clone(),
                        func.name.clone(),
                        arg.name.clone(),
                    );
                    inner_ctx.variables.insert(arg.name.clone(), arg_slot);
                }

                // The implicit return variable inside the function body
                let inner_ret_var_slot = TypeSlot::Variable(
                    inner_ctx.scope_path.clone(),
                    func.name.clone(),
                );
                // If the return type is known explicitly, the inner return var is also known
                self.insert_slot(inner_ret_var_slot.clone(), func.return_type.as_ref());
                inner_ctx.variables.insert(func.name.clone(), inner_ret_var_slot.clone());

                self.discover_slots(&func.body, &mut inner_ctx)?;

                // If the return type is NOT explicit, it depends on the inner return var
                if func.return_type.is_none() {
                    if self.nodes.contains_key(&inner_ret_var_slot) {
                        let node = self.nodes.get_mut(&ret_slot).unwrap();
                        node.rule = ResolutionRule::Mirror {
                            source: inner_ret_var_slot.clone(),
                            reason: format!(
                                "inferred from assignment to return variable '{}'",
                                func.name
                            ),
                        };
                        node.depends_on.insert(inner_ret_var_slot);
                    }
                }
            }
            StatementEnum::Procedure => {
                let proc = stmt.inner.as_any()
                    .downcast_ref::<ProcedureStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Procedure statement"))?;

                let mut arg_slots = Vec::new();
                for arg in &proc.args {
                    let arg_slot = TypeSlot::Argument(
                        ctx.scope_path.clone(),
                        proc.name.clone(),
                        arg.name.clone(),
                    );
                    self.insert_slot(arg_slot.clone(), arg.r#type.as_ref());
                    arg_slots.push((arg.name.clone(), arg_slot));
                }

                ctx.procedures.insert(proc.name.clone(), arg_slots);

                // Recurse into procedure body
                let mut inner_ctx = ScopeContext {
                    scope_path: {
                        let mut p = ctx.scope_path.clone();
                        p.push(proc.name.clone());
                        p
                    },
                    variables: ctx.variables.clone(),
                    functions: ctx.functions.clone(),
                    procedures: ctx.procedures.clone(),
                };

                for arg in &proc.args {
                    let arg_slot = TypeSlot::Argument(
                        ctx.scope_path.clone(),
                        proc.name.clone(),
                        arg.name.clone(),
                    );
                    inner_ctx.variables.insert(arg.name.clone(), arg_slot);
                }

                self.discover_slots(&proc.body, &mut inner_ctx)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Walk statements looking for assignments and call sites to establish dependencies.
    fn discover_dependencies(
        &mut self,
        stmt: &Statement,
        ctx: &mut ScopeContext,
    ) -> Result<()> {
        match stmt.enum_variant {
            StatementEnum::Assign => {
                let assign = stmt.inner.as_any()
                    .downcast_ref::<AssignStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Assign statement"))?;

                // Discover function call sites within the RHS expression
                self.discover_expr_function_calls(&assign.value, ctx)?;

                let var_name = &assign.identifier.name;
                let var_slot = match ctx.variables.get(var_name) {
                    Some(s) => s.clone(),
                    None => return Ok(()), // unknown variable, skip
                };

                // Only add dependency if the slot is still unresolved
                if let Some(node) = self.nodes.get(&var_slot) {
                    if node.resolved.is_some() {
                        return Ok(()); // already has explicit type
                    }
                } else {
                    return Ok(());
                }

                // Build a recipe for the RHS expression and collect its dependencies
                let mut deps = HashSet::new();
                let recipe = self.build_expr_recipe(&assign.value, ctx, &mut deps);

                if let Some(node) = self.nodes.get_mut(&var_slot) {
                    node.rule = ResolutionRule::InferredFrom {
                        recipe,
                        reason: "inferred from assignment".to_string(),
                    };
                    node.depends_on = deps;
                }
            }
            StatementEnum::Write => {
                let write_stmt = stmt.inner.as_any()
                    .downcast_ref::<WriteStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Write statement"))?;

                // Discover function call sites within all write expressions
                for expr in &write_stmt.exprs {
                    self.discover_expr_function_calls(expr, ctx)?;
                }
            }
            StatementEnum::ProcedureCall => {
                let call = stmt.inner.as_any()
                    .downcast_ref::<ProcedureCallStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast ProcedureCall statement"))?;

                self.discover_call_site_deps(&call.name, &call.args, false, ctx)?;
            }
            StatementEnum::FunctionCall => {
                let call = stmt.inner.as_any()
                    .downcast_ref::<FunctionCallStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast FunctionCall statement"))?;

                self.discover_call_site_deps(&call.name, &call.args, true, ctx)?;
            }
            StatementEnum::If => {
                let if_stmt = stmt.inner.as_any()
                    .downcast_ref::<IfStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast If statement"))?;

                self.discover_slots(&if_stmt.then_body, &mut ctx.clone())?;
                for elseif in &if_stmt.elseif_clauses {
                    self.discover_slots(&elseif.body, &mut ctx.clone())?;
                }
                if let Some(else_body) = &if_stmt.else_body {
                    self.discover_slots(else_body, &mut ctx.clone())?;
                }
            }
            StatementEnum::Loop => {
                let loop_stmt = stmt.inner.as_any()
                    .downcast_ref::<LoopStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Loop statement"))?;

                let mut inner_ctx = ctx.clone();
                // Loop iterator is always RE
                let iter_slot = TypeSlot::Variable(
                    ctx.scope_path.clone(),
                    loop_stmt.iterator.clone(),
                );
                self.insert_slot(iter_slot.clone(), Some(&RosyType::RE()));
                inner_ctx.variables.insert(loop_stmt.iterator.clone(), iter_slot);
                self.discover_slots(&loop_stmt.body, &mut inner_ctx)?;
            }
            StatementEnum::WhileLoop => {
                let while_stmt = stmt.inner.as_any()
                    .downcast_ref::<WhileStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast While statement"))?;

                self.discover_slots(&while_stmt.body, &mut ctx.clone())?;
            }
            StatementEnum::PLoop => {
                let ploop_stmt = stmt.inner.as_any()
                    .downcast_ref::<PLoopStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast PLoop statement"))?;

                let mut inner_ctx = ctx.clone();
                let iter_slot = TypeSlot::Variable(
                    ctx.scope_path.clone(),
                    ploop_stmt.iterator.clone(),
                );
                self.insert_slot(iter_slot.clone(), Some(&RosyType::RE()));
                inner_ctx.variables.insert(ploop_stmt.iterator.clone(), iter_slot);
                self.discover_slots(&ploop_stmt.body, &mut inner_ctx)?;
            }
            StatementEnum::Fit => {
                let fit_stmt = stmt.inner.as_any()
                    .downcast_ref::<FitStatement>()
                    .ok_or_else(|| anyhow!("Failed to downcast Fit statement"))?;

                self.discover_slots(&fit_stmt.body, &mut ctx.clone())?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Recursively walk an expression tree looking for function calls.
    /// For each one found, wire up call-site argument dependencies.
    fn discover_expr_function_calls(
        &mut self,
        expr: &Expr,
        ctx: &ScopeContext,
    ) -> Result<()> {
        match &expr.enum_variant {
            ExprEnum::FunctionCall => {
                if let Some(func_call) = expr.inner.as_any()
                    .downcast_ref::<expr_function_call::FunctionCallExpr>()
                {
                    self.discover_call_site_deps(
                        &func_call.name, &func_call.args, true, ctx
                    )?;
                    // Recurse into the arguments too
                    for arg in &func_call.args {
                        self.discover_expr_function_calls(arg, ctx)?;
                    }
                }
            }
            ExprEnum::Add => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<add::AddExpr>() {
                    self.discover_expr_function_calls(&e.left, ctx)?;
                    self.discover_expr_function_calls(&e.right, ctx)?;
                }
            }
            ExprEnum::Sub => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<sub::SubExpr>() {
                    self.discover_expr_function_calls(&e.left, ctx)?;
                    self.discover_expr_function_calls(&e.right, ctx)?;
                }
            }
            ExprEnum::Mult => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<mult::MultExpr>() {
                    self.discover_expr_function_calls(&e.left, ctx)?;
                    self.discover_expr_function_calls(&e.right, ctx)?;
                }
            }
            ExprEnum::Div => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<div::DivExpr>() {
                    self.discover_expr_function_calls(&e.left, ctx)?;
                    self.discover_expr_function_calls(&e.right, ctx)?;
                }
            }
            ExprEnum::Extract => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<extract::ExtractExpr>() {
                    self.discover_expr_function_calls(&e.object, ctx)?;
                    self.discover_expr_function_calls(&e.index, ctx)?;
                }
            }
            ExprEnum::Concat => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<concat::ConcatExpr>() {
                    for term in &e.terms {
                        self.discover_expr_function_calls(term, ctx)?;
                    }
                }
            }
            ExprEnum::Sin => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<sin::SinExpr>() {
                    self.discover_expr_function_calls(&e.expr, ctx)?;
                }
            }
            ExprEnum::StringConvert => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<string_convert::StringConvertExpr>() {
                    self.discover_expr_function_calls(&e.expr, ctx)?;
                }
            }
            // Leaf expressions — no children to recurse into
            _ => {}
        }
        Ok(())
    }

    /// For a call site like `F(X, Y)`, if `F` has untyped parameters, add
    /// dependencies from the parameter slots to the argument expressions.
    fn discover_call_site_deps(
        &mut self,
        name: &str,
        args: &[Expr],
        is_function: bool,
        ctx: &ScopeContext,
    ) -> Result<()> {
        let param_slots: Option<Vec<(String, TypeSlot)>> = if is_function {
            ctx.functions.get(name).map(|(_, params)| params.clone())
        } else {
            ctx.procedures.get(name).map(|params| params.clone())
        };

        if let Some(params) = param_slots {
            for (i, arg_expr) in args.iter().enumerate() {
                if let Some((_, param_slot)) = params.get(i) {
                    // Only update if the parameter slot is unresolved
                    if let Some(param_node) = self.nodes.get(param_slot) {
                        if param_node.resolved.is_some() {
                            continue;
                        }
                    } else {
                        continue;
                    }

                    // Build recipe for the argument expression
                    let mut deps = HashSet::new();
                    let recipe = self.build_expr_recipe(arg_expr, ctx, &mut deps);

                    let node = self.nodes.get_mut(param_slot).unwrap();
                    node.rule = ResolutionRule::InferredFrom {
                        recipe,
                        reason: format!("inferred from argument {} at call site", i + 1),
                    };
                    node.depends_on = deps;
                }
            }
        }

        Ok(())
    }

    /// Build an ExprRecipe from an AST expression, collecting dependency slots.
    fn build_expr_recipe(
        &self,
        expr: &Expr,
        ctx: &ScopeContext,
        deps: &mut HashSet<TypeSlot>,
    ) -> ExprRecipe {
        match &expr.enum_variant {
            ExprEnum::Number => ExprRecipe::Literal(RosyType::RE()),
            ExprEnum::String => ExprRecipe::Literal(RosyType::ST()),
            ExprEnum::Boolean => ExprRecipe::Literal(RosyType::LO()),
            ExprEnum::Complex => ExprRecipe::Literal(RosyType::CM()),
            ExprEnum::StringConvert => ExprRecipe::Literal(RosyType::ST()),
            ExprEnum::LogicalConvert => ExprRecipe::Literal(RosyType::LO()),
            ExprEnum::DA => ExprRecipe::Literal(RosyType::DA()),
            ExprEnum::Length => ExprRecipe::Literal(RosyType::RE()),
            ExprEnum::Not => ExprRecipe::Literal(RosyType::LO()),
            ExprEnum::Eq | ExprEnum::Neq | ExprEnum::Lt | ExprEnum::Gt |
            ExprEnum::Lte | ExprEnum::Gte => ExprRecipe::Literal(RosyType::LO()),

            ExprEnum::Var => {
                if let Some(var_expr) = expr.inner.as_any()
                    .downcast_ref::<var_expr::VarExpr>()
                {
                    let ident = &var_expr.identifier;
                    if let Some(slot) = ctx.variables.get(&ident.name) {
                        deps.insert(slot.clone());
                        ExprRecipe::Variable(slot.clone())
                    } else {
                        ExprRecipe::Unknown
                    }
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::FunctionCall => {
                if let Some(func_call) = expr.inner.as_any()
                    .downcast_ref::<expr_function_call::FunctionCallExpr>()
                {
                    if let Some((ret_slot, param_slots)) = ctx.functions.get(&func_call.name) {
                        deps.insert(ret_slot.clone());

                        // Also discover call-site arg dependencies inline
                        for (i, arg_expr) in func_call.args.iter().enumerate() {
                            if let Some((_, param_slot)) = param_slots.get(i) {
                                // Only wire up if param is unresolved
                                let is_unresolved = self.nodes.get(param_slot)
                                    .map_or(false, |n| n.resolved.is_none());

                                if is_unresolved {
                                    let mut arg_deps = HashSet::new();
                                    let recipe = self.build_expr_recipe(arg_expr, ctx, &mut arg_deps);

                                    // We can't mutate nodes here (borrow checker), so
                                    // just collect the arg expression deps for the
                                    // outer expression. The actual param slot wiring
                                    // is done by discover_call_site_deps for statement-
                                    // level calls and by WRITE/assign discovery for
                                    // expression-level calls.
                                    deps.extend(arg_deps);
                                    let _ = recipe;
                                }
                            }
                        }

                        ExprRecipe::FunctionCall(ret_slot.clone())
                    } else {
                        ExprRecipe::Unknown
                    }
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::Sin => {
                if let Some(sin_expr) = expr.inner.as_any()
                    .downcast_ref::<sin::SinExpr>()
                {
                    let inner = self.build_expr_recipe(&sin_expr.expr, ctx, deps);
                    ExprRecipe::Sin(Box::new(inner))
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::Add => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Add),
            ExprEnum::Sub => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Sub),
            ExprEnum::Mult => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Mult),
            ExprEnum::Div => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Div),
            ExprEnum::Extract => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Extract),
            ExprEnum::Concat => {
                if let Some(concat_expr) = expr.inner.as_any()
                    .downcast_ref::<concat::ConcatExpr>()
                {
                    let recipes: Vec<ExprRecipe> = concat_expr.terms.iter()
                        .map(|t| self.build_expr_recipe(t, ctx, deps))
                        .collect();
                    ExprRecipe::Concat(recipes)
                } else {
                    ExprRecipe::Unknown
                }
            }
        }
    }

    /// Helper: build a binary operator recipe from an expression.
    fn build_binop_recipe(
        &self,
        expr: &Expr,
        ctx: &ScopeContext,
        deps: &mut HashSet<TypeSlot>,
        op: BinaryOpKind,
    ) -> ExprRecipe {
        macro_rules! try_binop {
            ($type:ty) => {
                if let Some(binop) = expr.inner.as_any().downcast_ref::<$type>() {
                    let left = self.build_expr_recipe(&binop.left, ctx, deps);
                    let right = self.build_expr_recipe(&binop.right, ctx, deps);
                    return ExprRecipe::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
            };
        }
        match op {
            BinaryOpKind::Add => try_binop!(add::AddExpr),
            BinaryOpKind::Sub => try_binop!(sub::SubExpr),
            BinaryOpKind::Mult => try_binop!(mult::MultExpr),
            BinaryOpKind::Div => try_binop!(div::DivExpr),
            BinaryOpKind::Extract => {
                if let Some(ext) = expr.inner.as_any().downcast_ref::<extract::ExtractExpr>() {
                    let left = self.build_expr_recipe(&ext.object, ctx, deps);
                    let right = self.build_expr_recipe(&ext.index, ctx, deps);
                    return ExprRecipe::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
            }
        }
        ExprRecipe::Unknown
    }

    // ─── Phase 2: Topological Resolution (Kahn's Algorithm) ─────────────

    /// Process nodes whose dependencies are all resolved first, resolve them,
    /// then process their dependents, and so on. One pass — no iteration.
    fn topological_resolve(&mut self) -> Result<()> {
        // Build reverse dependency map: slot → set of slots that depend on it
        let mut dependents: HashMap<TypeSlot, Vec<TypeSlot>> = HashMap::new();
        let mut in_degree: HashMap<TypeSlot, usize> = HashMap::new();

        for (slot, node) in &self.nodes {
            // Only count edges to slots that exist in the graph
            let real_deps: usize = node.depends_on.iter()
                .filter(|d| self.nodes.contains_key(d))
                .count();
            in_degree.insert(slot.clone(), real_deps);

            for dep in &node.depends_on {
                if self.nodes.contains_key(dep) {
                    dependents.entry(dep.clone())
                        .or_default()
                        .push(slot.clone());
                }
            }
        }

        // Seed the queue with all nodes that have in-degree 0
        let mut queue: VecDeque<TypeSlot> = VecDeque::new();
        for (slot, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(slot.clone());
            }
        }

        let mut resolved_count = 0;

        while let Some(slot) = queue.pop_front() {
            // Resolve this node if not already resolved
            if self.nodes.get(&slot).map_or(true, |n| n.resolved.is_none()) {
                self.resolve_node(&slot)?;
            }
            resolved_count += 1;

            // Decrement in-degree for all dependents
            if let Some(deps) = dependents.get(&slot) {
                for dep_slot in deps {
                    if let Some(deg) = in_degree.get_mut(dep_slot) {
                        *deg = deg.saturating_sub(1);
                        if *deg == 0 {
                            queue.push_back(dep_slot.clone());
                        }
                    }
                }
            }
        }

        // Any remaining unresolved nodes are cycles or truly unresolvable
        let unresolved: Vec<&GraphNode> = self.nodes.values()
            .filter(|n| n.resolved.is_none())
            .collect();

        if unresolved.is_empty() {
            return Ok(());
        }

        // ── Build error message ──

        // Partition into cycle nodes (have unresolved deps) vs no-info nodes
        let cycle_slots: Vec<&TypeSlot> = unresolved.iter()
            .filter(|n| n.depends_on.iter().any(|d|
                self.nodes.get(d).map_or(false, |dn| dn.resolved.is_none())
            ))
            .map(|n| &n.slot)
            .collect();

        let no_info_slots: Vec<&TypeSlot> = unresolved.iter()
            .filter(|n| !n.depends_on.iter().any(|d|
                self.nodes.get(d).map_or(false, |dn| dn.resolved.is_none())
            ))
            .map(|n| &n.slot)
            .collect();

        let total = unresolved.len();
        let mut msg = format!(
            "\n╭─ Type Resolution Failed ─────────────────────────────────\n│\n│  {} unresolved type{} found:\n│",
            total,
            if total == 1 { "" } else { "s" }
        );

        // Report cycle errors
        if !cycle_slots.is_empty() {
            msg.push_str("\n│  🔄 Circular dependencies detected:");
            msg.push_str("\n│");
            for slot in &cycle_slots {
                let node = self.nodes.get(slot).unwrap();
                let dep_names: Vec<String> = node.depends_on.iter()
                    .filter(|d| self.nodes.get(*d).map_or(false, |n| n.resolved.is_none()))
                    .map(|d| format!("{}", d))
                    .collect();
                msg.push_str(&format!("\n│    ✗ {} depends on:",
                    slot,
                ));
                for dep in &dep_names {
                    msg.push_str(&format!("\n│        → {}", dep));
                }
            }
            msg.push_str("\n│");
            msg.push_str("\n│    Break the cycle by adding an explicit type annotation");
            msg.push_str("\n│    to at least one of the slots above.");
            msg.push_str("\n│");
        }

        // Report no-info errors
        for slot in &no_info_slots {
            let hint = match slot {
                TypeSlot::Variable(scope, name) => {
                    let scope_str = if scope.is_empty() {
                        "global scope".to_string()
                    } else {
                        format!("'{}'", scope.join(" > "))
                    };
                    format!(
                        "  ✗ Could not determine the type of variable '{}' (in {})\n\
                         \x20   • It is declared but never assigned a value with a known type.\n\
                         \x20   • Try assigning it a value (e.g. {} := 0;) or adding an explicit type.\n\
                         \x20   → Add an explicit type: VARIABLE (RE) {} ;",
                        name, scope_str, name, name
                    )
                }
                TypeSlot::FunctionReturn(_, name) => {
                    format!(
                        "  ✗ Could not determine the return type of function '{}'\n\
                         \x20   • The function body doesn't assign a known-type value to '{}'.\n\
                         \x20   → Add an explicit return type: FUNCTION (RE) {} ... ;",
                        name, name, name
                    )
                }
                TypeSlot::Argument(_, callable, arg) => {
                    format!(
                        "  ✗ Could not determine the type of argument '{}' of '{}'\n\
                         \x20   • No call site passes a value with a known type for this argument.\n\
                         \x20   → Add an explicit type: {} (RE)",
                        arg, callable, arg
                    )
                }
            };
            for line in hint.lines() {
                msg.push_str(&format!("\n│  {}", line));
            }
            msg.push_str("\n│");
        }

        msg.push_str("\n│  The type resolver builds a dependency graph and resolves");
        msg.push_str("\n│  types from leaves inward. If a slot has no path to a");
        msg.push_str("\n│  known type, or is part of a cycle, it cannot be resolved.");
        msg.push_str("\n│");
        msg.push_str("\n╰──────────────────────────────────────────────────────────");
        Err(anyhow!("{}", msg))
    }

    /// Resolve a single node by evaluating its rule.
    fn resolve_node(&mut self, slot: &TypeSlot) -> Result<()> {
        let node = self.nodes.get(slot)
            .ok_or_else(|| anyhow!("No node for slot {}", slot))?;

        if node.resolved.is_some() {
            return Ok(());
        }

        let rule = node.rule.clone();
        let resolved_type = match rule {
            ResolutionRule::Explicit(t) => t,
            ResolutionRule::InferredFrom { recipe, .. } => {
                self.evaluate_recipe(&recipe)?
            }
            ResolutionRule::Mirror { source, .. } => {
                self.nodes.get(&source)
                    .and_then(|n| n.resolved.clone())
                    .ok_or_else(|| anyhow!(
                        "Mirror source {} not resolved when resolving {}",
                        source, slot
                    ))?
            }
            ResolutionRule::Unresolved => {
                // No rule was ever established — leave as None
                return Ok(());
            }
        };

        self.nodes.get_mut(slot).unwrap().resolved = Some(resolved_type);
        Ok(())
    }

    /// Evaluate an ExprRecipe using already-resolved slot types.
    fn evaluate_recipe(&self, recipe: &ExprRecipe) -> Result<RosyType> {
        match recipe {
            ExprRecipe::Literal(t) => Ok(t.clone()),
            ExprRecipe::Variable(slot) => {
                self.nodes.get(slot)
                    .and_then(|n| n.resolved.clone())
                    .ok_or_else(|| anyhow!("Variable slot {} not resolved", slot))
            }
            ExprRecipe::FunctionCall(ret_slot) => {
                self.nodes.get(ret_slot)
                    .and_then(|n| n.resolved.clone())
                    .ok_or_else(|| anyhow!("Function return slot {} not resolved", ret_slot))
            }
            ExprRecipe::BinaryOp { op, left, right } => {
                let left_type = self.evaluate_recipe(left)?;
                let right_type = self.evaluate_recipe(right)?;
                let result = match op {
                    BinaryOpKind::Add => crate::rosy_lib::operators::add::get_return_type(&left_type, &right_type),
                    BinaryOpKind::Sub => crate::rosy_lib::operators::sub::get_return_type(&left_type, &right_type),
                    BinaryOpKind::Mult => crate::rosy_lib::operators::mult::get_return_type(&left_type, &right_type),
                    BinaryOpKind::Div => crate::rosy_lib::operators::div::get_return_type(&left_type, &right_type),
                    BinaryOpKind::Extract => crate::rosy_lib::operators::extract::get_return_type(&left_type, &right_type),
                };
                result.ok_or_else(|| anyhow!(
                    "No operator rule for {:?}({}, {})", op, left_type, right_type
                ))
            }
            ExprRecipe::Concat(recipes) => {
                let mut iter = recipes.iter();
                let first = iter.next()
                    .ok_or_else(|| anyhow!("Empty concat expression"))?;
                let mut result = self.evaluate_recipe(first)?;
                for r in iter {
                    let t = self.evaluate_recipe(r)?;
                    result = crate::rosy_lib::operators::concat::get_return_type(&result, &t)
                        .ok_or_else(|| anyhow!("No concat rule for {} & {}", result, t))?;
                }
                Ok(result)
            }
            ExprRecipe::Sin(inner) => {
                let input_type = self.evaluate_recipe(inner)?;
                crate::rosy_lib::intrinsics::sin::get_return_type(&input_type)
                    .ok_or_else(|| anyhow!("No SIN rule for {}", input_type))
            }
            ExprRecipe::Unknown => {
                Err(anyhow!("Cannot evaluate unknown expression recipe"))
            }
        }
    }

    // ─── Phase 3: Apply to AST ──────────────────────────────────────────

    /// Walk the AST and fill in all `None` type fields with resolved types.
    fn apply_to_ast(
        &self,
        statements: &mut [Statement],
        current_scope: &[String],
    ) -> Result<()> {
        for stmt in statements.iter_mut() {
            match stmt.enum_variant {
                StatementEnum::VarDecl => {
                    let var_decl = stmt.inner.as_any_mut()
                        .downcast_mut::<VarDeclStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast VarDecl for mutation"))?;

                    if var_decl.data.r#type.is_none() {
                        let slot = TypeSlot::Variable(
                            current_scope.to_vec(),
                            var_decl.data.name.clone(),
                        );
                        if let Some(node) = self.nodes.get(&slot) {
                            if let Some(t) = &node.resolved {
                                var_decl.data.r#type = Some(t.clone());
                            }
                        }
                    }
                }
                StatementEnum::Function => {
                    let func = stmt.inner.as_any_mut()
                        .downcast_mut::<FunctionStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast Function for mutation"))?;

                    // Return type
                    if func.return_type.is_none() {
                        let slot = TypeSlot::FunctionReturn(
                            current_scope.to_vec(),
                            func.name.clone(),
                        );
                        if let Some(node) = self.nodes.get(&slot) {
                            if let Some(t) = &node.resolved {
                                func.return_type = Some(t.clone());
                            }
                        }
                    }

                    // Argument types
                    for arg in &mut func.args {
                        if arg.r#type.is_none() {
                            let slot = TypeSlot::Argument(
                                current_scope.to_vec(),
                                func.name.clone(),
                                arg.name.clone(),
                            );
                            if let Some(node) = self.nodes.get(&slot) {
                                if let Some(t) = &node.resolved {
                                    arg.r#type = Some(t.clone());
                                }
                            }
                        }
                    }

                    // Resolve the implicit return variable (first stmt in body)
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
                    self.apply_to_ast(&mut func.body, &inner_scope)?;
                }
                StatementEnum::Procedure => {
                    let proc = stmt.inner.as_any_mut()
                        .downcast_mut::<ProcedureStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast Procedure for mutation"))?;

                    for arg in &mut proc.args {
                        if arg.r#type.is_none() {
                            let slot = TypeSlot::Argument(
                                current_scope.to_vec(),
                                proc.name.clone(),
                                arg.name.clone(),
                            );
                            if let Some(node) = self.nodes.get(&slot) {
                                if let Some(t) = &node.resolved {
                                    arg.r#type = Some(t.clone());
                                }
                            }
                        }
                    }

                    let mut inner_scope = current_scope.to_vec();
                    inner_scope.push(proc.name.clone());
                    self.apply_to_ast(&mut proc.body, &inner_scope)?;
                }
                StatementEnum::If => {
                    let if_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<IfStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast If for mutation"))?;

                    self.apply_to_ast(&mut if_stmt.then_body, current_scope)?;
                    for elseif in &mut if_stmt.elseif_clauses {
                        self.apply_to_ast(&mut elseif.body, current_scope)?;
                    }
                    if let Some(else_body) = &mut if_stmt.else_body {
                        self.apply_to_ast(else_body, current_scope)?;
                    }
                }
                StatementEnum::Loop => {
                    let loop_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<LoopStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast Loop for mutation"))?;

                    self.apply_to_ast(&mut loop_stmt.body, current_scope)?;
                }
                StatementEnum::WhileLoop => {
                    let while_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<WhileStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast While for mutation"))?;

                    self.apply_to_ast(&mut while_stmt.body, current_scope)?;
                }
                StatementEnum::PLoop => {
                    let ploop_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<PLoopStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast PLoop for mutation"))?;

                    self.apply_to_ast(&mut ploop_stmt.body, current_scope)?;
                }
                StatementEnum::Fit => {
                    let fit_stmt = stmt.inner.as_any_mut()
                        .downcast_mut::<FitStatement>()
                        .ok_or_else(|| anyhow!("Failed to downcast Fit for mutation"))?;

                    self.apply_to_ast(&mut fit_stmt.body, current_scope)?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
