use crate::core::error::OperationError;
use crate::core::operation::Operation;
pub struct Less;

impl Operation for Less {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let second_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let result: i16 = if second_item < item { -1 } else { 0 };
        stack.push(result);
        Ok(())
    }
}

#[test]
fn test_less_numbers() {
    let mut stack: Vec<i16> = vec![1, 2];
    Less.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![-1]);
}
#[test]
fn test_not_less_number() {
    let mut stack: Vec<i16> = vec![3, 2];
    Less.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![0]);
}
#[test]
fn test_less_numbers_many_elements() {
    let mut stack: Vec<i16> = vec![2, 4, 3];
    Less.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, 0]);
}

#[test]
fn test_underflow_less() {
    let mut stack: Vec<i16> = vec![];
    assert!(matches!(
        Less.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));

    let mut second_stack: Vec<i16> = vec![1];
    assert!(matches!(
        Less.apply(&mut second_stack),
        Err(OperationError::StackUnderflow)
    ));
}
