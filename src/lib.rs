//! # Logic Tracer
//! 
//! `logic_tracer` is a library for tracing the logic of a logic propopsition.
//! 
//! Also creates a truth table and a Karnaugh map (if possible).
//! 
//! ## Features
//! 
//! - Creates a truth table
//! - Creates a Karnaugh map
//! - Creates a logic trace
//! - Reduces the logic proposition to its simplest form
//! 


/// # Logic Tracer
/// 
/// `logic_tracer` is a library for tracing the logic of a logic propopsition.
mod components {
    pub mod ast;
    pub mod lexer;
    // pub mod parser;
    pub mod token;
}
pub use components::*;


mod util {
    pub mod terminal;
}
pub use util::*;


mod circuits {
    pub mod combinational;
    pub mod sequential;
}

mod proposition;
pub use proposition::*;


#[cfg(test)]  // Only compiles when running tests
mod tests {

    #[test]  // Indicates that this is a test
    fn test_eq() {
        assert_eq!(2 + 2, 4);
    }
    

    #[test]  // Indicates that this is a test
    fn test_test2() {
        assert_eq!(2 + 2, 4);
    }
}