mod lexer;
mod parser;
mod transpiler;
mod compiletask;

use std::fs;
use lexer::lex;
use parser::parse_tokens;
use transpiler::transpile;

fn main() {
    let source = fs::read_to_string("main.fp").expect("File read error");

    let tokens = lex(&source);
    let program = parse_tokens(tokens);
    println!("{:?}", program);
    transpile(program, "main");
}
