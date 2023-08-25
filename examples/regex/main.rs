#![allow(unused)]

extern crate logic_tracer;

use logic_tracer::{
    parser::Parser,
    lexer::Lexer,

    operators::Operator,
    operators::LogicOp,
    operators::LogicOp::*,
};



fn main() {
    println!("Regex example (2)");


    let parser = Parser::new("A & B");

    print!("{:?}", parser);
    panic!("Imlpement the parser example (1)")

}


