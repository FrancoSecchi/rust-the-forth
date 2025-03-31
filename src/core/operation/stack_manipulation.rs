use crate::core::error::OperationError;
use crate::core::operation::Operation;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dup;

impl Operation for Dup {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;

        stack.push(item);
        stack.push(item);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Drop;

impl Operation for Drop {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        stack.pop().ok_or(OperationError::StackUnderflow)?;
        Ok(())
    }
}

//DUP, DROP, SWAP, OVER, ROT


#[derive(Debug)]
pub struct Swap;

impl Operation for Swap {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let last_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let penultimate_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        stack.push(last_item);
        stack.push(penultimate_item);
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    mod dup_tests {
        use super::*;
        #[test]
        fn test_dup_last_number() {
            let mut stack: Vec<i16> = vec![2, 3];
            Dup.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![2, 3, 3]);
        }

        #[test]
        fn test_underflow_dup() {
            let mut stack: Vec<i16> = vec![];
            assert!(matches!(
                Dup.apply(&mut stack),
                Err(OperationError::StackUnderflow)
            ));
        }
    }

    mod drop_tests {
        use super::*;
        #[test]
        fn test_drop_last_number() {
            let mut stack: Vec<i16> = vec![2, 3];
            Drop.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![2]);
        }

        #[test]
        fn test_underflow_drop() {
            let mut stack: Vec<i16> = vec![];
            assert!(matches!(
                Drop.apply(&mut stack),
                Err(OperationError::StackUnderflow)
            ));
        }
    }


    mod swap_tests {
        use super::*;
        #[test]
        fn test_swap_last_number() {
            let mut stack: Vec<i16> = vec![2, 3];
            Swap.apply(&mut stack).unwrap();
            assert_eq!(stack, vec![3, 2]);
        }

        #[test]
        fn test_underflow_swap() {
            let mut stack: Vec<i16> = vec![1];
            assert!(matches!(
                Swap.apply(&mut stack),
                Err(OperationError::StackUnderflow)
            ));

            let mut second_stack: Vec<i16> = vec![];
            assert!(matches!(
                Swap.apply(&mut second_stack),
                Err(OperationError::StackUnderflow)
            ));
        }
    }
}
