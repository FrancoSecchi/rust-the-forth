use crate::core::error::OperationError;
use crate::core::operation::Operation;

#[derive(Debug)]
pub struct Rot;

impl Operation for Rot {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }

        let third_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let second_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let first_item = stack.pop().ok_or(OperationError::StackUnderflow)?;

        stack.push(second_item);
        stack.push(third_item);
        stack.push(first_item);
        Ok(())
    }
}
#[test]
fn test_rot_elements() {
    let mut stack: Vec<i16> = vec![1, 2, 3];
    Rot.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 3, 1]);
}

#[test]
fn test_underflow_rot() {
    let mut stack: Vec<i16> = vec![1];
    assert!(matches!(
        Rot.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));

    let mut second_stack: Vec<i16> = vec![];
    assert!(matches!(
        Rot.apply(&mut second_stack),
        Err(OperationError::StackUnderflow)
    ));
    let mut second_stack: Vec<i16> = vec![1, 2];
    assert!(matches!(
        Rot.apply(&mut second_stack),
        Err(OperationError::StackUnderflow)
    ));
}
