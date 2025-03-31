use std::collections::HashMap;

use crate::core::error::OperationError;

pub trait Operation {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError>;
}

#[derive(Debug)]
pub struct Add;

impl Operation for Add {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let a = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let b = stack.pop().ok_or(OperationError::StackUnderflow)?;
        stack.push(a + b);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Sub;

impl Operation for Sub {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let a = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let b = stack.pop().ok_or(OperationError::StackUnderflow)?;
        stack.push(b - a);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Mul;

impl Operation for Mul {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let a = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let b = stack.pop().ok_or(OperationError::StackUnderflow)?;
        stack.push(a * b);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Div;

impl Operation for Div {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let a = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let b = stack.pop().ok_or(OperationError::StackUnderflow)?;
        if a == 0 {
            return Err(OperationError::DivisionByZero)
        }
        stack.push(b / a);
        Ok(())
    }
}

pub fn get_operations() -> HashMap<String, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.insert("+".to_string(), Box::new(Add) as Box<dyn Operation>);
    ops.insert("-".to_string(), Box::new(Sub) as Box<dyn Operation>);
    ops.insert("*".to_string(), Box::new(Mul) as Box<dyn Operation>);
    ops.insert("/".to_string(), Box::new(Div) as Box<dyn Operation>);
    ops
}


#[cfg(test)] 
mod tests {
    use super::*;

    mod add_tests {
        use super::*;
        
        #[test]
        fn test_add_two_numbers() {
            let mut stack: Vec<i16> = vec![2, 3];
            Add.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![5]);
        }
        
        #[test]
        fn test_add_last_two_numbers() {
            let mut stack: Vec<i16> = vec![2, 3, 3];
            Add.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![2, 6]);
        }
        
        #[test]
        fn test_add_stack_underflow() {
            let mut stack: Vec<i16> = vec![1];
            assert!(matches!(
                Add.apply(&mut stack),
                Err(OperationError::StackUnderflow)
            ));
        }
    }

    mod sub_tests {
        use super::*;
        
        #[test]
        fn test_sub_two_numbers() {
            let mut stack: Vec<i16> = vec![2, 3];
            Sub.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![-1]);
        }

        #[test]
        fn test_sub_last_two_numbers() {
            let mut stack: Vec<i16> = vec![2, 3, 1];
            Sub.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![2, 2]);
        }
        

        #[test]
        fn test_sub_stack_underflow() {
            let mut stack: Vec<i16> = vec![1];
            assert!(matches!(
                Add.apply(&mut stack),
                Err(OperationError::StackUnderflow)
            ));
        }
    }
        
    mod divide_tests {
        use super::*;
        
        #[test]
        fn test_divide_normal() {
            let mut stack: Vec<i16> = vec![6, 2];
            Div.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![3]);
        }

        #[test]
        fn test_divide_last_two_numbers() {
            let mut stack: Vec<i16> = vec![6, 4, 2];
            Div.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![6, 2]);
        }

        #[test]
        fn test_truncate_to_zero_divide() {
            let mut stack: Vec<i16> = vec![2, 4];
            Div.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![0]);
        }
        
        #[test]
        fn test_divide_by_zero() {
            let mut stack: Vec<i16> = vec![1, 0];
            assert!(matches!(
                Div.apply(&mut stack),
                Err(OperationError::DivisionByZero)
            ));
        }
    }
}
