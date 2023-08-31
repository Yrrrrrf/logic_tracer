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


// ? Modules ----------------------------------------------------------------------------------------------------------------------

mod components;
pub use components::*;
pub use components::operators::Operator;  // Import the Operator enum because it is used in the AST struct

mod util;
pub use util::terminal;

mod circuits;
pub use circuits::*;


// ? Tests ------------------------------------------------------------------------------------------------------------------------

mod proto;

#[cfg(test)]  // Only compiles when running tests
mod tests {
    use crate::ast::Ast;
    use crate::components::proposition::Proposition;

    
    use crate::proto::base_change::change_from_base;

    /// Check if the check_pair_brackets() fn works well.
    #[test]  // Indicates that this is a test
    fn test_pair_brackets() {
        assert_eq!(Ast::new("A & (B & C)").check_pair_brackets(), true);
        assert_eq!(Ast::new("A & (B & C) & D").check_pair_brackets(), true);
        assert_eq!(Ast::new("(a + b) * (c - d)").check_pair_brackets(), true);
        assert_eq!(Ast::new("(a + b) * (c - d]").check_pair_brackets(), false);
        assert_eq!(Ast::new("x + y] * [z - w]").check_pair_brackets(), false);
        assert_eq!(Ast::new("x + y] * [z - w)").check_pair_brackets(), false);
        assert_eq!(Ast::new("1, 2, 3, 4}").check_pair_brackets(), false);
        assert_eq!(Ast::new("1, 2, 3, 4]").check_pair_brackets(), false);
        assert_eq!(Ast::new("html></html>").check_pair_brackets(), false);
        assert_eq!(Ast::new("html></htm>").check_pair_brackets(), false);
        assert_eq!(Ast::new("(").check_pair_brackets(), false);
        assert_eq!(Ast::new("[").check_pair_brackets(), false);
        assert_eq!(Ast::new("{").check_pair_brackets(), false);
        assert_eq!(Ast::new("<").check_pair_brackets(), false);
        assert_eq!(Ast::new("[{()}]").check_pair_brackets(), true);
        assert_eq!(Ast::new("{[()]}>").check_pair_brackets(), false);
        assert_eq!(Ast::new(" ").check_pair_brackets(), true);
        assert_eq!(Ast::new("").check_pair_brackets(), true);
    }


    #[test]  // Indicates that this is a test
    fn test_logic_evalaution() {
        assert_eq!(Proposition::new("A & B").evaluate_logic(), vec![false, false, false, true]);
        assert_eq!(Proposition::new("A | B").evaluate_logic(), vec![false, true, true, true]);
        assert_eq!(Proposition::new("A ^ B").evaluate_logic(), vec![false, true, true, false]);
    }


    #[test]  // Indicates that this is a test
    fn test_math_evaluation() {
        assert_eq!(Proposition::new("21 + 57").evaluate_math(), vec![78.0]);
        assert_eq!(Proposition::new("21 - 57").evaluate_math(), vec![-36.0]);
        assert_eq!(Proposition::new("21 * 57").evaluate_math(), vec![1197.0]);
    }


    #[test]  // Indicates that this is a test
    fn test_kmap() {
        assert_eq!(Proposition::new("A & B").show_kmap(), Ok(vec![vec![false, false], vec![false, true]]));
        assert_eq!(Proposition::new("A | B").show_kmap(), Ok(vec![vec![false, true], vec![true, true]]));
        assert_eq!(Proposition::new("A ^ B").show_kmap(), Ok(vec![vec![false, true], vec![true, false]]));
        assert_eq!(Proposition::new("A ⊕ B").show_kmap(), Ok(vec![vec![false, true], vec![true, false]]));
        assert_eq!(Proposition::new("A ⊻ B").show_kmap(), Ok(vec![vec![false, true], vec![true, false]]));
        assert_eq!(Proposition::new("A ⊼ B").show_kmap(), Ok(vec![vec![false, false], vec![false, false]]));
        assert_eq!(Proposition::new("A ⊽ B").show_kmap(), Ok(vec![vec![false, false], vec![false, false]]));
    }


    // * PROTO-TYPE TESTS =====================================================================================================


    #[test]  // Indicates that this is a test
    fn test_base_change() {
        vec![  // decimal to binary
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
