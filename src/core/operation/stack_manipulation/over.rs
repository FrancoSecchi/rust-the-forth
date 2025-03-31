use crate::core::error::OperationError;
use crate::core::operation::Operation;

#[derive(Debug)]
pub struct Over;

impl Operation for Over {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let last_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let penultimate_item = stack.pop().ok_or(OperationError::StackUnderflow)?;

        stack.push(penultimate_item);
        stack.push(last_item);
        stack.push(penultimate_item);
        Ok(())
    }
}

#[test]
fn test_over_last_number() {
    let mut stack: Vec<i16> = vec![2, 3];
    Over.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 3, 2]);
}

#[test]
fn test_underflow_over() {
    let mut stack: Vec<i16> = vec![1];
    assert!(matches!(
        Over.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));

    let mut second_stack: Vec<i16> = vec![];
    assert!(matches!(
        Over.apply(&mut second_stack),
        Err(OperationError::StackUnderflow)
    ));
}
