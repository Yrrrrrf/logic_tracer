#![allow(unused)]

use dev_utils::{app_dt, dlog::{set_max_level, Level}};

fn main() {
    app_dt!(file!());
    set_max_level(Level::Debug);
    
    

}
