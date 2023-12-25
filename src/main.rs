#![allow(unused)]

// ? Import modules -----------------------------------------------------------------------------------------------------------

// Standard library

// Third party crates
use dev_utils::{
    print_app_data,
    log::rlog::RLog,
};
use log::LevelFilter;

// Internal modules
mod components;
use components::*;
// * Prototyped modules (Not ready for production)
mod proto;
use proto::logic_notation::*;

// ? Main ---------------------------------------------------------------------------------------------------------------------

fn main() {
    print_app_data(file!());    
    RLog::init_logger(LevelFilter::Trace);

    // log::trace!("Starting the program");


    // ? Features
    // logic_notation::test_logic();  // * [OK]
    // logic_notation::test_var_regex();  // ! [FAIL]

    // logic_notation::test_logic_operators_regex();  // ^ [ pending ]


    println!("AS CHAR: {}", 10 as char);
    println!("AS NUM:  {}", 10);


    // let infix_expression = "(A + B) * C";
    // let infix_expression = "(1 + 4) * 2";
    // let postfix_expression = infix_to_postfix(infix_expression);
    // println!("Infix: {}", infix_expression);
    // println!("Postfix: {}", &postfix_expression);

    // // let postfix_expression = "12 3 4 * +";
    // let result = evaluate_postfix(&postfix_expression);
    // println!("Postfix: {}", postfix_expression);
    // println!("Result: {}", result);

}
