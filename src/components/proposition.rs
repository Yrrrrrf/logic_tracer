/// This module defines the `Propositions` enum, which represents various logical propositions.
/// It is used for modeling logical statements in Rust programs.
// Import necessary modules and dependencies if needed.
/// The `Propositions` enum represents different logical propositions.

use std::fmt;

use crate::grammar::GrammarToken;

/// The `Proposition` struct represents a logical proposition.
/// 
/// It is used for modeling logical statements in Rust programs.
#[derive(Clone, Default)]
pub struct Proposition {
    pub token_table: Vec<GrammarToken>,
    // pub f: String,  // function string
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
    /// use crate::{Proposition, GrammarToken};
    ///
    /// let prop = Proposition::new("A & B");
    /// assert_eq!(prop.token_table, vec![
    ///     GrammarToken::Variable('A'),
    ///     GrammarToken::And,
    ///     GrammarToken::Variable('B'),
    /// ]);
    /// ```
    pub fn new(src: & str) -> Proposition {       
        // * LEXER -> Remove all unuseful chars
        let trimmed = src.chars()  // Create a new iterator over the input string
            .filter(|c| !c.is_whitespace() && !c.is_ascii_control())  // filter all the whitespaces, tabs and newlines
            .collect::<Vec<char>>();
        // * PARSER -> TokenTable
        match check_pair_brackets(src) {
            true  => Proposition {token_table: trimmed.iter().map(|c| GrammarToken::new(*c)).collect::<Vec<GrammarToken>>()},  
            false => Proposition::default(),  // return an empty proposition if the brackets are not paired
        }
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


/// Implement my own Debug trait
impl fmt::Debug for Proposition {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&crate::util::terminal::set_fg("Proposition", "g"))
            .field("f", &self.token_table)
            .finish()

    }
}


/// Implement my own Display trait
impl fmt::Display for Proposition {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_table.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(""))
    }
}


/// Check if the brackets are paired
/// Also check if the brackets are in the correct order
/// 
/// This funciton evaluates: parenthesis, square brackets, curly brackets and chevrons
/// All of those are described on the [`BracketState`] enum
pub fn check_pair_brackets(src: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for char in src.chars().collect::<Vec<char>>() {
        match char {
            '(' | '[' | '{' | '<' => stack.push(char),  // push the opening bracket to the stack
            ')' | ']' | '}' | '>' => {  // if it is a closing bracket
                if stack.is_empty() {return false;}  // if the stack is empty, return false
                let top = stack.pop().unwrap();  // pop the top element from the stack
                if !((top == '(' && char == ')')  // if the top element is not the same as the closing bracket
                    || (top == '[' && char == ']')
                    || (top == '{' && char == '}')
                    || (top == '<' && char == '>')) {
                    return false;  // if the brackets are not paired, return false
                }
            }
            _ => (),  // do nothing if it is not a bracket
        }
    }
    stack.is_empty()  // If the stack is empty at the end, all brackets are properly paired
}
