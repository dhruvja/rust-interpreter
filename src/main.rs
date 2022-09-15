use crate::bytecode_types::{ByteCode, Variable, Result, Codes, CodeError};

pub mod bytecode_types;
pub mod tests;

fn main() {
    println!("Hello, world!");
}

macro_rules! perform_op {
    ($expression: expr, $op: tt) => {{
        if let Some(x) = $expression.stack.pop() {
            if let Some(y) = $expression.stack.pop() {
                let val = y.value $op x.value; 
                $expression.stack.push(Variable {
                    variable: None,
                    value: val 
                });
                None
            }
            else { Some (CodeError::StackUnderflow)}
        }
        else { Some (CodeError::StackUnderflow)}
    }}
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
                let loaded_value = bytes.stack.pop().unwrap();
                bytes.stack.push(Variable {
                    variable: Some(c),
                    value: loaded_value.value
                });
                None
            },
            ByteCode::ReadVar(c) => {
                let read_value = bytes.stack.iter().find(|&&item| item.variable == Some(c)).unwrap();
                bytes.stack.push(Variable {
                    variable: read_value.variable,
                    value: read_value.value
                });
                None
            },
            ByteCode::Add => perform_op!(bytes, +),
            ByteCode::Sub => perform_op!(bytes, -),
            ByteCode::Mul => perform_op!(bytes, *),
            ByteCode::Div => perform_op!(bytes, /),
            ByteCode::Mod => perform_op!(bytes, %),
            ByteCode::Return => break,
        } {
            return Err(err);
        }
    };

    if let Some(val) = bytes.stack.pop() {
        Ok(val)
    }
    else {
        Err(CodeError::StackUnderflow)
    }


}
