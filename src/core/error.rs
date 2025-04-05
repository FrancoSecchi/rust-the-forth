use std::fmt;

/// Represents possible errors that can occur during operation execution.
#[derive(Debug)]
pub enum OperationError {
    /// The stack does not contain enough elements to perform the operation.
    StackUnderflow,
    /// The stack has reached its maximum capacity.
    StackOverflow,
    /// Attempted to divide by zero.
    DivisionByZero,
    /// An invalid word was encountered.
    InvalidWord,
    /// Failed to write the stack to the file.
    FailWritingFile,
    /// Failed to red the file.
    FailReadFile,
    /// The output string is null or empty.
    StringNull,
    /// The character provided for the `emit` operation is invalid.
    InvalidCharacter,
    /// The provided word was not recognized as a valid operation.
    WordNotFound,
}

pub enum CommandArgsError {
    InvalidStackSize,
    FailParseStackSize,
    InvalidFormat,
    FileNotSpecified,
    InvalidFileFormat,
}

impl fmt::Display for CommandArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandArgsError::InvalidStackSize => {
                writeln!(f, "Stack size cannot be zero or less than or equal to one")
            }
            CommandArgsError::FailParseStackSize => {
                writeln!(f, "There was an error parsing the number for stack-size")
            }
            CommandArgsError::InvalidFileFormat => {
                writeln!(f, "The first parameter should be the file to read")
            }
            CommandArgsError::InvalidFormat => writeln!(f, "Invalid format: the '=' is missing "),
            CommandArgsError::FileNotSpecified => {
                writeln!(f, "The file to be read has not been specified ")
            }
        }
    }
}

impl fmt::Display for OperationError {
    /// Formats the error message for display.
    ///
    /// Each variant of `OperationError` is converted into a user-friendly message.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::StackUnderflow => writeln!(f, "stack-underflow"),
            OperationError::StackOverflow => writeln!(f, "stack-overflow"),
            OperationError::DivisionByZero => writeln!(f, "division-by-zero"),
            OperationError::InvalidWord => writeln!(f, "invalid-word"),
            OperationError::FailWritingFile => {
                writeln!(f, "We have a problem with writing the stack in stack.fht")
            }
            OperationError::FailReadFile => {
                writeln!(f, "We have a problem with reading the file")
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
