/// Phase 1: AST Discovery
///
/// Walks the AST to discover all type slots and build the dependency graph.
/// Creates graph nodes for variables, function args, function return types,
/// and procedure args, then establishes edges based on assignments and call sites.

use std::collections::HashSet;
use anyhow::{anyhow, Result};
use crate::rosy_lib::RosyType;
use crate::program::statements::*;
use crate::program::expressions::*;

use super::{
    TypeResolver, TypeSlot, 
    ScopeContext, ResolutionRule, 
    ExprRecipe, BinaryOpKind,
};

impl TypeResolver {
    /// Walk the AST, creating graph nodes for every type slot and recording
    /// their dependencies.
    pub(super) fn discover_slots(
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
                self.insert_slot(slot.clone(), var_decl.data.r#type.as_ref(), Some(stmt.source_location.clone()));
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
                self.insert_slot(ret_slot.clone(), func.return_type.as_ref(), Some(stmt.source_location.clone()));

                // Argument slots
                let mut arg_slots = Vec::new();
                for arg in &func.args {
                    let arg_slot = TypeSlot::Argument(
                        ctx.scope_path.clone(),
                        func.name.clone(),
                        arg.name.clone(),
                    );
                    self.insert_slot(arg_slot.clone(), arg.r#type.as_ref(), Some(stmt.source_location.clone()));
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
                self.insert_slot(inner_ret_var_slot.clone(), func.return_type.as_ref(), Some(stmt.source_location.clone()));
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
                    self.insert_slot(arg_slot.clone(), arg.r#type.as_ref(), Some(stmt.source_location.clone()));
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

                // Build a recipe for the RHS expression and collect its dependencies
                let mut deps = HashSet::new();
                let recipe = self.build_expr_recipe(&assign.value, ctx, &mut deps);

                if let Some(node) = self.nodes.get(&var_slot) {
                    if node.resolved.is_some() {
                        // Already has an explicit type — check that the new
                        // assignment is compatible (if evaluable now).
                        // Account for indexing on the LHS: X[I, J] := expr
                        // means we're assigning to a sub-element, so reduce
                        // the declared dimensions by the number of indices.
                        let mut explicit_type = node.resolved.as_ref().unwrap().clone();
                        let num_indices = assign.identifier.num_index_dimensions();
                        explicit_type.dimensions = explicit_type.dimensions
                            .saturating_sub(num_indices);
                        if let Ok(new_type) = self.evaluate_recipe(&recipe) {
                            if new_type != explicit_type {
                                let scope_str = if ctx.scope_path.is_empty() {
                                    "global scope".to_string()
                                } else {
                                    format!("'{}'", ctx.scope_path.join(" > "))
                                };
                                let decl_hint = node.declared_at.as_ref()
                                    .map(|loc| format!("\n│  📍 Declared at: {}", loc))
                                    .unwrap_or_default();
                                let assign_hint = format!("\n│  📍 Assigned at: {}", stmt.source_location);
                                return Err(anyhow!(
                                    "\n╭─ Type Conflict ──────────────────────────────────────────\n\
                                     │\n\
                                     │  Variable '{}' (in {}) is declared as {} but is\n\
                                     │  assigned a value of type {}.{}{}\n\
                                     │\n\
                                     │  💡 Either:\n\
                                     │     • Change the explicit type to match the assignment, or\n\
                                     │     • Split into separate variables: {}_{:?}  and  {}_{:?}\n\
                                     │\n\
                                     ╰──────────────────────────────────────────────────────────",
                                    var_name, scope_str, explicit_type, new_type,
                                    decl_hint, assign_hint,
                                    var_name, explicit_type.base_type, var_name, new_type.base_type,
                                ));
                            }
                        }
                        return Ok(()); // already has explicit type, no inference needed
                    }
                } else {
                    return Ok(());
                }

                // Check for conflicting re-assignment: if a previous assignment
                // already established an inference recipe, verify the new one
                // produces the same type (when both are evaluable).
                let has_existing_rule = matches!(
                    self.nodes.get(&var_slot).map(|n| &n.rule),
                    Some(ResolutionRule::InferredFrom { .. })
                );

                if has_existing_rule {
                    let old_recipe = if let Some(node) = self.nodes.get(&var_slot) {
                        if let ResolutionRule::InferredFrom { recipe: ref r, .. } = node.rule {
                            Some(r.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if let Some(ref old_recipe) = old_recipe {
                        // Try to evaluate both recipes. If the new recipe is
                        // self-referential (e.g. Y:=Y&I), evaluate_recipe will
                        // fail because Y isn't resolved yet. In that case,
                        // temporarily resolve Y to the old type so we can
                        // evaluate what the new assignment produces.
                        let old_type_result = self.evaluate_recipe(old_recipe);
                        let mut new_type_result = self.evaluate_recipe(&recipe);

                        // If the new recipe failed but old succeeded, try again
                        // with a temporary assumption that the variable has the
                        // old type (handles self-referential patterns like Y:=Y&I)
                        if new_type_result.is_err() {
                            if let Ok(ref old_type) = old_type_result {
                                // Temporarily mark this slot as resolved
                                if let Some(node) = self.nodes.get_mut(&var_slot) {
                                    node.resolved = Some(old_type.clone());
                                }
                                new_type_result = self.evaluate_recipe(&recipe);
                                // Undo the temporary resolution
                                if let Some(node) = self.nodes.get_mut(&var_slot) {
                                    node.resolved = None;
                                }
                            }
                        }

                        if let (Ok(old_type), Ok(new_type)) = (
                            old_type_result,
                            new_type_result,
                        ) {
                            if old_type != new_type {
                                // RE ↔ VE coercion: if one is RE and the other is VE,
                                // upgrade to VE. This supports COSY's dynamic typing
                                // pattern where a variable is initialized as a scalar
                                // (Y:=1) and then grown into a vector (Y:=Y&I).
                                let re = RosyType::RE();
                                let ve = RosyType::VE();
                                if (old_type == re && new_type == ve) || (old_type == ve && new_type == re) {
                                    tracing::debug!(
                                        "RE↔VE coercion for '{}': upgrading to VE (RE assignments will wrap in vec![])",
                                        var_name
                                    );
                                    // Use a literal VE recipe — we already know the
                                    // result type. Using the self-referential recipe
                                    // (e.g. Concat(Y, I)) would create a circular
                                    // dependency that can't be resolved.
                                    if let Some(node) = self.nodes.get_mut(&var_slot) {
                                        node.rule = ResolutionRule::InferredFrom {
                                            recipe: ExprRecipe::Literal(RosyType::VE()),
                                            reason: "inferred from assignment (RE→VE upgrade)".to_string(),
                                        };
                                        node.depends_on.clear();
                                    }
                                    return Ok(());
                                }

                                let scope_str = if ctx.scope_path.is_empty() {
                                    "global scope".to_string()
                                } else {
                                    format!("'{}'", ctx.scope_path.join(" > "))
                                };
                                let first_assign_hint = self.nodes.get(&var_slot)
                                    .and_then(|n| n.assigned_at.as_ref())
                                    .map(|loc| format!("\n│  📍 First assigned at:  {}", loc))
                                    .unwrap_or_default();
                                let second_assign_hint = format!("\n│  📍 Then assigned at:   {}", stmt.source_location);
                                return Err(anyhow!(
                                    "\n╭─ Type Conflict ──────────────────────────────────────────\n\
                                     │\n\
                                     │  Variable '{}' (in {}) is assigned conflicting types:\n\
                                     │     • First inferred as:  {}\n\
                                     │     • Then assigned as:   {}{}{}\n\
                                     │\n\
                                     │  Type elision requires each variable to have exactly one type.\n\
                                     │\n\
                                     │  💡 Either:\n\
                                     │     • Add an explicit type:  VARIABLE ({:?}) {} ;\n\
                                     │     • Split into separate variables: {}_{:?}  and  {}_{:?}\n\
                                     │\n\
                                     ╰──────────────────────────────────────────────────────────",
                                    var_name, scope_str, old_type, new_type,
                                    first_assign_hint, second_assign_hint,
                                    old_type.base_type, var_name,
                                    var_name, old_type.base_type, var_name, new_type.base_type,
                                ));
                            }
                        }
                    }

                    // Keep the first (non-self-referential) assignment's recipe.
                    // Subsequent assignments are mutations — the variable's type
                    // is established by its first assignment. This prevents false
                    // cycles when a variable is re-assigned from a value that
                    // transitively depends on itself (e.g. X1 := f(X3) after
                    // X3 := g(X1)).
                    return Ok(());
                }

                if let Some(node) = self.nodes.get_mut(&var_slot) {
                    // Remove self-reference from deps if present — the variable's
                    // type is being established by this very assignment
                    deps.remove(&var_slot);
                    node.rule = ResolutionRule::InferredFrom {
                        recipe,
                        reason: "inferred from assignment".to_string(),
                    };
                    node.depends_on = deps;
                    node.assigned_at = Some(stmt.source_location.clone());
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
                self.insert_slot(iter_slot.clone(), Some(&RosyType::RE()), Some(stmt.source_location.clone()));
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
                self.insert_slot(iter_slot.clone(), Some(&RosyType::RE()), Some(stmt.source_location.clone()));
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
    fn discover_expr_function_calls( // invesigate whether this actually does anything
        &mut self,
        expr: &Expr,
        ctx: &ScopeContext,
    ) -> Result<()> {
        match &expr.enum_variant {
            ExprEnum::Add => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<operators::add::AddExpr>() {
                    self.discover_expr_function_calls(&e.left, ctx)?;
                    self.discover_expr_function_calls(&e.right, ctx)?;
                }
            }
            ExprEnum::Sub => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<operators::sub::SubExpr>() {
                    self.discover_expr_function_calls(&e.left, ctx)?;
                    self.discover_expr_function_calls(&e.right, ctx)?;
                }
            }
            ExprEnum::Mult => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<operators::mult::MultExpr>() {
                    self.discover_expr_function_calls(&e.left, ctx)?;
                    self.discover_expr_function_calls(&e.right, ctx)?;
                }
            }
            ExprEnum::Div => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<operators::div::DivExpr>() {
                    self.discover_expr_function_calls(&e.left, ctx)?;
                    self.discover_expr_function_calls(&e.right, ctx)?;
                }
            }
            ExprEnum::Extract => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<operators::extract::ExtractExpr>() {
                    self.discover_expr_function_calls(&e.object, ctx)?;
                    self.discover_expr_function_calls(&e.index, ctx)?;
                }
            }
            ExprEnum::Concat => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<operators::concat::ConcatExpr>() {
                    for term in &e.terms {
                        self.discover_expr_function_calls(term, ctx)?;
                    }
                }
            }
            ExprEnum::Sin => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<functions::math::trig::sin::SinExpr>() {
                    self.discover_expr_function_calls(&e.expr, ctx)?;
                }
            }
            ExprEnum::Neg => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<operators::neg::NegExpr>() {
                    self.discover_expr_function_calls(&e.operand, ctx)?;
                }
            }
            ExprEnum::StringConvert => {
                if let Some(e) = expr.inner.as_any().downcast_ref::<functions::conversion::string_convert::StringConvertExpr>() {
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
    pub(super) fn build_expr_recipe(
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
            ExprEnum::CD => ExprRecipe::Literal(RosyType::CD()),
            ExprEnum::Length => ExprRecipe::Literal(RosyType::RE()),
            ExprEnum::Vmax => ExprRecipe::Literal(RosyType::RE()),
            ExprEnum::Lst => ExprRecipe::Literal(RosyType::RE()),
            ExprEnum::Lcm => ExprRecipe::Literal(RosyType::RE()),
            ExprEnum::Lcd => ExprRecipe::Literal(RosyType::RE()),
            ExprEnum::Not => ExprRecipe::Literal(RosyType::LO()),
            ExprEnum::Neg => {
                if let Some(neg_expr) = expr.inner.as_any()
                    .downcast_ref::<crate::program::expressions::operators::neg::NegExpr>()
                {
                    let inner = self.build_expr_recipe(&neg_expr.operand, ctx, deps);
                    // Negation preserves the operand type (RE - X gives same type as X for numeric)
                    inner
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::Eq | ExprEnum::Neq | ExprEnum::Lt | ExprEnum::Gt |
            ExprEnum::Lte | ExprEnum::Gte => ExprRecipe::Literal(RosyType::LO()),

            ExprEnum::Var => {
                if let Some(var_expr) = expr.inner.as_any()
                    .downcast_ref::<crate::program::expressions::core::var_expr::VarExpr>()
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
            },
            ExprEnum::Sin => {
                if let Some(sin_expr) = expr.inner.as_any()
                    .downcast_ref::<functions::math::trig::sin::SinExpr>()
                {
                    let inner = self.build_expr_recipe(&sin_expr.expr, ctx, deps);
                    ExprRecipe::Sin(Box::new(inner))
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::Sqr => {
                if let Some(sqr_expr) = expr.inner.as_any()
                    .downcast_ref::<crate::program::expressions::functions::math::sqr::SqrExpr>()
                {
                    let inner = self.build_expr_recipe(&sqr_expr.expr, ctx, deps);
                    ExprRecipe::Sin(Box::new(inner)) // Reuse Sin recipe - same shape (unary op)
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::Derive => {
                if let Some(derive_expr) = expr.inner.as_any()
                    .downcast_ref::<crate::program::expressions::operators::derive::DeriveExpr>()
                {
                    let left = self.build_expr_recipe(&derive_expr.object, ctx, deps);
                    let right = self.build_expr_recipe(&derive_expr.index, ctx, deps);
                    ExprRecipe::BinaryOp {
                        op: BinaryOpKind::Derive,
                        left: Box::new(left),
                        right: Box::new(right),
                    }
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::Add => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Add),
            ExprEnum::Sub => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Sub),
            ExprEnum::Mult => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Mult),
            ExprEnum::Div => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Div),
            ExprEnum::Extract => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Extract),
            ExprEnum::Pow => self.build_binop_recipe(expr, ctx, deps, BinaryOpKind::Pow),
            ExprEnum::Exp => {
                if let Some(exp_expr) = expr.inner.as_any()
                    .downcast_ref::<crate::program::expressions::functions::math::exp::ExpExpr>()
                {
                    let inner = self.build_expr_recipe(&exp_expr.expr, ctx, deps);
                    ExprRecipe::Sin(Box::new(inner)) // Reuse Sin recipe - same shape (unary op)
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::Tan => {
                if let Some(tan_expr) = expr.inner.as_any()
                    .downcast_ref::<crate::program::expressions::functions::math::trig::tan::TanExpr>()
                {
                    let inner = self.build_expr_recipe(&tan_expr.expr, ctx, deps);
                    ExprRecipe::Sin(Box::new(inner)) // Reuse Sin recipe - same shape (unary op)
                } else {
                    ExprRecipe::Unknown
                }
            }
            ExprEnum::Concat => {
                if let Some(concat_expr) = expr.inner.as_any()
                    .downcast_ref::<operators::concat::ConcatExpr>()
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
            BinaryOpKind::Add => try_binop!(operators::add::AddExpr),
            BinaryOpKind::Sub => try_binop!(operators::sub::SubExpr),
            BinaryOpKind::Mult => try_binop!(operators::mult::MultExpr),
            BinaryOpKind::Div => try_binop!(operators::div::DivExpr),
            BinaryOpKind::Extract => {
                if let Some(ext) = expr.inner.as_any().downcast_ref::<operators::extract::ExtractExpr>() {
                    let left = self.build_expr_recipe(&ext.object, ctx, deps);
                    let right = self.build_expr_recipe(&ext.index, ctx, deps);
                    return ExprRecipe::BinaryOp {
                        op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
            }
            BinaryOpKind::Derive => {
                // Derive is handled inline in build_expr_recipe, not here
            }
            BinaryOpKind::Pow => try_binop!(functions::math::pow::PowExpr),
        }
        ExprRecipe::Unknown
    }
}
