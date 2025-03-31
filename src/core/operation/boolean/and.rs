use crate::core::error::OperationError;
use crate::core::operation::Operation;

pub struct And;

impl Operation for And {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let second_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let result: i16 = if second_item != 0 && item != 0 { -1 } else { 0 };
        stack.push(result);
        Ok(())
    }
}

#[test]
fn test_and_numbers() {
    let mut stack: Vec<i16> = vec![-1, -1];
    And.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![-1]);
}
#[test]
fn test_not_and_number() {
    let mut stack: Vec<i16> = vec![-1, 0];
    And.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![0]);
    let mut stack_two: Vec<i16> = vec![0, 0];
    And.apply(&mut stack_two).unwrap();
    assert_eq!(stack_two, vec![0]);
}
#[test]
fn test_and_numbers_many_elements() {
    let mut stack: Vec<i16> = vec![2, -1, -1];
    And.apply(&mut stack).unwrap();
    assert_eq!(stack, vec![2, -1]);
}

#[test]
fn test_underflow_and() {
    let mut stack: Vec<i16> = vec![];
    assert!(matches!(
        And.apply(&mut stack),
        Err(OperationError::StackUnderflow)
    ));

    let mut second_stack: Vec<i16> = vec![1];
    assert!(matches!(
        And.apply(&mut second_stack),
        Err(OperationError::StackUnderflow)
    ));
}
