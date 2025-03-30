use std::fmt;
use std::fmt::Error;

#[derive(Debug)]
pub enum OperationError {
    StackUnderflow,
    DivisionByZero,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::StackUnderflow => write!(f, "Stack underflow"),
            OperationError::DivisionByZero => write!(f, "Division by zero"),
        }
    }
}
