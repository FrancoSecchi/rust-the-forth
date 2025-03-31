use crate::core::error::OperationError;
use crate::core::operation::get_all_operations;
use crate::core::operation::Operation;
use crate::utils::file_manager;
use std::collections::HashMap;

use super::operation::OperationType;

pub struct ForthCalculator {
    stack: Vec<i16>,
    output: String,
    content: String,
    operations: HashMap<OperationType, Box<dyn Operation>>,
}

impl ForthCalculator {
    pub fn new(content: String, stack_size: i16) -> Self {
        ForthCalculator {
            stack: Vec::with_capacity(stack_size.max(0) as usize),
            output: String::new(),
            content,
            operations: get_all_operations(),
        }
    }

    pub fn get_stack(&self) -> &Vec<i16> {
        &self.stack
    }

    pub fn run(&mut self) {
        for token in self.content.split_whitespace() {
            match token.parse::<i16>() {
                Ok(number) => {
                    self.stack.push(number);
                }
                Err(_) => {
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
        }
        if let Err(e) = file_manager::save_stack(&self.stack) {
            println!("{}", OperationError::FailWritingFile)
        };        
    }
}
