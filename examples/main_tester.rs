#![allow(unused)]


use log::*;
use dev_utils::{
    print_app_data, 
    log::rlog::RLog
};
use logic_tracer::lexer::*;
use logic_tracer::parser::*;


#[macro_use]
macro_rules! impl_some_lexer_test {
    (
        $lexer:ty;
        $( $txt:expr ),+
        $(,)?
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
    print_app_data(file!());
    // RLog::init_logger(LevelFilter::Info);
    RLog::init_logger(LevelFilter::Debug);
    // RLog::init_logger(LevelFilter::Trace);

    impl_some_lexer_test!(
        CompleteLexer; 
        // MathLexer; 
        // LogicLexer; 

        "- 123 + 17.6 - 6.4",
        "3.4 + 1",
        "25.1 * 42 - 13",
        "25.1 \\Gφx * φ 42 - 13.6",
        "25.1 \\GQxyz * φ - 13.6",
        "\\GG * φ - 0.9849xXQ_2",
    );

}
