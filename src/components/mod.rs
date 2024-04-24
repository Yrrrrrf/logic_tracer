// * Primitive Tokens
pub mod alphabet;  // alphabet of the language (All possible tokens)
pub mod operators;  // operators of the language (LogicOp, MathOp, RelOp)
pub mod number;


// * Grammar Rules
pub mod grammar;  // Grammar rules used by the parser
#[macro_export]
pub mod neo_grammar;  // Grammar rules used by the parser


// * Proposition
pub mod proposition;  // proposition builder (Proposition)
