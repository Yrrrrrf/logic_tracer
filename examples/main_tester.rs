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
use logic_tracer::lexer::*;
use logic_tracer::parser::*;

#[macro_use]
macro_rules! impl_some_lexer_test {
    (
        $lexer:ty;  // Lexer type
        $( $txt:expr ),+  // any str to analyse
        $(,)?  // as many as paramerts passed
    ) => {
        $(
            for token in Lexer::<$lexer>::new($txt) {
                // debug!("{:?}", token);
                debug!("{}", token.to_string());
            }
        )+
    };
}

fn main() {
    app_dt!(file!());
    set_max_level(Level::Debug);

    debug!("Testing lexer...");
    trace!("Some trace\n\n\n\nPenchs");

    impl_some_lexer_test!(
        CompleteLexer;
        // MathLexer;
        // LogicLexer;
        "23 - 45",
        "3.4 + 1",
        "- 123 + 17.6 - 6.4",
        "xyz",
        "25.1 * 42 - 13",
        "25.1 \\Gφx * φ 42 - 13.6",
        "\\GG * φ - 0.9849xXQ_2",
    );
}
