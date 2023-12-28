//! This module defines the `Propositions` enum, which represents various logical propositions.
//! It is used for modeling logical statements in Rust programs.
//! Import necessary modules and dependencies if needed.
//! The `Propositions` enum represents different logical propositions.

use dev_utils::console::format::set_fg;

use crate::{Operator, LogicOp, MathOp, RelOp};
use crate::components::grammar::GrammarToken;


/// Check if the brackets are paired
/// Also check if the brackets are in the correct order
pub fn check_pair_brackets(src: &Vec<char>) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let brackets: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

    for char in src.clone() {
        match char {
            ch if brackets.iter().any(|(open, _)| open == &ch) => stack.push(char),
            ch if brackets.iter().any(|(_, close)| close == &ch) => {
                if let Some(top) = stack.pop() {
                    if brackets.iter().any(|&(open, close)| open == top && close == ch) {
                        continue;  // If the brackets are paired, continue
                    }
                }
                return false;  // If the brackets are not paired, return false
            }
            _ => (),
        }
    }
    stack.is_empty()
}

// ? DEFINE THE PROPOSITONS TRAIT =============================================

// // Base trait for common functionality
pub trait PropositionTrait {

    fn new(src: &str) -> Self where Self: Sized;
    // fn get_function(&self) -> String;
    // todo: add this methods to the trait
    // fn get_postfix_string(&self) -> String;
    // fn get_prefix_string(&self) -> String;
    // fn get_infix_string(&self) -> String;
}

// Inherit from the base trait and add specific methods
pub trait LogicPTrait: PropositionTrait {
    fn evaluate(&self, variables: Vec<bool>) -> bool;
    // Logic-specific methods
    fn get_truth_table_string(&self) -> String;
    fn get_kmap_string(&self) -> String;
    // fn get_result_vec(&mut self) -> Vec<bool>;  // get all the possible results of the proposition
    // fn get_ast_string(&self) -> String;  // get the AST of the proposition
}


// // Inherit from the base trait and add specific methods
// pub trait MathPTrait: PropositionTrait {
//     fn evaluate(&self) -> f64;
//     // /// Get the plot points of the function
//     // fn get_plot_points(&self, x_range: (f64, f64), delta: f64) -> Vec<(f64, f64)>;
//     // Math-specific methods
// }

// ? PROPOSITIONS ========================================================

// * Logic Propositions
#[derive(Debug, Clone, PartialEq)]
pub struct LogicProposition {
    token_table: Vec<GrammarToken<LogicOp>>,
    function: String,
    variables: Vec<char>,
}

impl LogicProposition {
    pub fn new(src: impl Into<String>) -> Result<Self, &'static str> {

        // * Lexical analysis
        let src_vec: Vec<char> = src.into().chars()  // Remove whitespace and control characters from the expression
            .filter(|c| !c.is_whitespace() && !c.is_ascii_control()).collect();

        // * Syntactic analysis
        match check_pair_brackets(&src_vec) {
            true => (),
            false => Err("The brackets are not paired")?,
        }
        // match src_vec.iter().any(|c| !c.is_ascii_alphabetic()) {
        //     true => (),
        //     false => Err("The proposition must contain at least one variable")?,
        // }

        // * Semantic analysis
        // match check_syntax(&src_vec) {
        //     true => (),
        //     false => Err("The proposition is not valid")?,
        // }

        // * Build the proposition (Parse the expression)
        let token_table: Vec<GrammarToken<LogicOp>> = src_vec.iter()
            .map(|c| GrammarToken::<LogicOp>::from(*c)).collect::<Vec<GrammarToken<LogicOp>>>();

        Ok(Self {
            token_table,
            function: "".to_string(),  // todo: add the function field
            variables: vec![],  // todo: add the variables field
        })
    }

}

