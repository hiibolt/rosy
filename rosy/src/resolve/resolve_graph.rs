/// Phase 2: Topological Resolution (Kahn's Algorithm)
///
/// Processes the dependency graph built in Phase 1, resolving type slots
/// from leaves inward. Each slot is resolved exactly once. Cycles and
/// unresolvable slots produce clear error messages.

use std::collections::{HashMap, VecDeque};
use anyhow::{anyhow, Result};
use crate::rosy_lib::RosyType;

use super::{
    TypeResolver, TypeSlot, ResolutionRule, ExprRecipe, BinaryOpKind, GraphNode,
};

impl TypeResolver {
    /// Process nodes whose dependencies are all resolved first, resolve them,
    /// then process their dependents, and so on. One pass — no iteration.
    pub(super) fn topological_resolve(&mut self) -> Result<()> {
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
                    format!(
                        "  ✗ Could not determine the type of variable '{}' (in {})\n\
                         \x20   • It is declared but never assigned a value with a known type.{}\n\
                         \x20   • Try assigning it a value (e.g. {} := 0;) or adding an explicit type.\n\
                         \x20   → Add an explicit type: VARIABLE (RE) {} ;",
                        name, scope_str, reason_hint, name, name
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
                // No rule was ever established — leave as None
                return Ok(());
            }
        };

        self.nodes.get_mut(slot).unwrap().resolved = Some(resolved_type);
        Ok(())
    }

    /// Evaluate an ExprRecipe using already-resolved slot types.
    pub(super) fn evaluate_recipe(&self, recipe: &ExprRecipe) -> Result<RosyType> {
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
}
