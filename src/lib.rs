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


// ? Pub Lib Modules ----------------------------------------------------------------------------------------------------------

mod components;
pub use components::*;
pub use components::operators::*;  // Import the Operator enum because it is used in the AST struct

mod util;
pub use util::terminal;

mod circuits;
pub use circuits::*;


// ? Tests --------------------------------------------------------------------------------------------------------------------

mod proto;  // Hidden module for proto-type tests (for development only)

#[cfg(test)]  // Only compiles when running tests
mod tests {
    // This modules will be used in the tests, not in the library
    // So it's not necessary to import them in the library
    use crate::ast::Ast;
    use crate::components::proposition::Proposition;

    use crate::proto::base_change::*;


    /// Check if the check_pair_brackets() fn works well.
    #[test]  // Indicates that this is a test
    fn test_pair_brackets() {
    vec![
        ("A & (B & C)", true),
        ("A & (B & C) & D", true),
        ("(a + b) * (c - d)", true),
        ("(a + b) * (c - d]", false),
        ("x + y] * [z - w]", false),
        ("x + y] * [z - w)", false),
        ("1, 2, 3, 4}", false),
        ("1, 2, 3, 4]", false),
        ("html></html>", false),
        ("html></htm>", false),
        ("(", false),
        ("[", false),
        ("{", false),
        ("<", false),
        ("[{()}]", true),
        ("{[()]}>", false),
        (" ", true),
        ("", true),
    ].iter().for_each(|(src, result)| assert_eq!(Ast::new(src).check_pair_brackets(), *result));
    }


    /// Test if the notation for the AST is correct.
    #[test]
    fn test_ast_notation() {
        vec![// Test if the AST matches it's prefix, infix and postfix notation
            ("A+B", "+AB", "A+B", "AB+"),
            ("", "", "", ""),
            ("", "", "", ""),
            ("", "", "", ""),
            ("", "", "", ""),
            ("", "", "", ""),
        ].iter().for_each(|(src, prefix, infix, postfix)| {
            let mut ast = Ast::new(src);  // Create a new AST
            assert_eq!(ast.get_prefix_string(), prefix.to_string());
            assert_eq!(ast.get_infix_string(), infix.to_string());
            assert_eq!(ast.get_postfix_string(), postfix.to_string());
        });
    }


    #[test]
    fn test_logic_evalaution() {
        vec![
            ("A & B", 15),
            ("A ^ B", 15),
            ("A | B", 7),
            ("A", 2),
            ("A & B & C", 128),
            ("A + B + C", 255),
        ].iter().for_each(|(src, result)| assert_eq!(Proposition::new(src).evaluate_logic(), *result));
    }


    #[test]
    fn test_math_evaluation() {
        // 2 variables
        assert_eq!(Proposition::new("A*B").evaluate_math(vec![2.0, 3.0]), 6.0);
        assert_eq!(Proposition::new("A/B").evaluate_math(vec![24.0, 6.0]), 4.0);
        assert_eq!(Proposition::new("A+B").evaluate_math(vec![21.0, 57.0]), 78.0);
        assert_eq!(Proposition::new("A-B").evaluate_math(vec![21.0, 57.0]), -36.0);
        assert_eq!(Proposition::new("A*B").evaluate_math(vec![21.0, 57.0]), 1197.0);
        // 3 variables
        assert_eq!(Proposition::new("A/B").evaluate_math(vec![21.0, 57.0]), 0.3684210526315789);
        assert_eq!(Proposition::new("A+B+C").evaluate_math(vec![21.0, 57.0, 12.0]), 90.0);

        // todo: some random tests with random numbers..
    }


    // * PROTO-TYPE TESTS =====================================================================================================


    #[test]  // Indicates that this is a test
    fn test_base_change() {

        pub fn change_from_base(n: u64, n_base: u64, new_base: u64) -> String {
            let mut n = n;  // copy n
            let mut result = String::new();  // create an empty string
            while n > 0 {  // while n is greater than 0
                let rem = n % new_base;  // get the remainder
                n /= new_base;  // divide n by the new base
                // push the remainder to the result string (as a char)
                result.push("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(rem as usize).unwrap_or('?'))
            }
            result.chars().rev().collect::<String>()  // reverse the result and return it
        }

        vec![  // binary to decimal
            ("11011100", "220"),
            ("110011", "51"),
            ("11001100", "204"),
            ("11110011", "243"),
            ("1100111", "103"),
        ].iter().for_each(|(n, r)| assert_eq!(r, &change_from_base(u64::from_str_radix(n, 2).unwrap(), 2, 10)));
        vec![  // decimal to bin
            ("197", "11000101"), 
            ("253", "11111101"), 
            ("79", "1001111"), 
            ("297", "100101001"), 
            ("528", "1000010000")
        ].iter().for_each(|(n, r)| assert_eq!(r, &change_from_base(u64::from_str_radix(n, 10).unwrap(), 10, 2)));
        vec![  // bin to hex
            ("100111011", "13B"),
            ("11011011", "DB"),
            ("101111011", "17B"),
            ("11011001", "D9"),
            ("111011101", "1DD"),
        ].iter().for_each(|(n, r)| assert_eq!(r, &change_from_base(u64::from_str_radix(n, 2).unwrap(), 2, 16)));
        vec![  // hex to bin
            ("9F", "10011111"),
            ("9BAF", "1001101110101111"),
            ("8BCD", "1000101111001101"),
            ("72BA", "111001010111010"),
            ("987", "100110000111"),
            ("9F27", "1001111100100111"),
        ].iter().for_each(|(n, r)| assert_eq!(r, &change_from_base(u64::from_str_radix(n, 16).unwrap(), 16, 2)));
        vec![  // binary to octal
            ("11011001", "331"),
            ("100111001", "471"),
            ("11100110", "346"),
            ("11001100", "314"),
            ("1101110", "156"),
        ].iter().for_each(|(n, r)| assert_eq!(r, &change_from_base(u64::from_str_radix(n, 2).unwrap(), 2, 8)));
        vec![  // octal to binary
            ("245", "10100101"),
            ("327", "11010111"),
            ("651", "110101001"),
        ].iter().for_each(|(n, r)| assert_eq!(r, &change_from_base(u64::from_str_radix(n, 8).unwrap(), 8, 2)));
    }

}
