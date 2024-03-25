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


/// `PropositionTrait` is a trait for logical and mathematical propositions.
/// It provides the basic structure for creating and managing propositions.
pub trait PropositionTrait {
    /// Constructs a new proposition from a given string.
    ///
    /// # Arguments
    /// - `src` - A string representation of the proposition.
    ///
    /// # Returns
    /// A result containing the new proposition or an error message.
    fn new(src: impl Into<String>) -> Result<Self, String> where Self: Sized;
    
    // Additional methods to be implemented in the future.
    // fn get_function(&self) -> String;

    // fn get_ast(&self) -> AST;

    // fn get_postfix_string(&self) -> String;
    // fn get_prefix_string(&self) -> String;
    // fn get_infix_string(&self) -> String;
}


/// `LogicPTrait` extends `PropositionTrait` with methods specific to logic propositions.
pub trait LogicPTrait: PropositionTrait {
    /// Evaluates the logic proposition with the given variables.
    ///
    /// # Arguments
    /// - `variables` - A vector of boolean values representing the variables in the proposition.
    ///
    /// # Returns
    /// The result of the logical evaluation as a boolean.
    fn evaluate(&self, variables: Vec<bool>) -> bool;

    // Logic-specific methods.
    fn get_truth_table_string(&self) -> String;
    fn get_kmap_string(&self) -> String;
    // fn get_result_vec(&mut self) -> Vec<bool>;
}


/// `MathPTrait` extends `PropositionTrait` with methods specific to mathematical propositions.
pub trait MathPTrait: PropositionTrait {
    /// Evaluates the mathematical proposition.
    ///
    /// # Returns
    /// The result of the evaluation as a floating-point number.
    fn evaluate(&self) -> f64;

    // /// Get the plot points of the function
    // fn get_plot_points(&self, x_range: (f64, f64), delta: f64) -> Vec<(f64, f64)>;
    // Math-specific methods.
}


/// Macro to implement the `PropositionTrait` for a given proposition type.
///
/// # Arguments
/// - `$prop_type`: The type of the proposition.
/// - `$op_type`: The type of the operations used in the proposition.
macro_rules! impl_proposition {
    ($prop_type:ident, $op_type:ty) => {
        /// A struct representing a specific type of proposition.
        #[derive(Debug, Clone, PartialEq)]
        pub struct $prop_type {
            token_table: Vec<Token<$op_type>>,
            variables: Vec<Term<$op_type>>,
            pub function: String,
        }

        /// Implementation of `PropositionTrait` for `$prop_type`.
        impl PropositionTrait for $prop_type {
            /// Creates a new proposition from a string source.
            ///
            /// # Arguments
            /// - `src` - A string representation of the proposition.
            ///
            /// # Returns
            /// A result containing the new proposition or an error message.
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


// Specific implementations of the `PropositionTrait` for different types of propositions.
// impl_proposition!(Logic);
// impl_proposition!(Math);
impl_proposition!(LogicProposition, LogicOp);
impl_proposition!(MathProposition, MathOp);
