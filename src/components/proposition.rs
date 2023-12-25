//! This module defines the `Propositions` enum, which represents various logical propositions.
//! It is used for modeling logical statements in Rust programs.
//! Import necessary modules and dependencies if needed.
//! The `Propositions` enum represents different logical propositions.

use dev_utils::console::format::set_fg;
use crate::grammar::GrammarToken;

/// The `Proposition` struct represents a logical proposition.
/// 
/// It is used for modeling logical statements in Rust programs.
#[derive(Clone, Default, Debug)]
pub struct Proposition {
    pub token_table: Vec<GrammarToken>,
    // pub f: String,  // function string
    // pub postfix: String,
    // pub variables: Vec<>
}


impl Proposition {
    /// Creates a new `Proposition` from the given source string.
    ///
    /// If the input string contains unpaired brackets, the function returns an empty `Proposition`.
    ///
    /// # Arguments
    ///
    /// - `src` - A reference to a string slice containing the source string to parse.
    ///
    /// # Returns
    /// 
    /// - `Proposition` - A new `Proposition` instance.
    /// 
    /// # Examples
    ///
    /// ```
    /// use logic_tracer::grammar::GrammarToken::{self, *};
    /// use logic_tracer::proposition::Proposition;
    ///
    /// assert_eq!(Proposition::new("A & B").token_table, 
    ///     [Variable('A'), Operator('&'), Variable('B')]);
    /// assert_eq!(Proposition::new("A + B").token_table, 
    ///     [Variable('A'), Operator('+'), Variable('B')]);
    /// // assert_eq!(Proposition::new("A | B").token_table,
    /// //    [Variable('A'), Operator('+'), Variable('B')]);
    /// ```
    pub fn new(src: &str) -> Result<Proposition, &str> {
        // Lexer -> Remove all unuseful chars & get the TokenTable
        let trimmed: Vec<char> = src.chars()
            .filter(|c| !c.is_whitespace() && !c.is_ascii_control()).collect(); // Remove all unuseful chars
        // Parser -> Validate the sintax & create the AST
        check_pair_brackets(&trimmed)?;  // Check if the brackets are paired and in the correct order or return an error
        validate_prop_grammar(&trimmed)?;  // Validate the grammar of the proposition or return an error

        Ok(Proposition { 
            token_table: trimmed.iter().map(|c| GrammarToken::from(*c)).collect(),
            ..Default::default()
        })
    }


    /// Evaluate the proposition using the rules of BOOLEAN ALGEBRA.
    /// 
    /// ### Arguments:
    /// - `self` - the proposition
    /// 
    /// ### Returns:
    /// - `u64` - the result of the evaluation (binary number)
    pub fn evaluate_logic(&self) -> u64 {
        0
    }


    /// Evaluate the proposition using the rules of ARITHMETIC.
    /// 
    /// ### Arguments:
    /// - `self` - the proposition
    /// - `var` - the variable values that will be used in the evaluation
    /// 
    /// ### Returns:
    /// - `f64` - the result of the evaluation (number)
    pub fn evaluate_math(&self, var: Vec<f64>) -> f64 {
        0.0
    }


    /// This returns the result of the proposition as a vector of booleans.
    /// 
    /// ### Arguments:
    /// - `self` - the proposition
    /// 
    /// ### Returns:
    /// - `Vec<bool>` - the result of the evaluation 
    pub fn get_result_vec(&mut self) -> Vec<bool> {
        vec![]
    }


    /// Returns the function string of the AST
    /// 
    /// The function string is the string that represents the function of the AST
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the function string
    pub fn get_function(&mut self) -> String {
        // todo: implement function string
        todo!("Implement Function String")
    }

    // ? F NOTATION FUNCTIONS ===============================================    

    /// Get infix string
    /// 
    /// The infix string is the string that represents the function of the AST in infix notation
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the infix string
    pub fn get_infix_string(&mut self) -> String {
        // todo: implement infix string
        todo!("Implement Infix String")
    }


    /// Get postfix string
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the infix string
    pub fn get_postfix_string(&mut self) -> String {
        // todo: implement postfix string
        todo!("Implement Postfix String")
    }


    /// Get prefix string
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the infix string
    pub fn get_prefix_string(&mut self) -> String {
        // todo: implement prefix string
        todo!("Implement Prefix String")
    }


    // ? VIEW FUNCTIONS (KMAP, TRUTH TABLE, ETC.) ===========================


    /// Get the AST    
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the infix string
    // todo: make a better visualization of the AST (binary tree)
    pub fn get_ast_string(&mut self) -> String {
        // todo: implement AST visualization (binary tree)
        todo!("Implement AST visualization")
    }

    // * VIEW 4 LOGIC FUNCTIONS ================================================ 

    /// Show the Karnaugh Map of the proposition.
    /// 
    /// ### Arguments:
    /// - `self` - the proposition
    /// 
    /// ### Returns:
    /// - `String` - the Karnaugh Map
    pub fn get_kmap_string(&mut self) -> String {
        "KMap".to_string()
    }        


    /// Show the truth table of the proposition.
    /// 
    /// ### Arguments:
    /// - `self` - the proposition
    /// 
    /// ### Returns:
    /// - `String` - the truth table
    pub fn get_truth_table_string(&mut self) -> String {
        "Truth Table".to_string()
    }

}


/// Check if the brackets are paired
/// Also check if the brackets are in the correct order
pub fn check_pair_brackets(src: &Vec<char>) -> Result<bool, &'static str> {
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
                return Err("Unpaired Brackets");
            }
            _ => (),
        }
    }
    Ok(stack.is_empty())
}


// Validates the grammar as a proposition
// This should be used after the brackets are paired
// Validates the prpoposition to be logic and mathematically correct
pub fn validate_prop_grammar(token_table: &Vec<char>) -> Result<bool, &'static str> {
    // match all the tokens

    // This function will validate the sequence to be a valid proposition
    // This sequence is valid by the regex: `^([A-Z]|\(|\)|\[|\]|&|\||!|>|<|=|\+|\-|\*|\/|\^|\%|\.)+$`
    for c in token_table {
        match c {
            _ => (),  // do nothing
        }
    }
    Ok(true)
}
