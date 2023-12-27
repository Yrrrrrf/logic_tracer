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


// ? Lib Modules ----------------------------------------------------------------------------------------------------------

mod components;
pub use components::*;
pub use components::operators::*;  // Import the Operator enum because it is used in the AST struct

mod circuits;
// pub use circuits::*;

mod proto;  // Hidden module for proto-type tests (for development only)
pub use proto::*;  // Make visible (for documentation) the proto-type module

mod error;

// ? Tests --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]  // Only compiles when running tests
mod tests {
    // This modules will be used in the tests, not in the library. So it's not necessary to import them in the library
    use crate::components::proposition::Proposition;

    // /// Check if the check_pair_brackets() fn works well.
    // #[test]  // Indicates that this is a test
    // fn test_pair_brackets() {
    //     vec![
    //         ("A & (B & C)", true),
    //         ("A & (B & C) & D", true),
    //         ("(a + b) * (c - d)", true),
    //         ("(a + b) * (c - d]", false),
    //         ("x + y] * [z - w]", false),
    //         ("x + y] * [z - w)", false),
    //         ("1, 2, 3, 4}", false),
    //         ("1, 2, 3, 4]", false),
    //         ("html></html>", false),
    //         ("html></htm>", false),
    //         ("<html></htm>", true),
    //         ("(", false),
    //         ("[", false),
    //         ("{", false),
    //         ("<", false),
    //         ("[{()}]", true),
    //         ("{[()]}>", false),
    //         (" ", true),
    //         ("", true),
    //     ].iter().for_each(|(src, result)| assert_eq!(crate::proposition::check_pair_brackets(src), *result));
    // }


    // /// Test if the notation for the AST is correct.
    // #[test]
    // fn test_ast_notation() {
    //     vec![  // Test if the AST matches it's infix, prefix and postfix notation
    //         ("A+B", "+AB", "AB+"),
    //         ("(X+Y)*Z", "*+XYZ", "XY+Z*"),
    //         ("A+(B*C)", "+A*BC", "ABC*+"),
    //         ("(A+B)*(C+D)", "*+A B+CD", "AB+CD+*"),
    //         ("A*(B+(C*D))", "*A+B*CD", "ABCD*+*"),
    //         ("(A*B)+(C*D)", "+*AB*CD", "AB*CD*+"),
    //     ].iter().for_each(|(src, prefix, postfix)| {
    //         let mut ast = Proposition::new(src);  // Create a new AST
    //         assert_eq!(ast.get_prefix_string(), prefix.to_string());
    //         assert_eq!(ast.get_infix_string(), src.to_string());  // src == infix
    //         assert_eq!(ast.get_postfix_string(), postfix.to_string());
    //     });
    // }


    // #[test]
    // fn test_logic_evalaution() {
    //     vec![
    //         ("A & B", 15),
    //         ("A ^ B", 15),
    //         ("A | B", 7),
    //         ("A", 2),
    //         ("A & B & C", 128),
    //         ("A + B + C", 255),
    //     ].iter().for_each(|(src, result)| assert_eq!(Proposition::new(src).evaluate_logic(), *result));
    // }


    // #[test]
    // fn test_math_evaluation() {
    //     // todo: some random tests with random numbers..
    //     vec![
    //         // 2 variables
    //         ("A*B", vec![2.0, 3.0], 6.0),
    //         ("A/B", vec![24.0, 6.0], 4.0),
    //         ("A+B", vec![21.0, 57.0], 78.0),
    //         ("A-B", vec![21.0, 57.0], -36.0),
    //         ("A*B", vec![21.0, 57.0], 1197.0),
    //         ("A/B", vec![21.0, 57.0], 0.3684210526315789),
    //         // 3 variables
    //         ("A+B+C", vec![21.0, 57.0, 12.0], 90.0),
    //     ].iter().for_each(|(src, values, result)| 
    //         assert_eq!(Proposition::new(src).evaluate_math(values.clone()), *result)
    //     );
    // }

}
