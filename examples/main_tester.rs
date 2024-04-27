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
    RLog::init_logger(LevelFilter::Debug);
    // RLog::init_logger(LevelFilter::Trace);


    // * Number types
    let nums = [
        "123",
        "-456",
        "7.892",
        "-456.16",
        // "10.0i",
    ];
    nums.iter().for_each(|num| {
        // call the string as a number...
        if let Some(number) = Number::from(*num) {
            // debug!("{:?}", number);

            // match number.t_type() {
            //     "Natural" => {
            //         // println!("{:?}", &number);
            //         // let num: f64 = number.into();
            //     },
            //     _ => {},
            // }
        }
        // if type == Natural


    
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
        // call the char as an operator...
        if let Some(opeator) = Operator::from(*op) {
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




    let some_txt = "abcde2 -123 + 17.6 ";
    let mut lexer = Lexer::new(some_txt);

    for i in 0..=some_txt.len() {
        let token = lexer.next();
        debug!("{:?}", token);
    }






    // for token in lexer {
    //     debug!("{:?}", token);
    // }

}

