//! The AST represents the hierarchical structure of the parsed source code
//! 
//! Capturing the syntactic elements and their relationships.
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
pub struct AST {

}


// Implement my own Debug trait
impl fmt::Debug for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&set_fg("AST", "g"))
            // .field("src", &self.src)
            .finish()
    }
}