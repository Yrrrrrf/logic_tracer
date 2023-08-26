//! # AST
//! 
//! It represents the hierarchical structure of the parsed source code capturing the syntactic elements and their relationships.
//! 
//! This module offers functions to traverse, analyze, and manipulate the AST, 
//! enabling developers to perform various operations like code transformations, static analysis, and code generation.
//! 
//! It serves as a fundamental component in many Rust programming tools and compilers

use core::fmt;

use crate::util::terminal::set_fg;


/// The AST struct
/// This is the most basic trait used in the application.
/// 
/// It allows the structs that implement it to use the eval() function that it abstracts.
// add derive
#[derive(Clone, PartialEq)]
pub struct AST {
    name: String,
}

impl AST {
    
    pub fn new(name: &str) -> AST {
        let mut name = name;
        if name == "" && name.len() == 0 {
            name = "AST";
        } else {
            name = name;
        }
        AST {
            name: name.to_string(),
        }
    }

    pub fn evaluate(&mut self, input_vec: Vec<bool>) -> Result<Vec<bool>, String> {
        Ok(vec![])
    }

    pub fn get_results_vector(&mut self) -> Result<Vec<bool>, String> {
        Ok(vec![])
    }

    pub fn show_kmap(&mut self) -> Result<(), String> {
        Ok(())
    }        

    pub fn show_truth_table(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn show_ast(&mut self) -> Result<(), String> {
        Ok(())
    }

}








/// The node enum is a vertex on a binary tree (AST)
/// 
/// It can be a variable or an operator  
/// If it is a variable, it will be a leaf node  
/// If it is an operator, it will be a non-leaf node (internal Node)
pub enum Node {
    Variable(String),  // Any variable
    // Operator(LogicToken, Box<Node>, Box<Node>),  // Any operator with two operands
}


// Implement my own Debug trait
impl fmt::Debug for AST {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&set_fg("AST", "g"))
            .field("name", &self.name)

            // todo: make a better visualization of the AST (binary tree)

            .finish()
    }
}