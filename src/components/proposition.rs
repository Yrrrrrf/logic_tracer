/// This module defines the `Propositions` enum, which represents various logical propositions.
/// It is used for modeling logical statements in Rust programs.
// Import necessary modules and dependencies if needed.

/// The `Propositions` enum represents different logical propositions.
// todo: implement the PartialEq trait for the Propositions (to compare the propositions results)


// ? LOGICAL PROPOSITIONS ---------------------------------------------------------------------------------------------------------------------------------------------------------

use crate::components::ast::AST;


#[derive(Debug, Clone, PartialEq)]
pub struct Proposition {
    pub src: String,
    pub ast: AST,
}


impl Proposition {
    /// Create a new proposition.
    /// 
    /// ### Arguments:
    /// - `src` - the proposition source code
    /// 
    pub fn new(src: &str) -> Self {        
        Self {
            src: src.to_string(),
            ast: AST::new(src),
        }
    }


    pub fn evaluate_logic(&mut self) -> Result<Vec<bool>, String> {
        Ok(vec![])
    }


    pub fn  evaluate_math(&mut self) -> Result<Vec<f64>, String> {
        Ok(vec![])
    }

}

