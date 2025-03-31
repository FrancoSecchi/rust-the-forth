use std::fmt;

#[derive(Debug)]
pub enum OperationError {
    StackUnderflow,
    StackOverflow,
    DivisionByZero,
    InvalidWord,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::StackUnderflow => writeln!(f, "stack-underflow"),
            OperationError::StackOverflow => writeln!(f, "stack-overflow"),
            OperationError::DivisionByZero => writeln!(f, "division-by-zero"),            
            OperationError::InvalidWord => writeln!(f, "invalid-word"),            
        }
    }
}
