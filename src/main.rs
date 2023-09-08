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
use proto::{*, base_change::str_to_num_from_base};

fn main() {
    print_app_data();  // Print the app data
    run();  // Run the app
}


/// Run the app
pub fn run() {

    // let src = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZaz.";
    // let src = "9A.3";
    let src = "C_BA2.2C";
    let n_base = 2;
    let new_base = 18;

    println!("{} (b{:_>3}) = {} (b{:_>3})", src, n_base, 
        match str_to_num_from_base(src, n_base, new_base) {
            Ok(n) => n,
            Err(e) => {println!("Error: {}", e); return}
        }, new_base);

}
