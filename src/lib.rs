use std::iter;
use crate::wasm::{Expr, Function, FunctionIndex, Instruction, Module};

pub mod wasm;

pub struct Printer {
    indent: usize,
}

impl Printer {
    pub fn new() -> Self {
        Printer {
            indent: 0
        }
    }

    pub fn print_module(mut self, module: &Module) {
        self.indent();
        print!("(module");
        self.indent += 2;
        self.print_functions(module);
        self.print_start_function(module);
        print!(")");
        self.indent -= 2;
        assert_eq!(self.indent, 0);
    }

    fn print_functions(&mut self, module: &Module) {
        for function in &module.functions {
            self.print_function(function)
        }
    }

    fn print_function(&mut self, function: &Function) {
        println!();
        self.indent();
        print!("(func");
        if let Some(name) = &function.name {
            print!(" ${}", name);
        }
        // TODO -- (type ...)
        self.indent += 2;
        self.print_function_body(&function.body);
        print!(")");
        self.indent -= 2;
    }

    fn print_function_body(&mut self, body: &Expr) {
        for instr in body.0.iter() {
            self.print_instr(instr);
        }
    }

    fn print_instr(&mut self, instr: &Instruction) {
        println!();
        self.indent();
        match instr {
            Instruction::ConstI64(n) => print!("i64.const {}", n)
        }
    }

    fn print_start_function(&mut self, module: &Module) {
        if let Some(start_idx) = &module.start {
            println!();
            self.indent();
            print!("(start");
            match start_idx {
                FunctionIndex::Index(idx) => {
                    print!(" {}", idx);
                }
                FunctionIndex::Name(name) => {
                    print!(" ${}", name)
                }
            }
            print!(")");
        }
    }

    fn indent(&self) {
        print!("{}", iter::repeat(' ').take(self.indent).collect::<String>());
    }
}


