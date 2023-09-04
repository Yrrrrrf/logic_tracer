/// This module defines the `Propositions` enum, which represents various logical propositions.
/// It is used for modeling logical statements in Rust programs.
// Import necessary modules and dependencies if needed.
/// The `Propositions` enum represents different logical propositions.

use std::fmt;
use crate::ast::Ast;

/// The `Proposition` struct represents a logical proposition.
/// 
/// It is used for modeling logical statements in Rust programs.
#[derive(Clone, PartialEq)]
pub struct Proposition {
pub ast: Ast,
pub f: String,  // function string
}

impl Proposition {
    /// Create a new proposition.
    /// 
    /// ### Arguments:
    /// - `src` - the proposition source code
    /// 
    pub fn new(src: &str) -> Self {       
        let mut abst = Ast::new(src);
        Self {
            ast: abst.clone(),
            f: abst.get_function(),
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


    // ? VIEW FUNCTIONS ===============================================

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


// Implement my own Debug trait
impl fmt::Debug for Proposition {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&crate::util::terminal::set_fg("Proposition", "g"))
            .field("src:", &self.ast.src.iter().map(|x| x.to_string()).collect::<Vec<String>>())
            // .field("f:", &self.f)
            .finish()

    }
}
