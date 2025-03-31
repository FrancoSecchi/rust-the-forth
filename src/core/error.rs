use std::fmt;

#[derive(Debug)]
pub enum OperationError {
    StackUnderflow,
    DivisionByZero,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::StackUnderflow => writeln!(f, "stack-underflow"),
            OperationError::DivisionByZero => writeln!(f, "division-by-zero"),
        }
    }
}
