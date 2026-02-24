use anyhow::{Result, Context, Error, ensure};

use crate::{
    ast::*, program::expressions::Expr, transpile::{TranspilationInputContext, TranspilationOutput, Transpile}
};

#[derive(Debug)]
pub struct DAInitStatement {
    pub order: Expr,
    pub number_of_variables: Expr,
}

impl FromRule for DAInitStatement {
    fn from_rule(pair: pest::iterators::Pair<Rule>) -> Result<Option<Self>> {
        ensure!(pair.as_rule() == Rule::daini, 
            "Expected `daini` rule when building DA init, found: {:?}", pair.as_rule());
        
        let mut inner = pair.into_inner();
        
        // Parse the first expression (order)
        let order_pair = inner.next()
            .context("Missing order parameter in DAINI statement!")?;
        let order_expr = Expr::from_rule(order_pair)
            .context("Failed to build order expression in DAINI statement!")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for order in DAINI statement"))?;
        
        // Parse the second expression (number of variables)
        let num_vars_pair = inner.next()
            .context("Missing number of variables parameter in DAINI statement!")?;
        let num_vars_expr = Expr::from_rule(num_vars_pair)
            .context("Failed to build number of variables expression in DAINI statement!")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression for number of variables in DAINI statement"))?;
        
        // Parse optional 3rd and 4th arguments (COSY compatibility - ignored)
        if let Some(third_pair) = inner.next() {
            let _third_expr = Expr::from_rule(third_pair)
                .context("Failed to build 3rd expression in DAINI statement!")?;
            eprintln!("[rosy] Note: DAINI 3rd argument (output unit) ignored — Rosy handles this automatically~");
            
            if let Some(fourth_pair) = inner.next() {
                let _fourth_expr = Expr::from_rule(fourth_pair)
                    .context("Failed to build 4th expression in DAINI statement!")?;
                eprintln!("[rosy] Note: DAINI 4th argument (num monomials out) ignored — Rosy handles this automatically~");
            }
        }
        
        Ok(Some(DAInitStatement {
            order: order_expr,
            number_of_variables: num_vars_expr,
        }))
    }
}

impl Transpile for DAInitStatement {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    fn transpile(&self, context: &mut TranspilationInputContext) -> Result<TranspilationOutput, Vec<Error>> {
        // Transpile the order expression
        let order_output = self.order.transpile(context)
            .map_err(|errs| {
                errs.into_iter()
                    .map(|e| e.context("...while transpiling order expression in DAINI"))
                    .collect::<Vec<_>>()
            })?;
        
        // Transpile the number of variables expression
        let num_vars_output = self.number_of_variables.transpile(context)
            .map_err(|errs| {
                errs.into_iter()
                    .map(|e| e.context("...while transpiling number of variables expression in DAINI"))
                    .collect::<Vec<_>>()
            })?;
        
        let serialization = format!(
            "taylor::cleanup_taylor();\n\t\ttaylor::init_taylor(({}).to_owned() as u32, ({}).to_owned() as usize)?;", 
            order_output.serialization, 
            num_vars_output.serialization
        );
        
        let mut requested_variables = order_output.requested_variables;
        requested_variables.extend(num_vars_output.requested_variables);
        
        Ok(TranspilationOutput {
            serialization,
            requested_variables,
        })
    }
}
