use crate::core::error::OperationError;
use crate::core::operation::Operation;

#[derive(Debug)]
pub struct Div;

impl Operation for Div {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let divisor = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let dividen: i16 = stack.pop().ok_or(OperationError::StackUnderflow)?;
        if divisor == 0 {
            return Err(OperationError::DivisionByZero);
        }
        stack.push(dividen / divisor);
        Ok(())
    }
}

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
fn test_div_numbers_multiple_times() {
    let mut stack: Vec<i16> = vec![2, 6, 3];
    Div.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 2]);
    Div.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![1]);
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
