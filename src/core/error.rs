use std::fmt;

#[derive(Debug)]
pub enum OperationError {
    StackUnderflow,
    DivisionByZero,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::StackUnderflow => write!(f, "stack-underflow\n"),
            OperationError::DivisionByZero => write!(f, "division-by-zero\n"),
        }
    }
}
