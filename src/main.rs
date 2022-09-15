use crate::bytecode_types::{ByteCode, Variable, Result, Codes};

pub mod bytecode_types;

fn main() {
    println!("Hello, world!");
}

pub fn interpret(bytecodes: Vec<ByteCode>) -> Result<Variable> {

    let mut bytes = Codes {
        bytecodes,
        stack: Vec::new()
    };

    for bytecode in bytes.bytecodes {
        if let Some(err) = match bytecode {
            ByteCode::LoadVal(i) => {
                bytes.stack.push(Variable {
                    variable: None,
                    value: i
                });
                None
            },
            ByteCode::WriteVar(c) => {
                let loaded_value = bytes.stack.pop().unwrap_or_else(|| CodeError::StackUnderflow);
                bytes.stack.push(Variable {
                    variable: Some(c),
                    value: loaded_value
                });
                None
            },
            ByteCode::ReadVar(c) => {
                let read_value = bytes.stack.iter().find(|&&item| item.variable = Some(c)).unwrap_or_else(|| CodeError::VariableNotFound);
                bytes.stack.push(Variable {
                    variable: Some(read_value.variable),
                    value: read_value.value
                });
                None
            },
            ByteCode::Add => {

            },
            ByteCode::Sub => {

            },
            ByteCode::Mul => {

            },
            ByteCode::Div => {

            },
            ByteCode::Return => {

            }
        }
    };
    Ok(())

}
