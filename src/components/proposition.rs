/// This module defines the `Propositions` enum, which represents various logical propositions.
/// It is used for modeling logical statements in Rust programs.
// Import necessary modules and dependencies if needed.
/// The `Propositions` enum represents different logical propositions.

use std::fmt;

use crate::components::ast::AST;
use crate::components::lexer::Lexer;
use crate::components::parser::Parser;


/// The `Proposition` struct represents a logical proposition.
/// 
/// It is used for modeling logical statements in Rust programs.
#[derive(Clone, PartialEq)]
pub struct Proposition {
    pub src: String,
    pub ast: AST,
    pub f: String,  // function string
}

impl Proposition {
    /// Create a new proposition.
    /// 
    /// ### Arguments:
    /// - `src` - the proposition source code
    /// 
    pub fn new(src: &str) -> Self {        
        let mut abst = AST::new(src);
        Self {
            src: src.to_string(),
            ast: abst.clone(),
            f: abst.postfix_string(),
        }
    }


    /// Evaluate the proposition using the rules of BOOLEAN ALGEBRA.
    /// 
    /// ### Arguments:
    /// - `self` - the proposition
    /// 
    /// ### Returns:
    /// - `Vec<bool>` - the result of the evaluation
    pub fn evaluate_logic(&self) -> Vec<bool> {
        vec![]
    }


    /// Evaluate the proposition using the rules of ARITHMETIC.
    /// 
    /// ### Arguments:
    /// - `self` - the proposition
    /// 
    /// ### Returns:
    /// - `Vec<f64>` - the result of the evaluation
    pub fn  evaluate_math(&self) -> Vec<f64> {
        vec![]
    }


    /// Show the Karnaugh Map of the proposition.
    /// 
    /// ### Arguments:
    /// 
    /// ### Returns:
    /// - `Result<(), String>` - the result of the evaluation
    pub fn show_kmap(&mut self) -> Result<(), String> {
        // todo: implement this function
        Ok(())
    }        


    pub fn print_truth_table(&mut self) {
        
    }

}


// Implement my own Debug trait
impl fmt::Debug for Proposition {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&crate::util::terminal::set_fg("Proposition", "g"))
            .field("src:", &self.src)
            .field("f:", &self.f)
            .finish()

    }
}
