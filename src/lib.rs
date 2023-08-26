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
#![allow(unused)]


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
    grammar,
    proposition,
};
pub use components::operators::Operator;

/// Oseas
mod util {
    pub mod terminal;
}
// pub use util::*;


mod circuits;
pub use {
    circuits::circuit,
    circuits::combinational,
    circuits::sequential
};


// ? Tests ----------------------------------------------------------------------------------------------------------------------------------------------------------------


#[cfg(test)]  // Only compiles when running tests
mod tests {
    use crate::{lexer::Lexer, parser::Parser};
    use crate::components::proposition::Proposition;
    
    #[test]  // Indicates that this is a test
    fn test_pair_brackets() {
        assert_eq!(Lexer::new("A & (B & C)").check_pair_brackets(), true);
        assert_eq!(Lexer::new("A & (B & C) & D").check_pair_brackets(), true);
        assert_eq!(Lexer::new("(a + b) * (c - d)").check_pair_brackets(), true);
        assert_eq!(Lexer::new("(a + b) * (c - d]").check_pair_brackets(), false);
        assert_eq!(Lexer::new("x + y] * [z - w]").check_pair_brackets(), false);
        assert_eq!(Lexer::new("x + y] * [z - w)").check_pair_brackets(), false);
        assert_eq!(Lexer::new("1, 2, 3, 4}").check_pair_brackets(), false);
        assert_eq!(Lexer::new("1, 2, 3, 4]").check_pair_brackets(), false);
        assert_eq!(Lexer::new("html></html>").check_pair_brackets(), false);
        assert_eq!(Lexer::new("html></htm>").check_pair_brackets(), false);
        assert_eq!(Lexer::new("(").check_pair_brackets(), false);
        assert_eq!(Lexer::new("[").check_pair_brackets(), false);
        assert_eq!(Lexer::new("{").check_pair_brackets(), false);
        assert_eq!(Lexer::new("<").check_pair_brackets(), false);
        assert_eq!(Lexer::new("[{()}]").check_pair_brackets(), true);
        assert_eq!(Lexer::new("{[()]}>").check_pair_brackets(), false);
        assert_eq!(Lexer::new(" ").check_pair_brackets(), true);
        assert_eq!(Lexer::new("").check_pair_brackets(), true);
    }


    #[test]  // Indicates that this is a test
    fn test_logic_evalaution() {
        assert_eq!(Proposition::new("A & B").evaluate_logic().unwrap(), vec![false, false, false, true]);
        assert_eq!(Proposition::new("A | B").evaluate_logic().unwrap(), vec![false, true, true, true]);
        assert_eq!(Proposition::new("A ^ B").evaluate_logic().unwrap(), vec![false, true, true, false]);
    }


    #[test]  // Indicates that this is a test
    fn test_math_evaluation() {
        assert_eq!(Proposition::new("21 + 57").evaluate_math().unwrap(), vec![78.0]);
        assert_eq!(Proposition::new("21 - 57").evaluate_math().unwrap(), vec![-36.0]);
        assert_eq!(Proposition::new("21 * 57").evaluate_math().unwrap(), vec![1197.0]);
    }



}
