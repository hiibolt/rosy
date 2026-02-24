use pest::pratt_parser::PrattParser;
use pest_derive::Parser;
use anyhow::{ensure, Context, Result};
use crate::{program::expressions::Expr, rosy_lib::{RosyBaseType, RosyType}};

#[derive(Parser)]
#[grammar = "../assets/rosy.pest"]
pub struct CosyParser;

// Create a static PrattParser for expressions
lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined from lowest to highest priority
        // Following COSY INFINITY priorities:
        // - Priority 2: Concatenation (&), Equality (=), Not-Equals (#), Less/Greater, comparison
        // - Priority 3: Addition (+), Subtraction (-)
        // - Priority 4: Multiplication (*), Division (/)
        // - Priority 5: Exponentiation (^)
        // - Priority 6: Extraction (|), Derivation (%)
        PrattParser::new()
            // Lowest precedence (Priority 2): concatenation, equality, not-equals, comparisons
            .op(Op::infix(concat, Left) | Op::infix(eq, Left) | Op::infix(neq, Left) 
                | Op::infix(lt, Left) | Op::infix(gt, Left) | Op::infix(lte, Left) | Op::infix(gte, Left))
            // Priority 3: Addition and Subtraction
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            // Priority 4: Multiplication and Division
            .op(Op::infix(mult, Left) | Op::infix(div, Left))
            // Priority 6: Extraction (|) and Derivation (%)
            .op(Op::infix(extract, Left) | Op::infix(derive, Left))
    };
}

pub trait FromRule: Sized {
    fn from_rule ( pair: pest::iterators::Pair<Rule> ) -> Result<Option<Self>>;
}
// helper to build RosyType from type rule
pub fn build_type (pair: pest::iterators::Pair<Rule>) -> Result<(RosyType, Vec<Expr>)> {
    ensure!(pair.as_rule() == Rule::r#type, 
        "Expected `type` rule when building type, found: {:?}", pair.as_rule());
        
    let mut inner_pair = pair.into_inner();
    let type_str = inner_pair.next()
        .context("Missing type string when building var decl!")?
        .as_str().to_string();
    let mut dimensions: Vec<Expr> = Vec::new();
    while let Some(dim_pair) = inner_pair.next() {
        let expr = Expr::from_rule(dim_pair)
            .context("Failed to build dimension expression in variable declaration!")?
            .ok_or_else(|| anyhow::anyhow!("Expected expression in variable declaration"))?;
        dimensions.push(expr);
    }

    let base_type: RosyBaseType = type_str
        .as_str()
        .try_into()
        .with_context(|| format!("Unknown type: {type_str}"))?;
    let r#type = RosyType::new(base_type, dimensions.len()); 

    Ok((r#type, dimensions))
}