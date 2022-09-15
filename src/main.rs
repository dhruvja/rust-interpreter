use crate::bytecode_types::{ByteCode, Variable, Result};

pub mod bytecode_types;

fn main() {
    println!("Hello, world!");
}

pub fn interpret(bytecodes: Vec<ByteCode>) -> Result<Variable> {
    
}
