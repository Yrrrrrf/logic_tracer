//! # Parser
//! Syntax Analysis
//! 
//! Is the second stage of the compiler. It takes the tokens produced by the lexer and turns them
//! into an abstract syntax tree (AST). The parser is also responsible for reporting syntax errors.
//! 
//! The parser is implemented as a set of mutually recursive functions. Each function
//! implements one of the grammar rules. The parser is a predictive recursive descent parser (PRDP).
#![allow(unused)]


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


    pub fn parse(&mut self) -> Result<(), String> {

        // Iterate over the tokens
        // let mut token = self.lexer.


        Ok(())






    }


    /// Evaluate the expression and return the results a vectot of bools.
    pub fn evaluate(&mut self) -> Result<Vec<bool>, String> {

        // todo: implement this function well
        let mut results = vec![];  // results
        // while self.current_token != GrammarToken::End { // while not end of expression
            // let result = self.evaluate_expr()?;  // evaluate expression
            // results.push(result);  // push result to results
            // self.next_token();  // advance to next token
        // }
        Ok(results)
    }




}

// - todo: add grammar rules
// - todo: add grammar diagram (EBNF)
// 