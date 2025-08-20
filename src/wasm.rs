//! A representation of the _compiled_ wasm program.
//! Should be trivially convertible to .wat

use std::fmt::{Display, Formatter};

// ==== Wasm Values ====
pub struct Byte(pub u8);

// TODO -- ugggh
#[derive(Debug, Copy, Clone)]
pub struct Integer(pub u64);

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// ==== Wasm Types ====

#[derive(Copy, Clone)]
pub enum Instruction {
    ConstI64(Integer)
}

impl Instruction {
    pub const FALSE: Instruction = Instruction::ConstI64(Integer(0b0001));
    pub const TRUE: Instruction = Instruction::ConstI64(Integer(0b0011));
    pub const NIL: Instruction = Instruction::ConstI64(Integer(0b0111));
}

pub enum FunctionIndex {
    // TODO -- Spec defines indices to be wasm-u32
    // https://webassembly.github.io/spec/core/syntax/modules.html#syntax-start
    Index(usize),
    Name(String),
}

pub struct Function {
    pub name: Option<String>,
    pub body: Expr,
}

/// Sequence of instructions terminated by an `end` marker
/// https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-expr
pub struct Expr(pub Vec<Instruction>);

pub struct Module {
    pub functions: Vec<Function>,
    pub start: Option<FunctionIndex>,
}
