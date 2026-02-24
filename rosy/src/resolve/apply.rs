/// Phase 3: Apply Resolved Types to AST
///
/// Walks the AST one more time, filling in all `Option<RosyType>` fields
/// with the types resolved during Phase 2.

use anyhow::{anyhow, Result};
use crate::program::statements::*;

use super::{TypeResolver, TypeSlot};

impl TypeResolver {
    /// Walk the AST and fill in all `None` type fields with resolved types.
    pub(super) fn apply_to_ast(
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
