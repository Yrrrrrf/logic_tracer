#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod config;
use config::globals::*;

mod util;
use util::terminal::*;


fn main() {
    // terminal::clear();
    println!("{} {}\n", (set_fg(APP_NAME, "green")), APP_VERSION);  // Print the app name and version
    // println!("Author: {}", terminal::set_fg(&APP_AUTHOR, "blue"));  // Print the app author


}
