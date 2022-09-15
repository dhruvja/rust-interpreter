use std::result;

#[derive(Debug, Clone, Copy)]
pub enum ByteCode {
    LoadVal(i64),
    WriteVar(char),
    ReadVar(char),
    Add,
    Mul,
	Div,
	Sub,
    Return,
}

#[derive(Debug, Clone, Copy)]
pub struct Variable {
    pub variable: Option<char>,
    pub value: i64,
}

#[derive(Clone)]
pub struct Codes {
    pub bytecodes: Vec<ByteCode>,
    pub stack: Vec<Variable>,
}

#[derive(Debug)]
pub enum CodeError {
    StackUnderflow,
}

pub type Result<T> = result::Result<T, CodeError>;