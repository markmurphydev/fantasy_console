use fantasy_fukkou::Printer;
use fantasy_fukkou::wasm::{Expr, Function, FunctionIndex, Instruction, Integer, Module};

fn main() {
    let zero_module = Module {
        functions: vec![
            Function {
                name: Some("main".to_string()),
                body: Expr(vec![Instruction::ConstI64(Integer(0))]),
            }
        ],
        start: FunctionIndex::Name("main".to_string()),
    };

    let printer = Printer::new();
    printer.print_module(&zero_module);
}
