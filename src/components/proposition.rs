//! This module defines the `Propositions` enum, which represents various logical propositions.
//! It is used for modeling logical statements in Rust programs.
//! Import necessary modules and dependencies if needed.
//! The `Propositions` enum represents different logical propositions.

use crate::{
    BracketType, 
    Operator,
    LogicOp, 
    MathOp, 
    Token, 
};


pub fn check_pair_brackets<T>(tokens: &[Token<T>]) -> bool 
where
    T: Operator, // Ensure T is an Operator, which is a requirement for Token<T>
{
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

// Base trait for common functionality
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


// Inherit from the base trait and add specific methods
pub trait MathPTrait: PropositionTrait {
    fn evaluate(&self) -> f64;
    // /// Get the plot points of the function
    // fn get_plot_points(&self, x_range: (f64, f64), delta: f64) -> Vec<(f64, f64)>;
    // Math-specific methods
}

// ? PROPOSITIONS ========================================================

macro_rules! impl_proposition {
    ($prop_type:ident, $op_type:ty) => {
        // Define the struct
        #[derive(Debug, Clone, PartialEq)]
        pub struct $prop_type {
            token_table: Vec<Token<$op_type>>,
            function: String,
            variables: Vec<char>,
        }

        // Implement the PropositionTrait
        impl PropositionTrait for $prop_type {
            fn new(src: impl Into<String>) -> Result<Self, &'static str> {
                let mut has_variable = false;
                let mut has_number = false;
                let mut token_table = Vec::new();

                for c in src.into().chars().filter(|c| !c.is_whitespace() && !c.is_ascii_control()) {
                    let token = Token::<$op_type>::from(c);
                    if matches!(token, Token::Invalid(_)) {
                        println!("Invalid token: '{c}' at position {}\n", token_table.len());
                        return Err("The proposition contains invalid tokens");
                    }
                    if matches!(token, Token::Variable(_)) {has_variable = true}
                    if matches!(token, Token::Number(_)) {has_number = true;}
                    token_table.push(token);
                }

                if !check_pair_brackets(&token_table) {
                    return Err("The brackets are not paired");
                }
                if !(has_variable || has_number) {
                    return Err("The proposition must contain at least one variable/number");
                }

                Ok(Self {
                    token_table,
                    function: "".to_string(),
                    variables: vec![],
                })
            }
        }
    };
}

impl_proposition!(LogicProposition, LogicOp);
impl_proposition!(MathProposition, MathOp);


// impl LogicPTrait for LogicProposition {
//     fn evaluate(&self, variables: Vec<bool>) -> bool {
//         false
//     }

//     fn get_truth_table_string(&self) -> String {
//         "".to_string()
//     }

//     fn get_kmap_string(&self) -> String {
//         "".to_string()
//     }

// }
