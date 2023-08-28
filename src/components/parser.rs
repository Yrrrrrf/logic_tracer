//! # Parser
//! Syntax Analysis
//! 
//! Is the second stage of the compiler. It takes the tokens produced by the lexer and turns them
//! into an abstract syntax tree (AST). The parser is also responsible for reporting syntax errors.
//! 
//! The parser is implemented as a set of mutually recursive functions. Each function
//! implements one of the grammar rules. The parser is a predictive recursive descent parser (PRDP).

use crate::components::grammar::*;
use crate::components::grammar::GrammarToken::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    // lexer: Lexer,  // lexer
    current_token: GrammarToken,  // current token
    peek_token: Option<GrammarToken>  // next token
}


impl Parser {    
    /// Create a new parser with the trimmed lexer.
    /// 
    /// ### Arguments:
    /// - `lexer` - the lexer to be used
    /// 
    pub fn new(mut src: &str) -> Self {
        // let mut lexer = Lexer::new(&mut src);
        // lexer.trim();  // trim all whitespaces
        // let current_token = lexer.curr;
        Parser {
            // lexer,
            current_token: GrammarToken::default(),
            peek_token: None  // lexer.next()
        }
    }


    /// Parse the input string.
    /// 
    /// This function will parse the input string and create the AST.
    pub fn parse(&mut self) -> Result<(), String> {
        Ok(())

    }

}


// - todo: add grammar diagram (EBNF) -> https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form
