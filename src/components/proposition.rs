//! This module defines the `Propositions` enum, which represents various logical propositions.
//! It is used for modeling logical statements in Rust programs.
//! Import necessary modules and dependencies if needed.
//! The `Propositions` enum represents different logical propositions.

use dev_utils::console::format::set_fg;


use crate::{components::alphabet::Token, LogicOp, BracketType};


/// Check if the brackets are paired
/// Also check if the brackets are in the correct order
// pub fn check_pair_brackets(src: &Vec<char>) -> bool {
//     let mut stack: Vec<char> = Vec::new();
//     let brackets: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

//     for char in src.clone() {
//         match char {
//             ch if brackets.iter().any(|(open, _)| open == &ch) => stack.push(char),
//             ch if brackets.iter().any(|(_, close)| close == &ch) => {
//                 if let Some(top) = stack.pop() {
//                     if brackets.iter().any(|&(open, close)| open == top && close == ch) {
//                         continue;  // If the brackets are paired, continue
//                     }
//                 }
//                 return false;  // If the brackets are not paired, return false
//             }
//             _ => (),
//         }
//     }
//     stack.is_empty()
// }
pub fn check_pair_brackets(tokens: &[Token<LogicOp>]) -> bool {
    let mut stack: Vec<BracketType> = Vec::new();

    for token in tokens {
        match token {
            Token::OpenBracket(bracket_type) => stack.push(*bracket_type),
            Token::CloseBracket(bracket_type) => {
                match (stack.pop(), bracket_type) {
                    (Some(opening), BracketType::Parenthesis) if opening == BracketType::Parenthesis => (),
                    (Some(opening), BracketType::Square) if opening == BracketType::Square => (),
                    (Some(opening), BracketType::Curly) if opening == BracketType::Curly => (),
                    _ => return false,
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}


// ? DEFINE THE PROPOSITONS TRAIT =============================================

// // Base trait for common functionality
pub trait PropositionTrait {
    fn new(src: impl Into<String>) -> Result<Self, &'static str> where Self: Sized;
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
    token_table: Vec<Token<LogicOp>>,
    function: String,
    variables: Vec<char>,
}

impl PropositionTrait for LogicProposition {
    fn new(src: impl Into<String>) -> Result<Self, &'static str> {
        let mut has_variable = false;
        let mut has_number = false;
        let mut token_table = Vec::new();

        // * Lexical analysis: Streamline character processing and token creation
        for c in src.into().chars().filter(|c| !c.is_whitespace() && !c.is_ascii_control()) {
            let token = Token::<LogicOp>::from(c);
            if matches!(token, Token::Invalid(_)) {
                println!("Invalid token: {c} at position {}\n", token_table.len());
                return Err("The proposition contains invalid tokens")?;
            }

            if matches!(token, Token::Variable(_)) {has_variable = true} // It contains at least one variable
            if matches!(token, Token::Number(_)) {has_number = true;}  // It contains at least one number
            token_table.push(token);
        }

        // * Syntactic analysis (Parsing)
        if !check_pair_brackets(&token_table) {return Err("The brackets are not paired")?;}
        if !(has_variable || has_number) {return Err("The proposition must contain at least one variable/number")?;}

        Ok(Self {
            token_table,
            function: "".to_string(),  // todo: add the function field
            variables: vec![],  // todo: add the variables field
        })
    }
}

impl LogicPTrait for LogicProposition {
    fn evaluate(&self, variables: Vec<bool>) -> bool {
        false
    }

    fn get_truth_table_string(&self) -> String {
        "".to_string()
    }

    fn get_kmap_string(&self) -> String {
        "".to_string()
    }

}
