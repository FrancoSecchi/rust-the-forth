
use crate::core::error::OperationError;
use crate::core::operation::Operation;
use std::collections::HashMap;

pub struct Eq;

impl Operation for Eq {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let second_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let result: i16 = if item == second_item {-1} else {0};        
        stack.push(result);        
        Ok(())
    }
}

pub struct Greater;

impl Operation for Greater {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let second_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let result: i16 = if second_item > item {-1} else {0};        
        stack.push(result);        
        Ok(())
    }
}

pub struct Less;

impl Operation for Less {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let second_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let result: i16 = if second_item < item {-1} else {0};        
        stack.push(result);        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod eq_tests {
        use super::*;
        #[test]
        fn test_eq_numbers() {
            let mut stack: Vec<i16> = vec![3, 3];
            Eq.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![-1]);
        }
        #[test]
        fn test_not_eq_numbers() {
            let mut stack: Vec<i16> = vec![2, 3];
            Eq.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![0]);
        }
        #[test]
        fn test_eq_numbers_many_elements() {
            let mut stack: Vec<i16> = vec![2, 3, 3];
            Eq.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![2, -1]);
        }

        #[test]
        fn test_underflow_eq() {
            let mut stack: Vec<i16> = vec![];
            assert!(matches!(
                Eq.apply(&mut stack),
                Err(OperationError::StackUnderflow)
            ));
            
            let mut second_stack: Vec<i16> = vec![1];
            assert!(matches!(
                Eq.apply(&mut second_stack),
                Err(OperationError::StackUnderflow)
            ));
        }
    }

    mod grater_tests {
        use super::*;
        #[test]
        fn test_greater_numbers() {
            let mut stack: Vec<i16> = vec![2, 1];
            Greater.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![-1]);
        }
        #[test]
        fn test_not_greater_number() {
            let mut stack: Vec<i16> = vec![2, 3];
            Greater.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![0]);
        }
        #[test]
        fn test_greater_numbers_many_elements() {
            let mut stack: Vec<i16> = vec![2, 4, 3];
            Greater.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![2, -1]);
        }

        #[test]
        fn test_underflow_greater() {
            let mut stack: Vec<i16> = vec![];
            assert!(matches!(
                Greater.apply(&mut stack),
                Err(OperationError::StackUnderflow)
            ));
            
            let mut second_stack: Vec<i16> = vec![1];
            assert!(matches!(
                Greater.apply(&mut second_stack),
                Err(OperationError::StackUnderflow)
            ));
        }
    }
}