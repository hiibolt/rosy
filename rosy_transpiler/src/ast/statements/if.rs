use anyhow::{Result, Context, bail};

use super::super::{Rule, Statement, ElseIfClause, build_expr, build_statement};

pub fn build_if(pair: pest::iterators::Pair<Rule>) -> Result<Option<Statement>>{
    let mut inner = pair.into_inner();
    
    // Parse the main IF clause
    let (condition, then_body) = {
        let mut if_clause_inner = inner
            .next()
            .context("Missing if_clause!")?
            .into_inner();
        
        let condition = build_expr(if_clause_inner.next()
            .context("Missing condition in IF clause!")?)
            .context("Failed to build IF condition expression!")?;
        
        let mut then_body = Vec::new();
        while let Some(stmt_pair) = if_clause_inner.next() {
            if stmt_pair.as_rule() == Rule::semicolon {
                continue;
            }
            
            let pair_input = stmt_pair.as_str();
            if let Some(stmt) = build_statement(stmt_pair)
                .with_context(|| format!("Failed to build statement in IF body from:\n{}", pair_input))? {
                then_body.push(stmt);
            }
        }
        
        (condition, then_body)
    };
    
    // Parse ELSEIF clauses
    let mut elseif_clauses = Vec::new();
    let mut else_body = None;
    while let Some(element) = inner.next() {
        match element.as_rule() {
            Rule::elseif_clause => {
                let mut elseif_inner = element.into_inner();
                
                let condition = build_expr(elseif_inner.next()
                    .context("Missing condition in ELSEIF clause!")?)
                    .context("Failed to build ELSEIF condition expression!")?;
                
                let mut body = Vec::new();
                while let Some(stmt_pair) = elseif_inner.next() {
                    if stmt_pair.as_rule() == Rule::semicolon {
                        continue;
                    }
                    
                    let pair_input = stmt_pair.as_str();
                    if let Some(stmt) = build_statement(stmt_pair)
                        .with_context(|| format!("Failed to build statement in ELSEIF body from:\n{}", pair_input))? {
                        body.push(stmt);
                    }
                }
                
                elseif_clauses.push(ElseIfClause { condition, body });
            },
            Rule::else_clause => {
                let mut else_inner = element.into_inner();
                let mut body = Vec::new();
                while let Some(stmt_pair) = else_inner.next() {
                    if stmt_pair.as_rule() == Rule::semicolon {
                        continue;
                    }
                    
                    let pair_input = stmt_pair.as_str();
                    if let Some(stmt) = build_statement(stmt_pair)
                        .with_context(|| format!("Failed to build statement in ELSE body from:\n{}", pair_input))? {
                        body.push(stmt);
                    }
                }
                else_body = Some(body);
            },
            Rule::endif => {
                // End of IF statement
                break;
            },
            _ => {
                bail!("Unexpected element in IF statement: {:?}", element.as_rule());
            }
        }
    }
    
    Ok(Some(Statement::If { condition, then_body, elseif_clauses, else_body }))
}