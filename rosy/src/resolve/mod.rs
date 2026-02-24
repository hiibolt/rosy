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
///
/// ## Module Structure
///
/// - `mod.rs`           — Types, graph infrastructure, and public entry point
/// - `discovery.rs`     — Phase 1: AST walking and dependency graph construction
/// - `resolve_graph.rs` — Phase 2: Topological sort, resolution, and error reporting
/// - `apply.rs`         — Phase 3: Writing resolved types back to the AST

mod discovery;
mod resolve_graph;
mod apply;

use std::collections::{HashMap, HashSet};
use anyhow::Result;
use crate::rosy_lib::RosyType;
use crate::program::Program;
use crate::program::statements::SourceLocation;

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
    Add, Sub, Mult, Div, Extract, Derive,
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
    /// Where this slot was declared (VARIABLE statement source location).
    declared_at: Option<SourceLocation>,
    /// Where the assignment that established the type inference rule is.
    assigned_at: Option<SourceLocation>,
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

    // ─── Graph Infrastructure ───────────────────────────────────────────

    /// Insert a node for a slot. If it has an explicit type, mark it resolved.
    fn insert_slot(&mut self, slot: TypeSlot, explicit_type: Option<&RosyType>, declared_at: Option<SourceLocation>) {
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
}
