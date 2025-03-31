use crate::core::error::OperationError;
use crate::core::operation::OperationOutput;

pub struct Cr;

impl OperationOutput for Cr {
    fn apply(&self, _stack: &mut Vec<i16>, output: &mut String) -> Result<(), OperationError> {
        output.push_str("\n");
        Ok(())
    }
}

#[test]
fn test_cr_number() {
    let mut stack: Vec<i16> = vec![2, 3];
    let mut output = String::new();
    Cr.apply(&mut stack, &mut output).unwrap();
    assert_eq!(stack, vec![2,3]);
    assert_eq!(output, "\n");
}

#[test]
fn test_cr_many_numbers() {
    let mut stack: Vec<i16> = vec![2, 3, 4, 5, 6];
    let mut output = String::new();
    Cr.apply(&mut stack, &mut output).unwrap();
    Cr.apply(&mut stack, &mut output).unwrap();
    Cr.apply(&mut stack, &mut output).unwrap();
    Cr.apply(&mut stack, &mut output).unwrap();
    assert_eq!(stack, vec![2, 3, 4, 5, 6]);
    assert_eq!(output, "\n\n\n\n");
}
