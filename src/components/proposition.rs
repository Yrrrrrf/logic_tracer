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
    Term, TokenOrTerm,
    // TokenOrTerm,
};


/// Checks if the brackets in a given slice of tokens are correctly paired.
///
/// This function iterates through the slice of tokens and uses a stack to keep track
/// of the open brackets. It checks for three conditions:
/// 1. If the bracket types of the open and close brackets match.
/// 2. If there is an unmatched closing bracket.
/// 3. If there is an unmatched opening bracket.
///
/// # Arguments
/// - `tokens` - A slice of [`Token<T>`] where `T` must implement the `Operator` trait.
///
/// # Returns
/// - [`Ok(())`] if all brackets are correctly paired.
/// - [`Err(String)`] if there is a mismatch or an unmatched bracket, with a message indicating the issue.
///
/// # Type Parameters
/// - `T`: The type parameter constrained by the `Operator` trait, used in [`Token<T>`].
///
/// # Errors
/// This function returns an error if:
/// - There is a mismatch in the type of brackets.
/// - An opening bracket is not closed.
/// - A closing bracket does not have a matching opening bracket.
fn check_pair_brackets<T>(tokens: &[Token<T>]) -> Result<(), String>
where
    T: Operator,
{
    let mut stack = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::OpenBracket(bracket_type) => stack.push((bracket_type, i)),
            Token::CloseBracket(bracket_type) => {
                match stack.pop() {
                    Some((open_bracket_type, open_index)) => {
                        if open_bracket_type != bracket_type {
                            Err(format!("Mismatched brackets at positions {} and {}", open_index, i))?
                        }
                    },
                    None => {Err(format!("Unmatched closing bracket at position {}", i))?}
                }
            },
            _ => (),  // Ignore non-bracket tokens
        }
    }
    match stack.pop() {
        Some((_, open_index)) => Err(format!("Unmatched opening bracket at position {}", open_index))?,
        None => Ok(()),
    }
}


/// Checks if the provided token slice contains any variable or number tokens.
///
/// This function iterates through the slice of tokens and verifies if there are any
/// tokens that are either variables or numbers. It's useful for validating the presence
/// of these types of tokens in scenarios where their existence is necessary for further processing.
///
/// # Arguments
/// - `tokens` - A slice of [`Token<T>`] where `T` must implement the `Operator` trait.
///
/// # Returns
/// - [`Ok(())`] if the slice contains at least one variable or number token.
/// - [`Err(String)`] if no variable or number tokens are found.
///
/// # Type Parameters
/// - `T`: The type parameter constrained by the `Operator` trait, used in [`Token<T>`].
///
/// # Errors
/// This function returns an error if no variable or number tokens are present in the slice.
fn check_var_and_num<T>(tokens: &[Token<T>]) -> Result<(), String>
where
    T: Operator,
{
    match tokens.iter().any(|token| matches!(token, Token::Variable(_) | Token::Number(_))) {
        true => Ok(()),
        false => Err("No variables or numbers found".to_string())?,
    }
}


// ? PROPOSITONS TRAITS =============================================

pub trait PropositionTrait {
    fn new(src: impl Into<String>) -> Result<Self, String> where Self: Sized;
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

// ? PROPOSITIONS IMPL ========================================================

macro_rules! impl_proposition {
    ($prop_type:ident, $op_type:ty) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $prop_type {
            pub function: String,
            token_table: Vec<Token<$op_type>>,
            variables: Vec<Term<$op_type>>,
        }

        // Implement the PropositionTrait
        impl PropositionTrait for $prop_type {
            fn new(src: impl Into<String>) -> Result<Self, String> {
                let mut token_table = Vec::new();

                for (i, c) in src.into().chars().enumerate().filter(|&(_, c)| !c.is_whitespace() && !c.is_ascii_control()) {
                    let token = Token::<$op_type>::from(c);
                    if let Token::Invalid(_) = token {Err(format!("Invalid token: '{c}' at position {i}"))?}
                    token_table.push(token);
                }

                check_pair_brackets(&token_table)?; 
                check_var_and_num(&token_table)?;

                let t_1 = Term::<$op_type>::parse_from_tokens_vec(token_table.clone());
                let t_2 = Term::<$op_type>::parse_proposition(&t_1);
                println!("{:#?}", t_2);

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


impl<T> Term<T> where T: Operator {
    pub fn parse_proposition(proposition: &[TokenOrTerm<T>]) -> Result<bool, String> 
    where
        T: Operator,
    {
        if proposition.is_empty() {
            return Err("Empty proposition".to_string());
        }

        let mut previous_token: Option<&TokenOrTerm<T>> = None;

        for (i, token_or_term) in proposition.iter().enumerate() {
            match token_or_term {
                TokenOrTerm::SingleToken(token) => {
                    // if i == 0 && !matches!(token, Token::OpenBracket(_) | Token::Operator(op) if op == &T::NEGATOR) {
                    if i == 0 && !matches!(token, Token::OpenBracket(_)) {
                        return Err("Invalid start of proposition".to_string());
                    }

                    if let Token::CloseBracket(_) = token {
                        if i + 1 < proposition.len() {
                            if let TokenOrTerm::SingleToken(next_token) = &proposition[i + 1] {
                                if !matches!(next_token, Token::OpenBracket(_) | Token::Operator(_)) {
                                    return Err(format!("Invalid token following a close bracket at position {}", i));
                                }
                            }
                        }
                    }

                    if let Token::Operator(_) = token {
                        if let Some(TokenOrTerm::SingleToken(prev_token)) = previous_token {
                            if matches!(prev_token, Token::Operator(op) if *op != T::NEGATOR) {
                                return Err(format!("Invalid sequence of operators at position {}", i));
                            }
                        }
                    }
                },
                TokenOrTerm::Term(_) => {
                    if i == 0 { continue; }
                },
            }
            previous_token = Some(token_or_term);
        }

        Ok(true)
    }
}

