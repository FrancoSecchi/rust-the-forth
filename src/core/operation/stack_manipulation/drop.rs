use crate::core::error::OperationError;
use crate::core::operation::Operation;

#[derive(Debug)]
pub struct Drop;

impl Operation for Drop {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        stack.pop().ok_or(OperationError::StackUnderflow)?;
        Ok(())
    }
}
#[test]
fn test_drop_last_number() {
    let mut stack: Vec<i16> = vec![2, 3];
    Drop.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2]);
}

#[test]
fn test_underflow_drop() {
    let mut stack: Vec<i16> = vec![];
    assert!(matches!(
        Drop.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));
}
