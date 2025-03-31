use crate::core::error::OperationError;
use crate::core::operation::Operation;

#[derive(Debug)]
pub struct Dup;

impl Operation for Dup {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;

        stack.push(item);
        stack.push(item);
        Ok(())
    }
}
#[test]
fn test_dup_last_number() {
    let mut stack: Vec<i16> = vec![2, 3];
    Dup.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 3, 3]);
}

#[test]
fn test_underflow_dup() {
    let mut stack: Vec<i16> = vec![];
    assert!(matches!(
        Dup.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));
}
