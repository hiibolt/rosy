//! # Type Resolution
//!
//! Dependency-graph-based type inference pass that runs between AST
//! construction and transpilation. Fills in `Option<RosyType>` fields
//! left as `None` during parsing.
//!
//! ## Algorithm
//!
//! 1. Walk the AST to discover all "type slots" (variables, function args,
//!    function return types, procedure args)
//! 2. Build a dependency graph between unresolved slots
//! 3. Topologically sort (Kahn's algorithm) and resolve from leaves inward
//! 4. Report cycles as errors

use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::{anyhow, Result};
use crate::rosy_lib::RosyType;
use crate::program::Program;
use crate::program::statements::*;
use crate::program::expressions::*;

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
pub enum ResolutionRule {
    /// The type is already known from an explicit annotation.
    Explicit(RosyType),
    /// Inferred from an assignment RHS or call-site argument expression.
    InferredFrom {
        recipe: ExprRecipe,
        /// Human-readable explanation of where this inference came from.
        reason: String,
    },
    /// Mirrors another slot exactly (e.g., return type from implicit return var).
    Mirror {
        source: TypeSlot,
        /// Human-readable explanation of why this slot mirrors another.
        reason: String,
    },
    /// No rule has been established yet — the slot is truly unknown.
    /// Will remain unresolved and trigger an error if not replaced.
    Unresolved,
}

// ─── Expression Recipe ──────────────────────────────────────────────────────

/// A lightweight "recipe" for computing the type of an expression.
/// Stores just enough info to re-derive the type once dependencies are resolved.
#[derive(Debug, Clone)]
pub enum ExprRecipe {
    /// A literal type — always known.
    Literal(RosyType),
    /// A variable reference — look up its slot.
    Variable(TypeSlot),
    /// A binary operator applied to two sub-recipes.
    BinaryOp { op: BinaryOpKind, left: Box<ExprRecipe>, right: Box<ExprRecipe> },
    /// An n-ary concat of sub-recipes.
    Concat(Vec<ExprRecipe>),
    /// SIN intrinsic — result depends on input type.
    Sin(Box<ExprRecipe>),
    /// Expression whose type couldn't be determined statically.
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOpKind {
    Add, Sub, Mult, Div, Extract, Derive, Pow,
}

// ─── Dependency Graph Node ──────────────────────────────────────────────────

#[derive(Debug)]
pub struct GraphNode {
    pub slot: TypeSlot,
    /// How to compute this slot's type once dependencies are met.
    pub rule: ResolutionRule,
    /// Slots that this node depends on (must be resolved first).
    pub depends_on: HashSet<TypeSlot>,
    /// The resolved type (filled in during topological traversal).
    pub resolved: Option<RosyType>,
    /// Where this slot was declared (VARIABLE statement source location).
    pub declared_at: Option<SourceLocation>,
    /// Where the assignment that established the type inference rule is.
    pub assigned_at: Option<SourceLocation>,
}

// ─── Scope Context (used during graph construction) ─────────────────────────

/// Tracks what's been declared so far in a scope during the discovery walk.
#[derive(Debug, Clone, Default)]
pub struct ScopeContext {
    pub scope_path: Vec<String>,
    /// Maps variable name → its TypeSlot.
    pub variables: HashMap<String, TypeSlot>,
    /// Maps function name → (return_type_slot, vec of (arg_name, arg_slot)).
    pub functions: HashMap<String, (TypeSlot, Vec<(String, TypeSlot)>)>,
    /// Maps procedure name → vec of (arg_name, arg_slot).
    pub procedures: HashMap<String, Vec<(String, TypeSlot)>>,
}

// ─── Type Resolver ──────────────────────────────────────────────────────────

pub struct TypeResolver {
    /// All nodes in the dependency graph, keyed by their slot.
    pub nodes: HashMap<TypeSlot, GraphNode>,
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

    // ─── Graph Infrastructure ───────────────────────────────────────────

    /// Insert a node for a slot. If it has an explicit type, mark it resolved.
    pub fn insert_slot(&mut self, slot: TypeSlot, explicit_type: Option<&RosyType>, declared_at: Option<SourceLocation>) {
        if let Some(t) = explicit_type {
            self.nodes.insert(slot.clone(), GraphNode {
                slot,
                rule: ResolutionRule::Explicit(t.clone()),
                depends_on: HashSet::new(),
                resolved: Some(t.clone()),
                declared_at,
                assigned_at: None,
            });
        } else {
            // Placeholder — rule and deps will be set by discover_dependencies
            self.nodes.entry(slot.clone()).or_insert_with(|| GraphNode {
                slot,
                rule: ResolutionRule::Unresolved,
                depends_on: HashSet::new(),
                resolved: None,
                declared_at,
                assigned_at: None,
            });
        }
    }

    // ─── Phase 1: Discovery ─────────────────────────────────────────────

    /// Walk the AST, creating graph nodes for every type slot and recording
    /// their dependencies.
    pub fn discover_slots(
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

    /// Register a declaration, creating graph nodes for its type slots.
    pub fn register_declaration(
        &mut self,
        stmt: &Statement,
        ctx: &mut ScopeContext,
    ) -> Result<()> {
        let Some(result) = stmt.inner.register_declaration(self, ctx, stmt.source_location.clone()) else {
            return Ok(()); // not a declaration, skip
        };

        result
    }

    /// Walk statements looking for assignments and call sites to establish dependencies.
    pub fn discover_dependencies(
        &mut self,
        stmt: &Statement,
        ctx: &mut ScopeContext,
    ) -> Result<()> {
        let Some(result) = stmt.inner.discover_dependencies(self, ctx, stmt.source_location.clone()) else {
            return Ok(()); // no dependencies to discover, skip
        };

        result
    }

    /// Recursively walk an expression tree looking for function calls.
    /// For each one found, wire up call-site argument dependencies.
    pub fn discover_expr_function_calls(
        &mut self,
        expr: &Expr,
        ctx: &ScopeContext,
    ) -> Result<()> {
        let Some(result) = expr.inner.discover_expr_function_calls(self, ctx) else {
            return Ok(());
        };

        result
    }

    /// For a call site like `F(X, Y)`, if `F` has untyped parameters, add
    /// dependencies from the parameter slots to the argument expressions.
    pub fn discover_call_site_deps(
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
    pub fn build_expr_recipe(
        &self,
        expr: &Expr,
        ctx: &ScopeContext,
        deps: &mut HashSet<TypeSlot>,
    ) -> ExprRecipe {
        expr.inner.build_expr_recipe(self, ctx, deps).unwrap_or(ExprRecipe::Unknown)
    }

    // ─── Phase 2: Topological Resolution ────────────────────────────────

    /// Process nodes whose dependencies are all resolved first, resolve them,
    /// then process their dependents, and so on. One pass — no iteration.
    pub fn topological_resolve(&mut self) -> Result<()> {
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

        let mut resolved_count: usize = 0;

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
            tracing::debug!(
                "Type resolution complete: resolved {} slot{} successfully",
                resolved_count,
                if resolved_count == 1 { "" } else { "s" }
            );
            return Ok(());
        }

        self.build_resolution_error(&unresolved)
    }

    /// Build a detailed error message for unresolved type slots.
    fn build_resolution_error(&self, unresolved: &[&GraphNode]) -> Result<()> {
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
                // Include source locations if available
                if let Some(loc) = &node.declared_at {
                    msg.push_str(&format!("\n│        📍 Declared at: {}", loc));
                }
                if let Some(loc) = &node.assigned_at {
                    msg.push_str(&format!("\n│        📍 Assigned at: {}", loc));
                }
                // Include the resolution rule reason if available
                if let Some(reason) = Self::rule_reason(&node.rule) {
                    msg.push_str(&format!("\n│        ({})", reason));
                }
            }
            msg.push_str("\n│");
            msg.push_str("\n│    Break the cycle by adding an explicit type annotation");
            msg.push_str("\n│    to at least one of the slots above.");
            msg.push_str("\n│");
        }

        // Report no-info errors
        for slot in &no_info_slots {
            let node = self.nodes.get(slot).unwrap();
            let reason_hint = Self::rule_reason(&node.rule)
                .map(|r| format!("\n\x20   • Attempted: {}", r))
                .unwrap_or_default();
            let hint = match slot {
                TypeSlot::Variable(scope, name) => {
                    let scope_str = if scope.is_empty() {
                        "global scope".to_string()
                    } else {
                        format!("'{}'", scope.join(" > "))
                    };
                    let decl_hint = node.declared_at.as_ref()
                        .map(|loc| format!("\n\x20   • Declared at: {}", loc))
                        .unwrap_or_default();
                    format!(
                        "  ✗ Could not determine the type of variable '{}' (in {})\n\
                         \x20   • It is declared but never assigned a value with a known type.{}{}\n\
                         \x20   • Try assigning it a value (e.g. {} := 0;) or adding an explicit type.\n\
                         \x20   → Add an explicit type: VARIABLE (RE) {} ;",
                        name, scope_str, decl_hint, reason_hint, name, name
                    )
                }
                TypeSlot::FunctionReturn(_, name) => {
                    format!(
                        "  ✗ Could not determine the return type of function '{}'\n\
                         \x20   • The function body doesn't assign a known-type value to '{}'.{}\n\
                         \x20   → Add an explicit return type: FUNCTION (RE) {} ... ;",
                        name, name, reason_hint, name
                    )
                }
                TypeSlot::Argument(_, callable, arg) => {
                    format!(
                        "  ✗ Could not determine the type of argument '{}' of '{}'\n\
                         \x20   • No call site passes a value with a known type for this argument.{}\n\
                         \x20   → Add an explicit type: {} (RE)",
                        arg, callable, reason_hint, arg
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

    /// Extract the human-readable reason from a resolution rule, if available.
    fn rule_reason(rule: &ResolutionRule) -> Option<&str> {
        match rule {
            ResolutionRule::InferredFrom { reason, .. } => Some(reason.as_str()),
            ResolutionRule::Mirror { reason, .. } => Some(reason.as_str()),
            _ => None,
        }
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
                // No rule was ever established — default to RE (COSY behavior:
                // all untyped variables that are never assigned default to RE).
                tracing::debug!(
                    "Defaulting unresolved slot {} to RE (COSY default)",
                    slot
                );
                RosyType::RE()
            }
        };

        self.nodes.get_mut(slot).unwrap().resolved = Some(resolved_type);
        Ok(())
    }

    /// Evaluate an ExprRecipe using already-resolved slot types.
    pub fn evaluate_recipe(&self, recipe: &ExprRecipe) -> Result<RosyType> {
        match recipe {
            ExprRecipe::Literal(t) => Ok(t.clone()),
            ExprRecipe::Variable(slot) => {
                self.nodes.get(slot)
                    .and_then(|n| n.resolved.clone())
                    .ok_or_else(|| anyhow!("Variable slot {} not resolved", slot))
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
                    BinaryOpKind::Derive => {
                        // Derive preserves the object type: DA%RE -> DA, CD%RE -> CD
                        match left_type {
                            t if t == RosyType::DA() => Some(RosyType::DA()),
                            t if t == RosyType::CD() => Some(RosyType::CD()),
                            _ => None,
                        }
                    }
                    BinaryOpKind::Pow => crate::rosy_lib::operators::pow::get_return_type(&left_type, &right_type),
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

    // ─── Phase 3: Apply Resolved Types ──────────────────────────────────

    /// Walk the AST and fill in all `None` type fields with resolved types.
    pub fn apply_to_ast(
        &self,
        statements: &mut [Statement],
        current_scope: &[String],
    ) -> Result<()> {
        for stmt in statements.iter_mut() {
            let Some(result) = stmt.inner.apply_resolved_types(self, current_scope) else {
                continue;
            };
            result?;
        }

        Ok(())
    }
}
