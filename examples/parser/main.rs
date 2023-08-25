extern crate logic_tracer;

use logic_tracer::parser::Parser;


fn main() {
    println!("Parser example (1)");


    let parser = Parser::new("A & B");

    print!("{:?}", parser);
    panic!("Imlpement the parser example (1)")

}


