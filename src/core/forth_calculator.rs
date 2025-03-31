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
        for token in content.split_whitespace() {
            if let Ok(number) = token.parse::<i16>() {
                if let Err(e) = self.push_number(number) {
                    println!("{e}");
                    break;
                }
            } else {
                if let Err(e) = self.execute_operation(token, &mut output) {
                    println!("{}", e);
                }
            }
        }
        if let Err(e) = file_manager::save_stack(&self.stack) {
            println!("{e}");
        };

        if !output.is_empty() {
            println!("{output}");
        }
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
        if token.len() == 1 {
            if let Some(operation_type) = OperationType::from_token(token) {
                if let Some(operation) = self.operations.get(&operation_type) {
                    operation.apply(&mut self.stack)?;
                } else if let Some(operation) = self.output_operations.get(&operation_type) {
                    operation.apply(&mut self.stack, output)?;
                }
            } else {
               return Err(OperationError::WordNotFound);
            }
        }
        Ok(())
    }
}
