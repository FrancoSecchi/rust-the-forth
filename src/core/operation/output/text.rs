use crate::core::error::OperationError;
use crate::core::operation::OperationOutput;

pub struct PrintText;

impl OperationOutput for PrintText {
    fn apply(
        &self,
        _stack: &mut Vec<i16>,
        output: &mut String,
        text_to_print: &str,
    ) -> Result<(), OperationError> {
        if text_to_print.starts_with(".\"") && text_to_print.ends_with('"') {
            let text = &text_to_print[2..text_to_print.len() - 1];
            output.push_str(text);
            return Ok(());
        }
        Ok(())
    }
}

#[test]
fn test_print_text_basic() {
    let mut stack: Vec<i16> = vec![];
    let mut output = String::new();

    PrintText
        .apply(&mut stack, &mut output, &".\" hello world\"".to_string())
        .unwrap();

    assert_eq!(stack, vec![]);
    assert_eq!(output, " hello world");
}

#[test]
fn test_print_text_multiple_words() {
    let mut stack: Vec<i16> = vec![2, 3];
    let mut output = String::new();

    PrintText
        .apply(&mut stack, &mut output, &".\" hello world\"".to_string())
        .unwrap();

    assert_eq!(stack, vec![2, 3]);
    assert_eq!(output, " hello world");
}

#[test]
fn test_print_text_empty() {
    let mut stack: Vec<i16> = vec![];
    let mut output = String::new();

    PrintText
        .apply(&mut stack, &mut output, &".\" \"".to_string())
        .unwrap();

    assert_eq!(stack, vec![]);
    assert_eq!(output, " ");
}

#[test]
fn test_print_text_with_numbers_before() {
    let mut stack: Vec<i16> = vec![42];
    let mut output = String::new();

    PrintText
        .apply(&mut stack, &mut output, &".\" test\"".to_string())
        .unwrap();

    assert_eq!(stack, vec![42]);
    assert_eq!(output, " test");
}
