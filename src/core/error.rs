use std::fmt;

#[derive(Debug)]
pub enum OperationError {
    StackUnderflow,
    StackOverflow,
    DivisionByZero,
    InvalidWord,
    FailWritingFile,
    StringNull,
    InvalidCharacter,
    WordNotFound,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::StackUnderflow => writeln!(f, "stack-underflow"),
            OperationError::StackOverflow => writeln!(f, "stack-overflow"),
            OperationError::DivisionByZero => writeln!(f, "division-by-zero"),
            OperationError::InvalidWord => writeln!(f, "invalid-word"),
            OperationError::FailWritingFile => {
                writeln!(f, "We have a problem with writing the stack in stack.fht")
            }
            OperationError::StringNull => {
                writeln!(f, "The output string is Null")
            }
            OperationError::InvalidCharacter => {
                writeln!(f, "The current character to emit is not valid")
            }
            OperationError::WordNotFound => {
                writeln!(f, "?")
            }
        }
    }
}
