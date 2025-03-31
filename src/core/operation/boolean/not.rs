use crate::core::error::OperationError;
use crate::core::operation::Operation;

pub struct Not;

impl Operation for Not {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let result: i16 = if item != 0 { 0 } else { -1 };
        stack.push(result);
        Ok(())
    }
}

#[test]
fn test_not_numbers() {
    let mut stack: Vec<i16> = vec![10];
    Not.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![0]);
}
#[test]
fn test_not_multiple_times() {
    let mut stack: Vec<i16> = vec![10];
    Not.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![0]);
    Not.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![-1]);
}
#[test]
fn test_not_numbers_many_elements() {
    let mut stack: Vec<i16> = vec![2, -1, -1];
    Not.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, -1, 0]);
}

#[test]
fn test_underflow_not() {
    let mut stack: Vec<i16> = vec![];
    assert!(matches!(
        Not.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));
}