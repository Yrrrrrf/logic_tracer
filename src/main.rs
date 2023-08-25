#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]


// Import modules
mod config;
use crate::config::globals::*;

mod util;
use crate::util::terminal::*;


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

}
