use crate::core::error::OperationError;
use crate::core::operation::Operation;

/// Represents the addition operation (`+`).
///
/// This operation pops the top two values from the stack,
/// adds them, and pushes the result back onto the stack.
#[derive(Debug)]
pub struct Add;

impl Operation for Add {
    /// Applies the addition operation to the provided stack.
    ///
    /// # Arguments
    ///
    /// * `stack` - A mutable reference to a stack of 16-bit integers.
    ///
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let a = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let b = stack.pop().ok_or(OperationError::StackUnderflow)?;
        stack.push(a + b);
        Ok(())
    }
}

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
fn test_add_numbers_multiple_times() {
    let mut stack: Vec<i16> = vec![2, 3, 3];
    Add.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 6]);
    Add.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![8]);
}

#[test]
fn test_add_stack_underflow() {
    let mut stack: Vec<i16> = vec![1];
    assert!(matches!(
        Add.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));
}
