//! # Parser
//! Syntax Analysis
//! 
//! Is the second stage of the compiler. It takes the tokens produced by the lexer and turns them
//! into an abstract syntax tree (AST). The parser is also responsible for reporting syntax errors.
//! 
//! The parser is implemented as a set of mutually recursive functions. Each function
//! implements one of the grammar rules. The parser is a predictive recursive descent parser (PRDP).

#![allow(unused)]


use crate::{lexer::*, operators::Operator};
use crate::components::grammar::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    lexer: Lexer,  // lexer
    current_token: GrammarToken,  // current token
    peek_token: Option<GrammarToken>  // next token
}


impl Parser {    
    /// Create a new parser with the trimmed lexer.
    /// 
    /// ### Arguments:
    /// - `lexer` - the lexer to be used
    /// 
    pub fn new(mut lexer: Lexer) -> Self {
        lexer.trim();  // trim all whitespaces
        // let current_token = lexer.curr;
        Parser {
            lexer,
            current_token: GrammarToken::default(),
            peek_token: None  // lexer.next()
        }
    }


    pub fn parse(&mut self) -> Result<(), String> {
        Ok(())
    }


    pub fn check_brackets(&mut self) -> Result<(), String> {
        panic!("Not implemented yet")
        // let mut brackets = 0;
        // while self.current_token != TokenKind::Edn {
        //     match self.current_token {
        //         TokenKind::LeftBracket => brackets += 1,
        //         TokenKind::RightBracket => brackets -= 1,
        //         _ => ()
        //     }
        //     self.next_token();  // advance to next token
        // }
        // if brackets != 0 {
        //     Err(format!("Unbalanced brackets: {}", brackets))
        // } else {
        //     Ok(())
        // }
    }








}

// - todo: add grammar rules
// - todo: add grammar diagram (EBNF)
// 