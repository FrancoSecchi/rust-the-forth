use crate::core::error::OperationError;
use crate::core::operation::OperationOutput;

pub struct Dot;

impl OperationOutput for Dot {
    fn apply(
        &self,
        stack: &mut Vec<i16>,
        output: &mut String,
        _text_to_print: &str,
    ) -> Result<(), OperationError> {
        let num = stack.pop().ok_or(OperationError::StackUnderflow)?;
        output.push_str(&format!("{} ", num));
        Ok(())
    }
}

#[test]
fn test_print_number() {
    let mut stack: Vec<i16> = vec![2, 3];
    let mut output = String::new();
    Dot.apply(&mut stack, &mut output, &"".to_string()).unwrap();
    assert_eq!(stack, vec![2]);
    assert_eq!(output, "3 ");
}

#[test]
fn test_print_many_numbers() {
    let mut stack: Vec<i16> = vec![2, 3, 4, 5, 6];
    let mut output = String::new();
    Dot.apply(&mut stack, &mut output, &"".to_string()).unwrap();
    Dot.apply(&mut stack, &mut output, &"".to_string()).unwrap();
    Dot.apply(&mut stack, &mut output, &"".to_string()).unwrap();
    Dot.apply(&mut stack, &mut output, &"".to_string()).unwrap();
    assert_eq!(stack, vec![2]);
    assert_eq!(output, "6 5 4 3 ");
}

#[test]
fn test_print_number_underflow() {
    let mut stack: Vec<i16> = vec![];
    let mut output = String::new();
    assert!(matches!(
        Dot.apply(&mut stack, &mut output, &"".to_string()),
        Err(OperationError::StackUnderflow)
    ));
}
