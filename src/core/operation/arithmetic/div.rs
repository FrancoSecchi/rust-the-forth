use crate::core::error::OperationError;
use crate::core::operation::Operation;

/// Represents the division operation (`/`).
///
/// This operation pops the top two values from the stack,
/// divides the second-top value by the top value,
/// and pushes the result back onto the stack.
///
/// # Note
///
/// If division by zero is attempted, this operation returns
/// `OperationError::DivisionByZero`.
#[derive(Debug)]
pub struct Div;

impl Operation for Div {
    /// Applies the division operation to the provided stack.
    ///
    /// # Arguments
    ///
    /// * `stack` - A mutable reference to a stack of 16-bit integers.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the operation is successful.
    /// * `Err(OperationError::StackUnderflow)` if the stack has fewer than two elements.
    /// * `Err(OperationError::DivisionByZero)` if the divisor is zero.
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
