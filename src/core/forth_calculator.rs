use crate::core::error::OperationError;
use crate::core::operation::Operation;
use crate::core::operation::{get_all_operations, get_output_operations};
use crate::utils::file_manager;
use std::collections::HashMap;

use super::operation::OperationOutput;
use super::operation::OperationType;

pub struct ForthCalculator {
    max_stack_size: i16,
    stack: Vec<i16>,
    operations: HashMap<OperationType, Box<dyn Operation>>,
    output_operations: HashMap<OperationType, Box<dyn OperationOutput>>,
}

impl ForthCalculator {
    pub fn new(stack_size: i16) -> Self {
        ForthCalculator {
            max_stack_size: stack_size,
            stack: Vec::new(),
            operations: get_all_operations(),
            output_operations: get_output_operations(),
        }
    }

    pub fn get_stack(&self) -> &Vec<i16> {
        &self.stack
    }

    pub fn run(&mut self, content: String) {
        let mut output = String::new();
        let input_tokens = file_manager::tokenize(&content);
        for token in input_tokens.iter() {
            if let Ok(number) = token.parse::<i16>() {
                if let Err(e) = self.push_number(number) {
                    self.add_string_output_error(&mut output, e);
                    break;
                }
            } else if let Err(e) = self.execute_operation(token, &mut output) {
                self.add_string_output_error(&mut output, e);
                break;
            }
        }
        if let Err(_e) = file_manager::save_stack(&self.stack) {
            self.add_string_output_error(&mut output, OperationError::FailWritingFile);
        };

        if !output.is_empty() {
            println!("{output}");
        }
    }

    fn add_string_output_error(&mut self, output: &mut String, error: OperationError) {
        output.push_str(&format!("{}", error));
    }

    fn push_number(&mut self, number: i16) -> Result<(), OperationError> {
        if self.stack.len() == self.max_stack_size as usize {
            return Err(OperationError::StackOverflow);
        }

        self.stack.push(number);
        Ok(())
    }

    fn execute_operation(
        &mut self,
        token: &str,
        output: &mut String,
    ) -> Result<(), OperationError> {
        let operation_type =
            OperationType::from_token(token).ok_or(OperationError::WordNotFound)?;

        if token.len() == 1 {
            if let Some(operation) = self.operations.get(&operation_type) {
                return operation.apply(&mut self.stack);
            }
        }

        let operation = self
            .output_operations
            .get(&operation_type)
            .ok_or(OperationError::WordNotFound)?;
        operation.apply(&mut self.stack, output, token)
    }
}
