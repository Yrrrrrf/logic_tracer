#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


// Import modules
// UTIL
mod util;
use crate::util::terminal::*;

// APP COMPONENTS (MODULES)
mod components;
use components::*;

// ? Proto: Dev mode <Some test scripts for development (not for production)>
mod proto;
use proto::*;

fn main() {
    print_app_data();  // Print the app data
    run();  // Run the app
}

/// Run the app
pub fn run() {
    // ○ ○
    // ○ ●
    // ● ○
    // ● ●

    // logic_notation::run_prototype();  // Run the prototype

    // let token_table = ast::AST::new("A&(B|C)|D").get_token_table();  // Evaluate the logic of the proposition
    // println!("{:#?}", token_table);

    // let mut ast = ast::AST::new("A+B+C+D");
    // println!("{:#?}", ast.get_token_table());
}
