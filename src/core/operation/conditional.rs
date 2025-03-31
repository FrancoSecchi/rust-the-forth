
use crate::core::error::OperationError;
use crate::core::operation::Operation;
use std::collections::HashMap;

pub struct Eq;

impl Operation for Eq {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let second_item = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let result: i16 = if item == second_item {-1} else {0};        
        stack.push(result);        
        Ok(())
    }
}