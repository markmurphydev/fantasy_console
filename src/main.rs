use fantasy_fukkou::Printer;
use fantasy_fukkou::ruby::parse::Parser;
use fantasy_fukkou::ruby::tokenize::Tokenizer;
use fantasy_fukkou::wasm::{Expr, Function, FunctionIndex, Instruction, Integer, Module};

fn main() {
    let text = "xxx";
    let tokens = Tokenizer::new(&text).tokenize();
    let program = Parser::new(tokens).parse();

    println!("Done")
}
