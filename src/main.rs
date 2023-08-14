#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


mod config;
use config::globals::*;

mod util;
use util::terminal::*;

mod components;
use components::*;

mod circuits;
use circuits::sequential::*;
use circuits::combinational::*;

pub mod gates;

use crate::components::ast::AST;

use crate::gates::*;


use colorful;  // Colorful text
use colorful::Color;  // Enum for colors
use colorful::Colorful;  // Trait for coloring strings with ANSI escape codes


fn main() {
    // clear();  // Clear the terminal
    println!("{} {}\n", (set_fg(APP_NAME, "g")), APP_VERSION);  // Print the app name and version
    // println!("Author: {}\n", set_fg(APP_AUTHOR, "blue"));  // Print the app author
    // println!("{} ", set_fg("Loading", "y"));    

    run();  // Run the app

    // cargo watch -q -c -x 'run -q'  // Run the app, quite, clear terminal
    // cargo doc --no-deps --open
}


/// Run the app
pub fn run() {

    let mut lexer = AST::new("Hello!");
    println!("{:?}", lexer);
    println!("{:#?}", lexer);
    // ● ○


    // test_colorful_crate();
}


pub fn test_colorful_crate() {
    println!("{}", "This code is editable and runnable!".gradient(Color::Red));
    println!("{}", "¡Este código es editable y ejecutable!".gradient(Color::Green));
    println!("{}", "Ce code est modifiable et exécutable !".gradient(Color::Yellow));
    println!("{}", "Questo codice è modificabile ed eseguibile!".gradient(Color::Blue));
    println!("{}", "このコードは編集して実行出来ます！".gradient(Color::Magenta));
    println!("{}", "여기에서 코드를 수정하고 실행할 수 있습니다!".gradient(Color::Cyan));
    println!("{}", "Ten kod można edytować oraz uruchomić!".gradient(Color::LightGray));

}