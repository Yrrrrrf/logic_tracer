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


fn main() {
    // clear();  // Clear the terminal
    println!("{} {}", (set_fg(APP_NAME, "g")), APP_VERSION);  // Print the app name and version
    // println!("Author: {}\n", set_fg(APP_AUTHOR, "blue"));  // Print the app author
    // println!("{} ", set_fg("Loading", "y"));    

    run();  // Run the app
    // cargo watch -q -c -x 'run -q'  // Run the app in quiet mode

}

pub fn run() {
    // let mut config = config
    
    let test_var = "Hello World!";
    println!("{}", test_var);
}