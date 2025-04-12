use std::fmt;

/// Represents possible errors that can occur during operation execution.
#[derive(Debug, PartialEq)]
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

    InvalidWordFormat,

    InvalidIfFormat,
}

/// Represents possible errors that can occur during the validation of command-line arguments.
///
/// These errors are used to handle incorrect or malformed input passed to the CLI.
/// Each variant corresponds to a specific issue that may be encountered.
pub enum CommandArgsError {
    /// The specified stack size is zero or less than or equal to one, which is invalid.
    InvalidStackSize,

    /// Failed to parse the provided stack size value into a valid number.
    FailParseStackSize,

    /// The provided argument format is incorrect (e.g., missing `=` in `--key=value`).
    InvalidFormat,

    /// No input file was specified by the user.
    FileNotSpecified,

    /// The file argument is not in the expected position or format.
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
            OperationError::InvalidWordFormat => {
                writeln!(f, "? invalid word format")
            }
            OperationError::InvalidIfFormat => {
                writeln!(f, "if - invalid if format")
            }
        }
    }
}
