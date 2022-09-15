#[cfg(test)]
mod interpreter_tests {

    // use super::*;
    use crate::bytecode_types::{ByteCode::*, CodeError, Operations};
    use crate::bytecode_interpreter::interpret;

    #[test]
    fn test_err_no_value_loaded() {
        assert_eq!(
            interpret(vec![Return]).err(),
            Some(CodeError::StackUnderflow)
        );
    }

    #[test]
    fn test_value_loaded() {
        let x = 7;
        let instructions = vec![LoadVal(x), Return];

        assert_eq!(interpret(instructions).unwrap().value, x);
    }

    #[test]
    fn test_err_single_value_operation() {
        let x = 5;

        let instructions = vec![LoadVal(x), Add, Return];

        assert_eq!(
            interpret(instructions).err(),
            Some(CodeError::StackUnderflow)
        );
    }

    #[test]
    fn test_addition() {
        let x = 2;
        let y = 4;
        let sum = x + y;
        let instructions = vec![LoadVal(x), LoadVal(y), Add, Return];

        assert_eq!(interpret(instructions).unwrap().value, sum);
    }
    #[test]
    fn test_subtraction() {
        let x = 2;
        let y = 4;
        let diff = x - y;
        let instructions = vec![LoadVal(x), LoadVal(y), Sub, Return];

        assert_eq!(interpret(instructions).unwrap().value, diff);
    }
    #[test]
    fn test_multiplication() {
        let x = 2;
        let y = 4;
        let product = x * y;
        let instructions = vec![LoadVal(x), LoadVal(y), Mul, Return];

        assert_eq!(interpret(instructions).unwrap().value, product);
    }
    #[test]
    fn test_division() {
        let x = 2;
        let y = 4;
        let quotient = x / y;
        let instructions = vec![LoadVal(x), LoadVal(y), Div, Return];

        assert_eq!(interpret(instructions).unwrap().value, quotient);
    }
    #[test]
    fn test_mod() {
        let x = 2;
        let y = 4;
        let remainder = x % y;
        let instructions = vec![LoadVal(x), LoadVal(y), Mod, Return];

        assert_eq!(interpret(instructions).unwrap().value, remainder);
    }

    #[test]
    fn test_add_loop() {
        let mut x = 2;
        let mut y = 4;

        let orig_y = 4;
        let orig_x = 2;

        let start = 0;
        let end = 3;

        let mut sum = 0;

        for i in start..end {
            sum = x + y;
            x = y;
            y = sum;
        }

        let instructions = vec![LoadVal(orig_x),LoadVal(orig_y), Loop(start, end, Operations::Add), Return];

        assert_eq!(interpret(instructions).unwrap().value, sum);
    }

    #[test]
    fn test_mul_loop() {
        let mut x = 2;
        let mut y = 4;

        let orig_y = 4;
        let orig_x = 2;

        let start = 0;
        let end = 3;

        let mut product = 0;

        for i in start..end {
            product = x * y;
            x = y;
            y = product;
        }

        let instructions = vec![LoadVal(orig_x),LoadVal(orig_y), Loop(start, end, Operations::Mul), Return];

        assert_eq!(interpret(instructions).unwrap().value, product); 
    }

    #[test]
    fn test_multiple_operations() {
        let x = 2;
        let y = 4;
        let z = 6;

        let result = (x * y) + z - x;

        let instructions = vec![
            LoadVal(x),
            LoadVal(y),
            Mul,
            LoadVal(z),
            Add,
            LoadVal(x),
            Sub,
            Return,
        ];

        assert_eq!(interpret(instructions).unwrap().value, result);
    }
}
