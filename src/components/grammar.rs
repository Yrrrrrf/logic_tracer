//! Grammar Processing Module
//!
//! This module is responsible for defining and handling the grammar of a specific language
//! or logical system. It contains structures and functions that parse, analyze, and validate
//! sequences of tokens, grouping them into meaningful units (terms) and ensuring they conform 
//! to the defined syntactical rules.
//!
//! Key Components:
//! - `TokenOrTerm`: An enum that encapsulates either a single token or a term. This allows 
//!   for flexibility in handling both individual elements and grouped sequences in expressions.
//! - `Term`: A structure representing a term, which is a sequence of tokens that collectively 
//!   form a logical or mathematical unit within an expression. Terms are essential for parsing 
//!   and interpreting more complex expressions.
//! - Parsing and Validation Functions: These functions include logic for grouping tokens into 
//!   terms based on specific rules and for validating the structure of entire propositions to 
//!   ensure they adhere to the grammar of the language or system.
//!
//! The module is designed to be versatile and adaptable, capable of being used for various 
//! types of languages or logical systems. It emphasizes clarity and robustness in parsing and 
//! validation processes.
//!
//! Examples of use cases include constructing logical propositions, parsing mathematical expressions,
//! or analyzing syntax in programming languages.
//!
//! # Examples
//! ```
// todo: Example of how to use the module's functionality...
//! ```
use std::clone;

use crate::components::{
    operators::*,
    alphabet::*,
};


/// An enumeration representing either a single token or a term.
///
/// This enum is used to encapsulate either a single token of type [`Token<T>`] or a term,
/// which is a collection of tokens that together represent a logical or mathematical unit.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenOrTerm<T> where T: Operator {
    SingleToken(Token<T>),
    Term(Term<T>),
}


/// Represents a term, which is a collection of tokens that together form a logical or mathematical unit.
///
/// This struct is used to group tokens that are collectively interpreted as a single entity in an expression.
///
/// # Fields
/// - `tokens`: A vector of [`Token<T>`] representing the tokens that make up the term.
#[derive(Clone, Debug, PartialEq)]
pub struct Term<T> where T: Operator {
    pub tokens: Vec<Token<T>>,
}

impl<T> Term<T> where T: Operator {
    /// Creates a new term from a vector of tokens.
    /// 
    /// # Arguments
    /// - `tokens` - A vector of [`Token<T>`] representing the tokens that make up the term.
    /// 
    /// # Returns
    /// Returns a new [`Term<T>`] containing the given tokens.
    pub fn from(tokens: Vec<Token<T>>) -> Self {
        Self {tokens,}
    }


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
    // todo: Add examples
    pub fn check_pair_brackets(tokens: &[Token<T>]) -> Result<(), String> {
        let mut stack = Vec::new();

        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::OpenBracket(bracket_type) => stack.push((bracket_type, i)),
                Token::CloseBracket(bracket_type) => {
                    match stack.pop() {
                        Some((open_bracket_type, open_index)) => {
                            if open_bracket_type != bracket_type {
                                Err(format!("Mismatched brackets at positions {open_index} and {i}"))?
                            }
                        },
                        None => {Err(format!("Unmatched closing bracket at position {i}"))?}
                    }
                },
                _ => (),  // Ignore non-bracket tokens
            }
        }
        match stack.pop() {
            Some((_, open_index)) => Err(format!("Unmatched opening bracket at position {open_index}"))?,
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
    // todo: Add examples
    pub fn check_var_and_num(tokens: &[Token<T>]) -> Result<(), String> {
        match tokens.iter().any(|token| matches!(token, Token::Variable(_) | Token::Number(_))) {
            true => Ok(()),
            false => Err("No variables or numbers found".to_string())?,
        }
    }


    /// Parses a vector of tokens into a vector of tokens or terms.
    ///
    /// This function processes a sequence of tokens and groups them into terms based on specific rules.
    /// A 'term' in this context is defined as a sequence of tokens that represent a logical or mathematical
    /// unit in an expression. The function identifies such sequences and groups them together.
    ///
    /// # Arguments
    /// - `src_tokens` - A vector of `Token<T>` representing the source tokens to be parsed.
    ///
    /// # Returns
    /// Returns a vector of `TokenOrTerm<T>`. Each element in the returned vector is either a single token
    /// or a term (a grouped sequence of tokens).
    ///
    /// # Rules for Grouping Tokens into Terms
    /// - A term starts with a negator or a number, followed optionally by a variable.
    /// - If a variable is encountered, it checks for an underscore followed by numbers to form a subterm.
    /// - The term ends before the next token that is not part of the term (like an operator or a different kind of token).
    // todo: Add examples
    pub fn group_tokens_into_terms(src_tokens: Vec<Token<T>>) -> Result<Vec<TokenOrTerm<T>>, String> {
        let mut iter = src_tokens.into_iter().peekable();
        let mut result_tokens = Vec::new();
    
        while let Some(token) = iter.next() {
            let mut term_tokens = Vec::new();  // Collect tokens for a term
            match &token {
                Token::Operator(op) if *op == T::NEGATOR => term_tokens.push(token),
                Token::Number(_) => term_tokens.push(token),
                Token::Variable(_) => {
                    term_tokens.push(token);
                    if let Some(&Token::UnderScore()) = iter.peek() {
                        term_tokens.push(iter.next().unwrap()); // Consume the underscore
                        // Check for at least one number following the underscore
                        if iter.peek().map_or(false, |t| matches!(t, Token::Number(_))) {
                            while let Some(&Token::Number(_)) = iter.peek() {
                                term_tokens.push(iter.next().unwrap()); // Consume the number
                            }
                        } else {Err("Expected a number after underscore".to_string())?}
                    }
                    result_tokens.push(TokenOrTerm::Term(Term::from(term_tokens)));  // Add the collected term
                },
                _ => result_tokens.push(TokenOrTerm::SingleToken(token)),
            }
        }
        match result_tokens.last() {  // Check if the last element is a Term
            Some(TokenOrTerm::Term(_)) | Some(TokenOrTerm::SingleToken(Token::CloseBracket(_))) => Ok(result_tokens),
            _ => Err("The last element of the sequence must be a Term".to_string()),
        }
    }


    /// Parses a proposition and validates its structure according to specific rules.
    ///
    /// This function checks a sequence of tokens or terms to ensure they follow the defined syntactical rules
    /// of the language or logical system. It validates the proper arrangement of operators, brackets,
    /// and terms within the proposition.
    ///
    /// # Arguments
    /// - `proposition` - A slice of `TokenOrTerm<T>` representing the sequence to be validated.
    ///
    /// # Returns
    /// Returns `Ok(true)` if the sequence adheres to the rules, or an `Err` with a message describing
    /// the nature of the syntax error.
    ///
    /// # Rules for Validation
    /// - The first element must be an open bracket. Other starters (like a lone operator) are invalid.
    /// - A close bracket must be followed by either an open bracket or an operator.
    /// - Operators must not be immediately preceded by another operator, except for the negator.
    // todo: Add examples
    pub fn validate_proposition_structure(proposition: &[TokenOrTerm<T>]) -> Result<bool, String> {
        let mut previous_token: Option<&TokenOrTerm<T>> = None;

        for (i, token_or_term) in proposition.iter().enumerate() {
            match token_or_term {  // Analyze each token or term based on its type
                TokenOrTerm::SingleToken(token) => {
                    // if i == 0 && !matches!(token, Token::OpenBracket(_) | Token::Operator(T::NEGATOR)) {
                    if i == 0 && !matches!(token, Token::OpenBracket(_)) {
                        Err("Invalid start of proposition".to_string())?
                    }
                    // Close bracket rule: followed by an open bracket or an operator
                    if let Token::CloseBracket(_) = token {
                        if i + 1 < proposition.len() {
                            if let TokenOrTerm::SingleToken(next_token) = &proposition[i + 1] {
                                if !matches!(next_token, Token::OpenBracket(_) | Token::Operator(_)) {
                                    Err(format!("Invalid token following a close bracket at position {i}"))?;
                                }
                            }
                        }
                    }
                    // Operator sequence rule: not preceded by another operator unless it's a negator
                    if let Token::Operator(_) = token {
                        if let Some(TokenOrTerm::SingleToken(prev_token)) = previous_token {
                            if matches!(prev_token, Token::Operator(op) if *op != T::NEGATOR) {
                                Err(format!("Invalid sequence of operators at position {i}"))?
                            }
                        }
                    }
                },
                TokenOrTerm::Term(_) => if i == 0 {continue},  //  For Term, no specific rule in the current context
            }
            previous_token = Some(token_or_term);
        }
        Ok(true)  // If no errors are encountered, return true
    }

}
