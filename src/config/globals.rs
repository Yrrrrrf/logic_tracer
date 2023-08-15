// globals is a module that contains global variables that are used throughout the application

// Read the doc ./../Cargo.toml to gett the constan settings

use std::fs::File;
use::std::io;  // io library is part of the standard library (std)
use::std::io::{Write, Read};  // io library is part of the standard library (std) (Write trait)
use std::str::FromStr;  // io library is part of the standard library (std) (Read trait)


// todo: read the constants from the Cargo.toml file
// fn read_toml() -> Result<(), io::Error> {
//     let mut file = File::open("Cargo.toml")?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     println!("File contents: {}", contents);
//     Ok(())
// }  


// global variables (constants)
pub const APP_NAME: &str = "Logic Tracer";
pub const APP_VERSION: &str = "v0.1.0";
pub const APP_AUTHOR: &str = "Yrrrrrf";

// paths
pub const DATA_PATH: &str = ".\\resources\\data\\";
pub const IMG_PATH: &str = ".\\resources\\img\\";
// pub const FONT_PATH: &str = ".\\resources\\fonts\\";
