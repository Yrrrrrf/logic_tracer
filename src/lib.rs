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

pub mod util;  // ^ THIS SHOULD BE PRIVATE, BUT IT'S PUBLIC FOR TESTING PURPOSES

mod circuits;
// pub use circuits::*;

// ? Tests --------------------------------------------------------------------------------------------------------------------

mod proto;  // Hidden module for proto-type tests (for development only)
// pub use proto::*;  // Make visible (for documentation) the proto-type module


#[cfg(test)]  // Only compiles when running tests
mod tests {
    // This modules will be used in the tests, not in the library
    // So it's not necessary to import them in the library
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
        ].iter().for_each(|(src, result)| assert_eq!(crate::proposition::check_pair_brackets(src), *result));
    }


    /// Test if the notation for the AST is correct.
    #[test]
    fn test_ast_notation() {
        vec![  // Test if the AST matches it's infix, prefix and postfix notation
            ("A+B", "+AB", "AB+"),
            ("(X+Y)*Z", "*+XYZ", "XY+Z*"),
            ("A+(B*C)", "+A*BC", "ABC*+"),
            ("(A+B)*(C+D)", "*+A B+CD", "AB+CD+*"),
            ("A*(B+(C*D))", "*A+B*CD", "ABCD*+*"),
            ("(A*B)+(C*D)", "+*AB*CD", "AB*CD*+"),
        ].iter().for_each(|(src, prefix, postfix)| {
            let mut ast = Proposition::new(src);  // Create a new AST
            assert_eq!(ast.get_prefix_string(), prefix.to_string());
            assert_eq!(ast.get_infix_string(), src.to_string());  // src == infix
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
        vec![  // vec![src_base, new_base, src, result]
            // bin -> dec
            (2, 10, "11011100", "220"),
            (2, 10, "110011", "51"),
            (2, 10, "11001100", "204"),
            (2, 10, "11110011", "243"),
            (2, 10, "1100111", "103"),
            // dec -> bin
            (10, 2, "197", "11000101"),
            (10, 2, "253", "11111101"),
            (10, 2, "79", "1001111"),
            (10, 2, "297", "100101001"),
            (10, 2, "528", "1000010000"),
            // bin -> hex
            (2, 16, "100111011", "13B"),
            (2, 16, "11011011", "DB"),
            (2, 16, "101111011", "17B"),
            (2, 16, "11011001", "D9"),
            (2, 16, "111011101", "1DD"),
            // hex -> bin
            (16, 2, "9F", "10011111"),
            (16, 2, "9BAF", "1001101110101111"),
            (16, 2, "8BCD", "1000101111001101"),
            (16, 2, "72BA", "111001010111010"),
            (16, 2, "987", "100110000111"),
            (16, 2, "9F27", "1001111100100111"),
            // bin -> oct
            (2, 8, "11011001", "331"),
            (2, 8, "100111001", "471"),
            (2, 8, "11100110", "346"),
            (2, 8, "11001100", "314"),
            (2, 8, "1101110", "156"),
            // oct -> bin
            (8, 2, "245", "10100101"),
            (8, 2, "327", "11010111"),
            (8, 2, "651", "110101001"),

            // ? Decimal numbers test
            // These aproximate numbers are not exact because of the floating point precision
            // So the result is not exact, but it's close enough
            // The str_to_num_from_base() fn returns the last number that is not 0. So the result is not exact
            // Example: 0.102000 -> 0.102 (the last 0s are not returned)
            (10, 2, "450.5", "111000010.1"),
            (10, 2, "8.5", "1000.1"),
            (10, 8, "450.5", "702.4"),
            (10, 8, "7.5", "7.4"),
            (10, 16, "450.5", "1C2.8"),
            (10, 16, "8.5", "8.8"),
            (8, 10, "450.5", "296.625"),
            (8, 10, "7.5", "7.625"),
            (2, 10, "1010.1", "10.5"),
            (2, 10, "1010", "10"),
            (20, 6, "AA.21", "550.03405012"),
            (10, 16, "2197.42", "895.6B851EB8"),
            (16, 10, "9E.D", "158.8125"),

        ].iter().for_each(|(src_base, new_base, src, result)|
            assert_eq!(str_to_num_from_base(src, *src_base, *new_base).unwrap(), result.to_string()));

        // * To print the results in the terminal
        // ].iter().for_each(|(src_base, new_base, src, result)|
        //     println!("{} b{:_>2} = {} b{:_>2}\t{}", src, src_base, 
        //         str_to_num_from_base(src, *src_base, *new_base).unwrap(), new_base,
        //         crate::terminal::set_fg(result, if str_to_num_from_base(src, *src_base, *new_base).unwrap() == result.to_string() {"g"} else {"r"})
        // ));
    }

}
