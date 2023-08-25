//! # Logic Tracer
//! 
//! `logic_tracer` is a library for tracing the logic of a logic propopsition.
//! 
//! Also creates a truth table and a Karnaugh map (if possible).
//! 
//! ## Features
//! 
//! - Creates a logic trace
//! - Creates a truth table
//! - Creates a Karnaugh map
//! - Reduces the logic proposition to its simplest form (if possible)
//! - Creates a circuit diagram (if possible)
//! - Serializes the logic proposition to a file (must be implemented)
//! 
#![allow(dead_code)]


/// # Logic Tracer
/// 
/// `logic_tracer` is a library for tracing the logic of a logic propopsition.

mod components;
pub use components::{
    lexer,
    // token,
    parser,
    ast,
    operators,
    grammar
};
pub use components::operators::Operator;

/// Oseas
mod util {
    pub mod terminal;
}
// pub use util::*;

// mod circuits {
//     pub mod combinational;
//     pub mod sequential;
// }
// pub use circuits::*;


mod circuits;
pub use {
    circuits::circuit,
    circuits::combinational,
    circuits::sequential
};


// ? Tests ----------------------------------------------------------------------------------------------------------------------------------------------------------------


#[cfg(test)]  // Only compiles when running tests
mod tests {
    // use crate::parser::*;

    // use crate::{lexer, parser::{self, Parser}};


    #[test]  // Indicates that this is a test
    fn parse_test_01() {
        let mut parse = crate::parser::Parser::new("A & B");
        // lexer.pair_brackets();
        // lexer.parse();  // Parse the tokens (It means that it will create the AST)
        // parse.parse();
        let output = parse.evaluate();
        // println!("Results = \n{:?}", output);
        assert_eq!(output, Ok(vec![false, false, false, true]));
    }
    
    
    #[test]  // Indicates that this is a test
    fn test_test2() {
        let mut parse = crate::parser::Parser::new("A & B & C");
        let output = parse.evaluate();
        assert_eq!(output, Ok(vec![false, false, false, false, false, false, false, true]));
    }
}