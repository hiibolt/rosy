/// Phase 1: AST Discovery
///
/// Walks the AST to discover all type slots and build the dependency graph.
/// Creates graph nodes for variables, function args, function return types,
/// and procedure args, then establishes edges based on assignments and call sites.

use std::collections::HashSet;
use anyhow::Result;
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
    pub fn discover_expr_function_calls( // invesigate whether this actually does anything
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
