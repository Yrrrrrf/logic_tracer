//! This module defines the `Propositions` enum, which represents various logical propositions.
//! It is used for modeling logical statements in Rust programs.
//! Import necessary modules and dependencies if needed.
//! The `Propositions` enum represents different logical propositions.
use crate::{
    LogicOp, 
    MathOp, 
    Token, 
    Term,
};


pub trait PropositionTrait {
    fn new(src: impl Into<String>) -> Result<Self, String> where Self: Sized;
    // fn get_function(&self) -> String;
    // todo: add this methods to the trait
    // fn get_postfix_string(&self) -> String;
    // fn get_prefix_string(&self) -> String;
    // fn get_infix_string(&self) -> String;
}

pub trait LogicPTrait: PropositionTrait {
    fn evaluate(&self, variables: Vec<bool>) -> bool;
    // Logic-specific methods
    fn get_truth_table_string(&self) -> String;
    fn get_kmap_string(&self) -> String;
    // fn get_result_vec(&mut self) -> Vec<bool>;  // get all the possible results of the proposition
    // fn get_ast_string(&self) -> String;  // get the AST of the proposition
}

pub trait MathPTrait: PropositionTrait {
    fn evaluate(&self) -> f64;
    // /// Get the plot points of the function
    // fn get_plot_points(&self, x_range: (f64, f64), delta: f64) -> Vec<(f64, f64)>;
    // Math-specific methods
}

// ? PROPOSITIONS IMPL ========================================================

macro_rules! impl_proposition {
    ($prop_type:ident, $op_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $prop_type {
            token_table: Vec<Token<$op_type>>,
            variables: Vec<Term<$op_type>>,
            pub function: String,
        }

        impl PropositionTrait for $prop_type {
            fn new(src: impl Into<String>) -> Result<Self, String> {
                let mut token_table = Vec::new();
                // * Lexical analysis
                for (i, c) in src.into().chars().enumerate().filter(|&(_, c)| !c.is_whitespace() && !c.is_ascii_control()) {
                    let token = Token::<$op_type>::from(c);
                    if let Token::Invalid(_) = token {Err(format!("Invalid token: '{c}' at position {i}"))?}
                    else {token_table.push(token)}
                }

                // * Syntactic analysis
                Term::<$op_type>::check_pair_brackets(&token_table)?; 
                Term::<$op_type>::check_var_and_num(&token_table)?;
                let function_vec = Term::<$op_type>::group_tokens_into_terms(token_table.clone())?;
                let parsed_vec = Term::<$op_type>::validate_proposition_structure(&function_vec)?;

                // ^ for debugging
                log::debug!("Token table: {:#?}", function_vec);
                log::debug!("Parsed vector: {:#?}", parsed_vec);

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
