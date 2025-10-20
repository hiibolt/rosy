use pest::iterators::Pair;
use anyhow::{Context, Result, ensure};
use crate::ast::{Rule, Statement, DAInitStatement, build_expr};

pub fn build_da_init(pair: Pair<Rule>) -> Result<Option<Statement>> {
    ensure!(pair.as_rule() == Rule::daini, 
        "Expected `daini` rule when building DA init, found: {:?}", pair.as_rule());
    
    let mut inner = pair.into_inner();
    
    // Parse the first expression (order)
    let order_pair = inner.next()
        .context("Missing order parameter in DAINI statement!")?;
    let order_expr = build_expr(order_pair)
        .context("Failed to build order expression in DAINI statement!")?;
    
    // Parse the second expression (number of variables)
    let num_vars_pair = inner.next()
        .context("Missing number of variables parameter in DAINI statement!")?;
    let num_vars_expr = build_expr(num_vars_pair)
        .context("Failed to build number of variables expression in DAINI statement!")?;
    
    Ok(Some(Statement::DAInit(DAInitStatement {
        order: order_expr,
        number_of_variables: num_vars_expr,
    })))
}