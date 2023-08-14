//! # Parser
//! 
//! Is the second stage of the compiler. It takes the tokens produced by the lexer and turns them
//! into an abstract syntax tree (AST). The parser is also responsible for reporting syntax errors.
//! 
//! The parser is implemented as a set of mutually recursive functions. Each function
//! implements one of the grammar rules. The parser is a predictive recursive descent parser (PRDP).
//! 

// use crate::lexer::{Lexer, Token, TokenKind};

#![allow(unused)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token
}

//! ## Grammar
//! 
//! - todo: describe grammar
//! - todo: add grammar rules
//! - todo: add grammar diagram (EBNF)
//! 