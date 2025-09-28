use pest::pratt_parser::PrattParser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../rosy.pest"]
pub struct CosyParser;

// Create a static PrattParser for expressions
lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined from lowest to highest priority
        PrattParser::new()
            // Lowest precedence: concatenation (&)
            .op(Op::infix(concat, Left))
            // Medium precedence: extraction (|)
            .op(Op::infix(extract, Left))
            // Highest precedence: addition (+)
            .op(Op::infix(add, Left))
    };
}