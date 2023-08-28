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
use components::lexer::{*, self};


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

}
