#![allow(unused)]


use log::{debug, LevelFilter};
use dev_utils::{
    print_app_data, 
    log::rlog::RLog
};
use logic_tracer::*;


fn main() {
    print_app_data(file!());
    // RLog::init_logger(LevelFilter::Info);
    // RLog::init_logger(LevelFilter::Debug);
    RLog::init_logger(LevelFilter::Trace);


    // * Number types
    let nums = [
        "123",
        "-456",
        "7.892",
        "-456.16",
        // "10.0i",
    ];
    nums.iter().for_each(|num| {
        if let Some(number) = Number::from(*num) {  // call the string as a number...
            // debug!("{:?}", number);
        }
    });


    // * Operator types
    let ops = [
        "+",
        "^",
        "~",
        "¬",
        "!",
        "&",

        // "*",
        // "|",
        // ">=",
        // "!=",
        // "<",
    ];
    ops.iter().for_each(|op| {
        if let Some(opeator) = Operator::from(*op) {  // call the char as an operator...
            // debug!("{:?}", opeator);
        }
    });


    // * PrimitiveToken types
    let tokens = [
        // // ^ Variables
        // "A",
        // "C",
        // "D",

        // ^ Numbers
        "-23",
        "2",
        "3.0",
        "3.1223",

        // ^ Operators
        "!",
        "&",
        "*",
        "|",
        ">=",
        "!=",

        // ^ Some...
        // "a",
    ];
    tokens.iter().for_each(|token| {
        if let Some(token) = Lexer::token_from(*token) {
            // debug!("{:?}", token);
            // debug!("{}", token.to_string());
        }

    });


    // todo: fix the Number handle for the case of
    // todo: `x.xx`, this means that I need to check when the string is something like..
    // todo: `x.*`, because I need to handle when the x
    let some_txt = "32 + 7 - 123 + 17.6 - 64x";
    // let some_txt = "- 123 + 17.6 - 6.4";
    // let some_txt = "3.4 + 1";

    let mut lexer: Lexer = Lexer::new(some_txt);
    // println!("Lexer: {:?}", lexer);

    // let tokens = lexer.get_token_table();
    // println!("Tokens: {:#?}", tokens);



    // for token in lexer {
    // same as iterating over the lexer...
    while let Some(token) = lexer.next() {
        debug!("{:?}", token);
    }

}
