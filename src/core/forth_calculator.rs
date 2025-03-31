use crate::core::error::OperationError;
use crate::core::operation::get_all_operations;
use crate::core::operation::Operation;
use crate::utils::file_manager;
use std::collections::HashMap;

use super::operation::OperationType;

pub struct ForthCalculator {
    max_stack_size: i16,
    stack: Vec<i16>,
    output: String,
    operations: HashMap<OperationType, Box<dyn Operation>>,
}

impl ForthCalculator {
    pub fn new(stack_size: i16) -> Self {
        ForthCalculator {
            max_stack_size: stack_size,
            stack: Vec::new(),
            output: String::new(),            
            operations: get_all_operations(),
        }
    }

    pub fn get_stack(&self) -> &Vec<i16> {
        &self.stack
    }

    pub fn run(&mut self, content: String) {
        for token in content.split_whitespace() {
            if let Ok(number) = token.parse::<i16>() {
                if let Err(e) = self.process_number(number) {
                    println!("{}", OperationError::StackOverflow);
                    break;
                }
            } else {
                if token.len() == 1 {
                    if let Some(operation_type) = OperationType::from_token(token) {
                        if let Some(op) = self.operations.get(&operation_type) {
                            match op.apply(&mut self.stack) {
                                Ok(_) => {}
                                Err(error) => {
                                    println!("{error}");
                                }
                            }
                        }
                    }                    
                }
            }        
        }
        if let Err(e) = file_manager::save_stack(&self.stack) {
            println!("{}", OperationError::FailWritingFile)
        };        
    }

    fn process_number(&mut self, number: i16) -> Result<(), OperationError> {
        if self.stack.len() == self.max_stack_size as usize {
            return Err(OperationError::StackOverflow);        
        }   

        self.stack.push(number);
        Ok(())
    }
}
