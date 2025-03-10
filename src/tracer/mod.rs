//! The 'tracer' module is a simple language processor that includes a lexer and parser.
//!
//! The lexer is responsible for converting a string of source code into a stream of tokens,
//! while the parser constructs an abstract syntax tree (AST) from the token stream.
//!
//! The lexer supports any character from Unicode and can theoretically be used to tokenize any language.
//!
//! The tokens are defined in the [`tokens`] module, and the lexer can iterate over these tokens.
//!
//! # Usage
//!
//! ```rust
//!
//! ```
//!
//! // * Primitive Token types
/// This module defines the primitive token types that are recognized and used
/// by the lexer and parser components.
///
/// It includes definitions for various types of tokens such as numbers, operators,
/// and variables, which are fundamental for the lexical and syntactic analysis processes.
pub mod tokens;

// * Compiler Components

// * Lexer (Lexical Analysis) -> TokenStream
/// This module contains the lexer, which is responsible for the lexical analysis
/// of the source code.
///
/// The lexer converts a string into a stream of tokens, which can then be used
/// by the parser for further analysis. The lexer supports any character from Unicode
/// and can theoretically be used to tokenize any language. The tokens are defined
/// in the `tokens` module, and the lexer can iterate over these tokens.
///
/// # Usage
///
/// The lexer can be used to process a string of source code and produce a sequence
/// of tokens that represent the lexical elements of the code.
pub mod lexer;

// * Parser (Syntactic Analysis) -> AST
/// This module provides the parser, which is responsible for syntactic and semantic
/// analysis of the token stream produced by the lexer.
///
/// The parser uses grammar rules to construct an abstract syntax tree (AST) from
/// the token stream. It also performs semantic analysis, including type checking,
/// to ensure the correctness of the AST. The syntax and semantics analysis processes
/// are closely linked and may change depending on the language and its grammar rules.
///
/// # Features
///
/// - Constructs an AST from the token stream.
/// - Performs type checking and other semantic analysis.
/// - Supports customizable grammar rules for different languages.
pub mod parser;

pub mod ast; // + semantic analysis...

// // * Proposition
// /// This module is intended to implement a simple propositional logic language (to be implemented).
// ///
// /// The goal is to use the lexer with a specific set of tokens to tokenize the source code
// /// and then use the parser to parse the tokens into an AST. This module will serve as an
// /// example of how to use the lexer and parser components to build a language processor.
// ///
// /// # TODO
// ///
// /// - Implement a simple propositional logic language.
// /// - Use the lexer to tokenize the source code.
// /// - Implement the parser to parse the tokens into an AST.

// * Proposition
// todo: Implement a simple propositional logic language...
// * use the Lexer with some Tokens to tokenize the source code...
// * Then somehow impl the parser to parse the tokens into a AST...
// pub mod proposition;  // proposition builder (Proposition)
// pub mod proposition;  // proposition builder (Proposition)
