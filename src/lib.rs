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

// mod operators;
// pub use operators::Operator;


/// Oseas
mod util {
    pub mod terminal;
}
// pub use util::*;

mod circuits {
    pub mod combinational;
    pub mod sequential;
}
// pub use circuits::*;

mod proposition;
pub use proposition::*;





// ? Tests ----------------------------------------------------------------------------------------------------------------------------------------------------------------

#[cfg(test)]  // Only compiles when running tests
mod tests {
    // use crate::parser::*;


    #[test]  // Indicates that this is a test
    fn parse_test_01() {
        // let mut parse = Parser::new("A & B");
        // lexer.pair_brackets();

        assert_eq!(2 + 2, 4);
    }
    

    #[test]  // Indicates that this is a test
    fn test_test2() {
        assert_eq!(2 + 2, 4);
    }
}