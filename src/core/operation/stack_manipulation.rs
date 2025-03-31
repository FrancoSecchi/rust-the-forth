use crate::core::error::OperationError;
use crate::core::operation::Operation;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Dup;

impl Operation for Dup {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        if stack.is_empty() {
            return Err(OperationError::StackUnderflow);
        }
        let item = stack.pop().ok_or(OperationError::StackUnderflow)?;

        stack.push(item);
        stack.push(item);
        Ok(())
    }
}

