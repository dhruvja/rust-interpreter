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
    Mod,
    Loop(usize, usize, Operations),
    Return,
}

#[derive(Debug, Clone, Copy)]
pub enum Operations{
    Add,
    Sub,
    Mul,
    Div,
    Mod
}

#[derive(Debug, Clone, Copy)]
pub struct Variable {
    pub variable: Option<char>,
    pub value: i64,
}

#[derive(Clone, Debug)]
pub struct Codes {
    pub bytecodes: Vec<ByteCode>,
    pub stack: Vec<Variable>,
}

#[derive(Debug, PartialEq)]
pub enum CodeError {
    StackUnderflow,
    VariableNotFound
}

pub type Result<T> = result::Result<T, CodeError>;