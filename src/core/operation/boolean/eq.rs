use crate::core::error::OperationError;
use crate::core::operation::Operation;

pub struct Eq;

impl Operation for Eq {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let second_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let result: i16 = if item == second_item { -1 } else { 0 };
        stack.push(result);
        Ok(())
    }
}

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
