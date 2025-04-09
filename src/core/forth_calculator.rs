use super::operation::OperationOutput;
use super::operation::OperationType;
use crate::core::error::OperationError;
use crate::core::operation::Operation;
use crate::core::operation::{get_all_standar_operations, get_output_operations};
use crate::utils::file_manager;
use std::collections::HashMap;

/// A stack-based calculator implementing a subset of the Forth language.
/// This calculator supports arithmetic operations, boolean operations,
/// stack manipulation, and output operations.
pub struct ForthCalculator {
    /// Maximum allowed stack size.
    max_stack_size: i16,
    /// Stack that stores numeric values for operations.
    stack: Vec<i16>,
    /// Mapping of standard operations (e.g., arithmetic, boolean) to their implementations.
    operations: HashMap<OperationType, Box<dyn Operation>>,
    /// Mapping of output-related operations (e.g., printing, emitting characters).
    output_operations: HashMap<OperationType, Box<dyn OperationOutput>>,

    output: String,
}

impl ForthCalculator {
    /// Creates a new instance of `ForthCalculator` with a given stack size.
    ///
    /// # Arguments
    ///
    /// * `stack_size` - The maximum number of elements allowed in the stack.
    pub fn new(stack_size: i16) -> Self {
        ForthCalculator {
            max_stack_size: stack_size,
            stack: Vec::new(),
            operations: get_all_standar_operations(),
            output_operations: get_output_operations(),
            output: String::new(),
        }
    }

    pub fn get_output(&self) -> &String {
        &self.output
    }

    /// Returns a reference to the current stack.
    pub fn get_stack(&self) -> &Vec<i16> {
        &self.stack
    }

    /// Checks if a word definition in the token list is valid.
    /// 
    /// A valid word definition must:
    /// - Contain exactly one colon `:` and one semicolon `;`.
    /// - Have no other tokens that partially include `:` or `;`.
    ///     
    fn is_valid_word_definition(&mut self, tokens: &[String]) -> bool {
        let total_semicolons = tokens.iter().filter(|token| token == &";").count();
        let total_colons = tokens.iter().filter(|token| token == &":").count();

        let all_colons_valid = tokens.iter().filter(|t| t.contains(':')).all(|t| t == ":");
        let all_semicolons_valid = tokens.iter().filter(|t| t.contains(';')).all(|t| t == ";");

        total_colons == 1 && total_semicolons == 1 && all_colons_valid && all_semicolons_valid
    }

    /// Validates that the provided tokens are syntactically and semantically correct.
    ///
    /// This function performs two checks:
    /// 1. If there is a custom word definition (indicated by a colon `:`),
    ///    it must be valid as defined by `is_valid_word_definition`.
    /// 2. All tokens must either be:
    ///    - A valid integer
    ///    - A known predefined operation
    ///    - A user-defined word
    ///     
    fn are_valid_tokens(&mut self, tokens: &Vec<String>) -> Result<(), OperationError> {
        let total_colons = tokens.iter().filter(|token| token == &":").count();
        if total_colons != 0 && !self.is_valid_word_definition(tokens) {
            return Err(OperationError::WordNotFound);
        }
        for token in tokens {
            if let Err(_error) = token.parse::<i16>() {
                let operation_type =
                    OperationType::from_token(token).ok_or(OperationError::WordNotFound)?;

                if !self.operations.contains_key(&operation_type)
                    && !self.output_operations.contains_key(&operation_type)
                //&& !self.words.contains_key(&operation_type)
                {
                    return Err(OperationError::WordNotFound);
                }
            }
        }

        Ok(())
    }

    /// Processes a given file input string by tokenizing it and executing the corresponding operations.
    ///
    /// # Arguments
    ///
    /// * `content` - The input string containing operations and numbers.
    ///
    /// The function iterates over the tokens, executing either number insertion or operations.
    /// If an error occurs, it is logged and execution stops.
    pub fn run(&mut self, content: String) {
        let mut output: String = String::new();
        let input_tokens = file_manager::tokenize(&content);
        match self.are_valid_tokens(&input_tokens) {
            Ok(()) => {
                for token in input_tokens {
                    let result = match token.parse::<i16>() {
                        Ok(number) => self.push_number(number),
                        Err(_) => self.execute_operation(&token, &mut output),
                    };

                    if let Err(e) = result {
                        self.add_string_output_error(&mut output, e);
                        break;
                    }
                }
            }
            Err(e) => {
                self.add_string_output_error(&mut output, e);
            }
        }

        if let Err(_e) = file_manager::save_stack(&self.stack) {
            self.add_string_output_error(&mut output, OperationError::FailWritingFile);
        }

        self.output = output;
    }

    /// Appends an error message to the output string.
    ///
    /// # Arguments
    ///
    /// * `output` - The output string where the error message will be appended.
    /// * `error` - The error that occurred.
    fn add_string_output_error(&mut self, output: &mut String, error: OperationError) {
        output.push_str(&format!("{}", error));
    }

    /// Pushes a number onto the stack.
    ///
    /// # Arguments
    ///
    /// * `number` - The number to be pushed onto the stack.
    fn push_number(&mut self, number: i16) -> Result<(), OperationError> {
        if self.stack.len() == self.max_stack_size as usize {
            return Err(OperationError::StackOverflow);
        }

        self.stack.push(number);
        Ok(())
    }

    /// Executes an operation based on the provided token.  
    ///
    /// # Arguments
    ///
    /// * `token` - The string representation of the operation.
    /// * `output` - A mutable reference to the output string (used for operations that produce output).
    fn execute_operation(
        &mut self,
        token: &str,
        output: &mut String,
    ) -> Result<(), OperationError> {
        let operation_type =
            OperationType::from_token(token).ok_or(OperationError::WordNotFound)?;
        if let Some(operation) = self.operations.get(&operation_type) {
            return operation.apply(&mut self.stack);
        }

        if let Some(operation) = self.output_operations.get(&operation_type) {
            return operation.apply(&mut self.stack, output, token);
        }

        let operation = self
            .output_operations
            .get(&operation_type)
            .ok_or(OperationError::WordNotFound)?;
        operation.apply(&mut self.stack, output, token)
    }
}
