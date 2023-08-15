//! # Lexer
//! Lexical Analysis
//! 
//! Is the **first step in the compilation process**. It takes the input string and breaks it down into a list of tokens.
//! 
//! Each Token will be categorized into wheter it is READING the FILE, a LINK, or a COMPONENT
//! 
//! Then the list of tokens will be passed to the ['Parser'], which will be responsible for creating the AST (Abstract Syntax Tree).

// ? Imports --------------------------------------------------------------------------------------------------------------------------------------------------------------

use std::fmt;

use crate::util::terminal::set_fg;

use super::grammar::GrammarToken;

// ? Lexer --------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone, PartialEq)]
pub struct Lexer {
    /// The source code to be parsed
    pub src: String,  // input string
    /// Current character being processed
    pub curr: char,   // current character
    /// Current position in input (points to current char)
    pub pos: usize,  // current position in input (points to current char)
    /// End of file flag
    pub eof: bool  // end of file flag
}


impl Lexer {
    /// Creates a new instance of the Lexer struct
    ///
    ///  These fields will hold information about the input string provided, such as the current character being processed, its position, and whether or not the end of the string has been reached.
    /// 
    /// ### Parameters
    /// - `src`: A [`String`] that represents the input string
    /// 
    /// ### Returns
    /// - `Lexer`: A Lexer struct instance
    pub fn new(src: &str) -> Lexer {
        let mut lexer = Lexer {  // Create a new Lexer instance
            src: src.to_string(),  // Set the input string to the src field
            curr: '\0',  // Set the current character to null
            pos: 0,  // Set the current position to 0
            eof: false  // Set the end of file flag to false
        };
        if lexer.pos >= src.len() {lexer.eof = true;  // If the current position is greater than or equal to the length of the input string then set the end of file flag to true
        } else {lexer.curr = src.chars().nth(0).unwrap();}  // Otherwise, set the current character to the first character in the input string
        return lexer
    }
    

    // next_token(): This method is responsible for moving to the new token in the list of tokens created by deconstructing the input string. The next token is categorized into whether it is a number, math operation, parentheses or unexpected character. The category is then returned in the Ok() method
    /// This method is responsible for moving to the new token in the list of tokens created by deconstructing the input string. The next token is categorized into whether it is a number, math operation, parentheses or unexpected character. The category is then returned in the Ok() method
    /// 
    /// ### Parameters
    /// - `self`: A mutable reference to the Lexer struct
    /// 
    /// ### Returns
    /// - `Result<Token, String>`: A Result enum that contains either a Token or an error message
    pub fn next_token(&mut self) -> Result<GrammarToken, String> {
        if self.eof {
            return Ok(GrammarToken::End);  // If the end of file flag is true then return the EndOfFile token
        }
        match self.curr {  // for all the values in the input string
            ' ' | '\n' | '\t' | '\r' => {
                self.bump();
                return Ok(GrammarToken::Space);
            }
            _ => {
                self.bump();
                return Ok(GrammarToken::Reading);
            }
        }
    }


    /// Increments the current position in the input string being read and assigns the new value to the field self.curr
    /// 
    /// ### Parameters:
    /// - `self`: A mutable reference to the Lexer struct
    pub fn bump(&mut self) {
        self.curr = self.src.chars().nth(self.pos).unwrap();  // Otherwise, set the current character to the character at the current position in the input string
        self.pos +=1;  // Increment the current position by 1
        if self.pos >= self.src.len() {  // If the current position is greater than or equal to the length of the input string then set the end of file flag to true
            self.eof = true;  // Set the end of file flag to true
            return;  // Return
        }
        // print!("{}", self.pos);
    }


    /// Removes any whitespace from the input string
    /// 
    /// ### Parameters:
    /// - `self`: A mutable reference to the Lexer struct
    pub fn trim(&mut self) {
        let mut trimmed = String::new();
        for i in 0..self.src.len() {
            self.curr = self.src.chars().nth(i).unwrap();
            match self.curr {
                // \r = carriage return
                // \n = line feed
                // \t = tab
                ' ' | '\n' | '\t' | '\r' => {continue;},  // ignore whitespace
                _ => {trimmed.push(self.curr);}  // add the current character to the trimmed string
            }
        }
        // println!("Trimmed: {}", trimmed);
        self.src = trimmed;
        self.reset_lexer();
    }


    /// Return the data of the lexer in a Vec[Result<Token, String>]
    /// 
    /// ### Parameters:
    /// - `self`: A mutable reference to the Lexer struct
    /// 
    /// ### Returns:
    /// - `Vec<(Token, char)>`: A Vec of tuples that contain a Token and a char
    pub fn get_tokens(&mut self) -> Vec<(GrammarToken, char)> {
        let mut tokens: Vec<(GrammarToken, char)> = Vec::new();  // create a new Vec to hold the tokens

        self.trim();  // remove any whitespace from the input string

        loop {  // print the tokens
            match self.next_token() {  // get the next token
                Ok(token) => {  // if the token is Ok
                    match token {
                        GrammarToken::End => {  // if the token is EndOfFile
                            // println!("{}{:<10}{} ", "\x1b[31m", token.to_string().as_str(), "\x1b[0m");
                            break;
                        },
                        _ => {
                            let n_th_char: char = self.src.chars().nth(self.pos-1).unwrap();
                            let item: (GrammarToken, char) = (token, n_th_char);
                            // println!("{:<10} {}{:>4}{} {}{}{}", item.0.to_string().as_str(), "\x1b[32m", (self.pos-1), "\x1b[0m", "\x1b[34m", n_th_char, "\x1b[0m");
                            tokens.push(item);  // push the tuple into the Vec (Token, char)
                        }
                    }  // now that we have the token, we can do something with it, like, we can parse it
                },
                // todo: this code looks a little bit messy, maybe we can improve it

                // todo: improve error handling here
                Err(error) => {  // if the token is Err
                    println!("Error: {}", error);  // print the error
                    break;  // break the loop
                }
            }
        };
        return tokens;  // return the Vec
    }


    /// Resets the Lexer struct fields (curr, pos, eof)
    /// Mantain the same src string
    /// 
    /// ### Parameters:
    /// - `self`: A mutable reference to the Lexer struct
    pub fn reset_lexer(&mut self) {
        self.curr = '\0';
        self.pos = 0;
        self.eof = false;
    }


}


// Implement my own Debug trait
impl fmt::Debug for Lexer {
    /// This function is used to format the output of the AST
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&set_fg("AST", "g"))
            .field("src", &self.src)

            
            // todo: CHANGE THE SRC FIELD TO PRINT THE TOKEN TABLE
            

            .field("curr", &self.curr)
            .field("pos", &self.pos)
            .field("eof", &self.eof)
            .finish()
    }
}
