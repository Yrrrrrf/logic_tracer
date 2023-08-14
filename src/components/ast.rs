//! # AST
//! 
//! It represents the hierarchical structure of the parsed source code capturing the syntactic elements and their relationships.
//! 
//! This module offers functions to traverse, analyze, and manipulate the AST, 
//! enabling developers to perform various operations like code transformations, static analysis, and code generation.
//! 
//! It serves as a fundamental component in many Rust programming tools and compilers

use core::fmt;

use super::token::LogicToken;
use crate::util::terminal::set_fg;


/// The AST struct
/// This is the most basic trait used in the application.
/// 
/// It allows the structs that implement it to use the eval() function that it abstracts.
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
}

pub enum Node {
    Variable(String),  // Any variable
    Operator(LogicToken, Box<Node>, Box<Node>),  // Any operator with two operands
}


// Implement my own Debug trait
impl fmt::Debug for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&set_fg("AST", "g"))
            .field("name", &self.name)
            .finish()
    }
}