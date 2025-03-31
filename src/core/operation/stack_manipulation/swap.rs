use crate::core::error::OperationError;
use crate::core::operation::Operation;

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
