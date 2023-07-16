#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


mod config;
use config::globals::*;

mod util;
use logic_tracer::lexer::Lexer;
use util::terminal::*;

mod components;
use components::*;

mod circuits;
use circuits::sequential::*;
use circuits::combinational::*;



fn main() {
    // clear();  // Clear the terminal
    println!("{} {}\n", (set_fg(APP_NAME, "g")), APP_VERSION);  // Print the app name and version
    // println!("Author: {}\n", set_fg(APP_AUTHOR, "blue"));  // Print the app author
    // println!("{} ", set_fg("Loading", "y"));    

    run();  // Run the app

    // cargo watch -q -c -x 'run -q'  // Run the app, quite, clear terminal
    // cargo doc --open


    let mut lexer = Lexer::new("Hello World!");
    println!("{:?}", lexer);
    println!("{:#?}", lexer);

}

/// Run the app
pub fn run() {
    let test_var = "Hello World!";
    println!("{}", test_var);
    // ●
    // ○ 

}