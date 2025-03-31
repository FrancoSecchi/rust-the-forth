use crate::core::error::OperationError;
use crate::core::operation::Operation;

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
fn test_sub_numbers_multiple_times() {
    let mut stack: Vec<i16> = vec![2, 4, 3];
    Sub.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 1]);
    Sub.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![1]);
}

#[test]
fn test_sub_stack_underflow() {
    let mut stack: Vec<i16> = vec![1];
    assert!(matches!(
        Sub.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));
}
