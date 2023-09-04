//! # AST
//! 
//! It represents the hierarchical structure of the parsed source code capturing the syntactic elements and their relationships.
//! 
//! This module offers functions to traverse, analyze, and manipulate the AST, 
//! enabling developers to perform various operations like code transformations, static analysis, and code generation.
//! 
//! It serves as a fundamental component in many Rust programming tools and compilers

use std::fmt;

use crate::grammar::{GrammarToken, BracketState};
use crate::operators::*;
use crate::util::terminal::set_fg;


/// The AST struct
/// This is the most basic trait used in the application.
/// 
/// It allows the structs that implement it to use the eval() function that it abstracts.
// add derive
#[derive(Clone, PartialEq)]
pub struct Ast {
    pub src: Vec<char>,
    token_table: Vec<GrammarToken>,
}


impl Ast {
    /// Create a new AST
    pub fn new(src: &str) -> Ast {
        let trimeed = src.chars()  // Create a new iterator over the input string
            .filter(|c| !c.is_whitespace() || !c.is_ascii_control())  // filter all the whitespaces, tabs and newlines
            .collect::<Vec<char>>();
        Ast {
            src: trimeed.clone(),
            token_table: Ast::get_token_table(trimeed),
        }
    }


    /// ? This function makes the same work that will do the **Lexer** in a compile process ---------------------------------------
    /// This funciton makes the same as the Lexer in a compile process.
    /// 
    /// To be a data type optimiser for speed such as std::collections::VecDeque<char>
    /// 
    fn get_token_table(src: Vec<char>) -> Vec<GrammarToken> {
        let mut tokens = Vec::new();
    
        // while let Some(c) = src. {
        for c in src.iter() {
            tokens.push(match c {
                // if it's a number
                //* This radix (10) is used to specify the number base
                //* the base change logic is still on the `prototype` stage
                '0'..='9' => GrammarToken::Number(c.to_digit(10).unwrap()),

                // if it's part of the alphabet
                'A'..='Z' | 'a'..='z' => GrammarToken::Variable(*c),  // *c pass the reference of the char

                // ? Grammar Rules --------------------------------------------------------------------------------------------

                // if the char is a bracket
                '(' => GrammarToken::Brackets(BracketState::OpenParenthesis),
                ')' => GrammarToken::Brackets(BracketState::CloseParenthesis),
                '[' => GrammarToken::Brackets(BracketState::OpenSquareBracket),
                ']' => GrammarToken::Brackets(BracketState::CloseSquareBracket),
                '{' => GrammarToken::Brackets(BracketState::OpenCurlyBracket),
                '}' => GrammarToken::Brackets(BracketState::CloseCurlyBracket),
                '<' => GrammarToken::Brackets(BracketState::OpenChevron),
                '>' => GrammarToken::Brackets(BracketState::CloseChevron),                

                // if it's a logic operator
                '&' | '∧' => GrammarToken::Operator(LogicOp::And),
                '|' | '∨' | '+' => GrammarToken::Operator(LogicOp::Or),
                '!' | '¬' => GrammarToken::Operator(LogicOp::Not),
                '⊕' | '⊻' => GrammarToken::Operator(LogicOp::XOr),
                '⊙' => GrammarToken::Operator(LogicOp::XNOr),
                '↑' => GrammarToken::Operator(LogicOp::NAnd),
                '↓' => GrammarToken::Operator(LogicOp::NOr),
                '→' => GrammarToken::Operator(LogicOp::Implies),
                '↔' => GrammarToken::Operator(LogicOp::IFf),

                // if it's a variable
                // _ => if "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(*c) {GrammarToken::Variable(*c)}
                // else {GrammarToken::Error(c.to_string())}
                _ => GrammarToken::Error(format!("Unvalid char: {}", c)),
            });
        }
        tokens
        
    }


    /// ? This function makes the function of the **Parser**.
    /// Parse the input string.
    /// 
    /// This function will parse the input string and create the AST.
    pub fn parse(&mut self) -> Result<(), String> {
        self.check_pair_brackets();
        // Ok(AST::new(self.src))
        Ok(())
    }


    /// Check if the brackets are paired
    /// Also check if the brackets are in the correct order
    /// 
    /// This funciton evaluates: parenthesis, square brackets, curly brackets and chevrons
    /// All of those are described on the [`BracketState`] enum
    pub fn check_pair_brackets(&mut self) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for char in self.src.iter() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(*char),  // push the opening bracket to the stack
                ')' | ']' | '}' | '>' => {  // if it is a closing bracket
                    if stack.is_empty() {return false;}  // if the stack is empty, return false
                    let top = stack.pop().unwrap();  // pop the top element from the stack
                    if !((top == '(' && *char == ')')  // if the top element is not the same as the closing bracket
                      || (top == '[' && *char == ']')
                      || (top == '{' && *char == '}')
                      || (top == '<' && *char == '>')) {
                        return false;  // return false
                    }
                }
                _ => (),  // do nothing if it is not a bracket
            }
        }
        stack.is_empty()  // If the stack is empty at the end, all brackets are properly paired
    }


    /// Returns the function string of the AST
    /// 
    /// The function string is the string that represents the function of the AST
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the function string
    pub fn get_function(&mut self) -> String {
        // todo: implement function string
        todo!("Implement Function String")
    }


    /// Get infix string
    /// 
    /// The infix string is the string that represents the function of the AST in infix notation
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the infix string
    pub fn get_infix_string(&mut self) -> String {
        // todo: implement infix string
        todo!("Implement Infix String")
    }
    
    /// Get postfix string
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the infix string
    pub fn get_postfix_string(&mut self) -> String {
        // todo: implement postfix string
        todo!("Implement Postfix String")
    }
    
    /// Get prefix string
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the infix string
    pub fn get_prefix_string(&mut self) -> String {
        // todo: implement prefix string
        todo!("Implement Prefix String")
    }

    /// Get the AST    
    /// 
    /// ### Arguments:
    /// - `self` - the AST
    /// 
    /// ### Returns:
    /// - `String` - the infix string
    // todo: make a better visualization of the AST (binary tree)
    pub fn get_ast_string(&mut self) -> String {
        // todo: implement AST visualization (binary tree)
        todo!("Implement AST visualization")
    }


}


// Implement my own Debug trait
impl fmt::Debug for Ast {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&set_fg("AST", "g"))
            .field("src", &self.src)
            .finish()
    }
}
