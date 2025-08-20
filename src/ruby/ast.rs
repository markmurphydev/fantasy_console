//! Ruby AST nodes.
//! Euh, presumably we don't need this.


pub enum Value {
    True,
    False,
    Nil
}

pub enum Node {
    Const(Value)
}