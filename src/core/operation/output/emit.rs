use crate::core::error::OperationError;
use crate::core::operation::OperationOutput;

pub struct Emit;

impl OperationOutput for Emit {
    fn apply(
        &self,
        stack: &mut Vec<i16>,
        output: &mut String,
        _text_to_print: &str,
    ) -> Result<(), OperationError> {
        let num = stack.pop().ok_or(OperationError::StackUnderflow)?;
        if let Some(ch) = char::from_u32(num as u32) {
            output.push_str(&format!("{} ", ch));
            Ok(())
        } else {
            Err(OperationError::InvalidCharacter)
        }
    }
}

#[test]
fn test_emit_lower_case_number() {
    let mut stack: Vec<i16> = vec![97];
    let mut output = String::new();

    Emit.apply(&mut stack, &mut output, &"".to_string())
        .unwrap();
    assert_eq!(stack, vec![]);
    assert_eq!(output, "a ");
}

#[test]
fn test_emit_upper_case_number() {
    let mut stack: Vec<i16> = vec![65];
    let mut output = String::new();

    Emit.apply(&mut stack, &mut output, &"".to_string())
        .unwrap();
    assert_eq!(stack, vec![]);
    assert_eq!(output, "A ");
}
#[test]

fn test_emit_multiple_numbers() {
    let mut stack: Vec<i16> = vec![68, 67, 66, 65];
    let mut output = String::new();

    Emit.apply(&mut stack, &mut output, &"".to_string())
        .unwrap();
    Emit.apply(&mut stack, &mut output, &"".to_string())
        .unwrap();
    Emit.apply(&mut stack, &mut output, &"".to_string())
        .unwrap();
    Emit.apply(&mut stack, &mut output, &"".to_string())
        .unwrap();
    assert_eq!(stack, vec![]);
    assert_eq!(output, "A B C D ");
}
