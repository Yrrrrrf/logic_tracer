#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


// Import modules
mod config;
use crate::config::globals::*;

mod util;
use crate::util::terminal::*;


mod components;
use components::lexer::{*, self};

fn main() {
    // clear();  // Clear the terminal
    println!("{} {}", (set_fg(APP_NAME, "g")), APP_VERSION);  // Print the app name and version
    println!("Author: {}", set_fg(APP_AUTHOR, "blue"));  // Print the app author
    // println!("{}", set_fg("ProtoType", "y"));    
    print!("\n");


    run();  // Run the app
}

/// Run the app
pub fn run() {
    // ● ○


    let mut lexer = Lexer::new("A & B | C & D");

    println!("{}", Lexer::new("A & (B & C)").check_pair_brackets());
    println!("{}", Lexer::new("A & (B & C) & D").check_pair_brackets());
    println!("{}", Lexer::new("(a + b) * (c - d)").check_pair_brackets());
    println!("{}", Lexer::new("(a + b) * (c - d]").check_pair_brackets());
    println!("{}", Lexer::new("x + y] * [z - w]").check_pair_brackets());
    println!("{}", Lexer::new("x + y] * [z - w)").check_pair_brackets());
    println!("{}", Lexer::new("1, 2, 3, 4}").check_pair_brackets());
    println!("{}", Lexer::new("1, 2, 3, 4]").check_pair_brackets());
    println!("{}", Lexer::new("html></html>").check_pair_brackets());
    println!("{}", Lexer::new("html></htm>").check_pair_brackets());
    println!("{}", Lexer::new("(").check_pair_brackets());
    println!("{}", Lexer::new("[").check_pair_brackets());
    println!("{}", Lexer::new("{").check_pair_brackets());
    println!("{}", Lexer::new("<").check_pair_brackets());
    println!("{}", Lexer::new("[{()}]").check_pair_brackets());
    println!("{}", Lexer::new("{[()]}>").check_pair_brackets());
    println!("{}", Lexer::new(" ").check_pair_brackets());
    println!("{}", Lexer::new("").check_pair_brackets());

    // print!("{:?}", lexer.src);

    // println!("{:?}", lexer.reduced_src);


}
