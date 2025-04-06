use crate::core::error::OperationError;
use crate::core::operation::Operation;

/// Represents the multiplication operation (`*`).
///
/// This operation pops the top two values from the stack,
/// multiplies them, and pushes the result back onto the stack.
#[derive(Debug)]
pub struct Mul;

impl Operation for Mul {
    /// Applies the multiplication operation to the provided stack.
    ///
    /// # Arguments
    ///
    /// * `stack` - A mutable reference to a stack of 16-bit integers.    
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let a = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let b = stack.pop().ok_or(OperationError::StackUnderflow)?;
        stack.push(a * b);
        Ok(())
    }
}

#[test]
fn test_mul_two_numbers() {
    let mut stack: Vec<i16> = vec![2, 3];
    Mul.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![6]);
}

#[test]
fn test_mul_last_two_numbers() {
    let mut stack: Vec<i16> = vec![2, 3, 1];
    Mul.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 3]);
}

#[test]
fn test_mul_numbers_multiple_times() {
    let mut stack: Vec<i16> = vec![2, 4, 3];
    Mul.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 12]);
    Mul.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![24]);
}

#[test]
fn test_mul_stack_underflow() {
    let mut stack: Vec<i16> = vec![1];
    assert!(matches!(
        Mul.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));
}
