use std::io;

use lexer::get_token_stream;

use crate::parser::Parser;

mod lexer;
mod parser;

fn main() {
    // let source = "(23-43.1)/7.23e-13*12";
    println!("Enter some equation using *,/,+,-,( and )");
    let mut source = String::new();
    io::stdin()
        .read_line(&mut source)
        .expect("failed reading line from stdin");
    let tokens = get_token_stream(&source);
    println!("Tokenstream: {tokens:?}");
    let parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("AST: {ast}");
    let result = ast.eval();
    println!("Result: {result}")
}
