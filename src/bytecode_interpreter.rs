use crate::bytecode_types::{ByteCode, Variable, Result, Codes, CodeError, Operations};

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

macro_rules! perform_loop_op {
    ($expression: expr, $op: tt) => {{
        let size = $expression.stack.len(); 
        if let x = $expression.stack[size-1] {
            if let y = $expression.stack.remove(size-2) {
                let val = y.value $op x.value; 
                println!("{} {}", x.value, y.value);
                println!("{val}");
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
            ByteCode::Loop(x,y, op) => {
                for i in x..y {
                    if let Some(err) = match op {
                        Operations::Add => perform_loop_op!(bytes, +),
                        Operations::Sub => perform_loop_op!(bytes, -),
                        Operations::Mul => perform_loop_op!(bytes, *),
                        Operations::Div => perform_loop_op!(bytes, /),
                        Operations::Mod => perform_loop_op!(bytes, %)
                        
                    } {
                        return Err(err);
                    };
                };
                None
            }
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