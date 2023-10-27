#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


// ? Import modules -----------------------------------------------------------------------------------------------------------

use dev_utils::{
    rlog::RLog, 
    terminal::{self, set_fg}, 
};
// ? Extern crates
use log::LevelFilter;

mod components;
use components::*;

// ? Proto: Dev mode ----------------------------------------------------------------------------------------------------------
mod proto;
use proto::logic_notation::*;

// ? Main ---------------------------------------------------------------------------------------------------------------------

fn main() {
    dev_utils::print_app_data();    
    RLog::init_logger(LevelFilter::Trace);  // Initialize the logger with the given log level,

//     log::trace!("Starting the program");


    // ? Features
    // logic_notation::test_logic();  // * [OK]
    // logic_notation::test_var_regex();  // ! [FAIL]

    // logic_notation::test_logic_operators_regex();  // ^ [ pending ]

    let infix_expression = "(A + B) * C";
    let infix_expression = "(1 + 4) * 2";
    let postfix_expression = infix_to_postfix(infix_expression);
    println!("Infix: {}", infix_expression);
    println!("Postfix: {}", &postfix_expression);

    // let postfix_expression = "12 3 4 * +";
    let result = evaluate_postfix(&postfix_expression);
    println!("Postfix: {}", postfix_expression);
    println!("Result: {}", result);

}


