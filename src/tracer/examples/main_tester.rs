#![allow(unused)]

use dev_utils::{
    // todo: Add this stuff to the dev_utils::prelude
    app_dt,
    debug,
    dlog::*,
    error,
    info,
    trace,
    warn,
};


use logic_tracer::*;

fn main() {
    app_dt!(file!());
    set_max_level(Level::Debug);

    let src = "g2+3 & 3^ &";
    let lexer = Lexer::<CompleteLexer>::new(src);

    for token in lexer.clone() {
        debug!("{}", token.to_string());
    }

    for token in lexer.clone() {
        debug!("{:?}", token);
    }

    debug!("src: {}", lexer.src_code);

}
