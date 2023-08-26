//! # Lexer
//! Lexical Analysis
//! 
//! Is the **first step in the compilation process**. It takes the input string and breaks it down into a list of tokens.
//! 
//! Each Token will be categorized into wheter it is READING the FILE, a LINK, or a COMPONENT
//! 
//! Then the list of tokens will be passed to the ['Parser'], which will be responsible for creating the AST (Abstract Syntax Tree).
#[allow(unused)]

// ? Imports --------------------------------------------------------------------------------------------------------------------------------------------------------------

use std::fmt;

use crate::{
    util::terminal::set_fg, 
    components::grammar::*
};

use super::grammar::GrammarToken;

use std::collections::VecDeque;

// ? Lexer --------------------------------------------------------------------------------------------------------------------------------------------------------------



/// Lexer: This struct is responsible for breaking down the input string into a list of tokens.
/// 
/// It also holds information about the input string provided, such as the current character being processed, its position, and whether or not the end of the string has been reached.
/// It functions similar as a queue, where the current character is the first element in the queue and the next character is the second element in the queue.
#[derive(Clone, PartialEq)]
pub struct Lexer {
    /// The source code to be parsed
    pub src: String,  // input string
    /// Current character being processed
    pub reduced_src: VecDeque<char>,  // input string
}


impl Lexer {
    /// Creates a new instance of the Lexer struct
    ///
    /// ### Parameters
    /// - `src`: A [`String`] that represents the input string
    /// 
    /// ### Returns
    /// - `Lexer`: A Lexer struct instance
    pub fn new(src: &str) -> Lexer {
        Lexer {  // Create a new Lexer instance
            src: src.to_string(),  // Set the input string to the src field
            reduced_src: src.chars()  // Create a new iterator over the input string
                // filter all the whitespaces, tabs and newlines and return the filtered characters
                .filter(|c| !c.is_whitespace() || !c.is_ascii_control())
                .collect::<VecDeque<char>>(),  // return the filtered characters    
        }
    }

    
    
    /// Check if the brackets are paired
    /// Also check if the brackets are in the correct order
    /// 
    /// This funciton evaluates: parenthesis, square brackets, curly brackets and chevrons
    /// All of those are described on the [`BracketState`] enum
    pub fn check_pair_brackets(&mut self) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for char in self.src.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),  // push the opening bracket to the stack
                ')' | ']' | '}' | '>' => {  // if it is a closing bracket
                    if stack.is_empty() {return false;}  // if the stack is empty, return false
                    let top = stack.pop().unwrap();  // pop the top element from the stack
                    if !((top == '(' && char == ')')
                      || (top == '[' && char == ']')
                      || (top == '{' && char == '}')
                      || (top == '<' && char == '>')) {
                        return false;
                    }
                }
                _ => (),  // do nothing if it is not a bracket
            }
        }
        stack.is_empty()  // If the stack is empty at the end, all brackets are properly paired
    }
    



    pub fn get_token_table(&mut self) -> Vec<(GrammarToken, char)> {
        let mut tokens: Vec<(GrammarToken, char)> = Vec::new();  // create a new Vec to hold the tokens

        // || !c.is_ascii_punctuation()

        // iterate over the reduces_src and get the tokens (chars)
        // at the end of the iteration, the reduced_src should be empty, and the tokens should be filled
        while let Some(c) = self.reduced_src.pop_front() {
            match c {
                '&' => tokens.push((GrammarToken::Reading, c)),
                '|' => tokens.push((GrammarToken::Reading, c)),
                '(' => tokens.push((GrammarToken::Brackets(BracketState::OpenParenthesis), c)),
                ')' => tokens.push((GrammarToken::Brackets(BracketState::ClosedParenthesis), c)),
                '[' => tokens.push((GrammarToken::Brackets(BracketState::OpenSquareBracket), c)),
                ']' => tokens.push((GrammarToken::Brackets(BracketState::ClosedSquareBracket), c)),
                '{' => tokens.push((GrammarToken::Brackets(BracketState::OpenCurlyBracket), c)),
                '}' => tokens.push((GrammarToken::Brackets(BracketState::ClosedCurlyBracket), c)),
                '<' => tokens.push((GrammarToken::Brackets(BracketState::OpenChevron), c)),
                '>' => tokens.push((GrammarToken::Brackets(BracketState::ClosedChevron), c)),
                _ => tokens.push((GrammarToken::Error((c).to_string()), c)),
            }
        }

        tokens
    }


    pub fn print_token_table(&mut self) {
        let tokens = self.get_token_table();
        for (token, c) in tokens {
            println!("{:?} {}", token, c);
        }
    }


}


// Implement my own Debug trait
impl fmt::Debug for Lexer {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&set_fg("Lexer", "g"))
            .field("reduced expresion", &self.reduced_src)
            .field("src", &self.src)
            .finish()
    }
}
