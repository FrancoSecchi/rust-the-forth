use crate::core::error::OperationError;
use crate::core::operation::Operation;

/// Represents the subtraction operation (`-`).
///
/// This operation pops the top two values from the stack,
/// subtracts the top value from the second-top value,
/// and pushes the result back onto the stack.
#[derive(Debug)]
pub struct Sub;

impl Operation for Sub {
    /// Applies the subtraction operation to the provided stack.
    ///
    /// # Arguments
    ///
    /// * `stack` - A mutable reference to a stack of 16-bit integers.
    ///    
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
