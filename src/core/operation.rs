use std::collections::HashMap;

use crate::core::error::OperationError;

pub trait Operation {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError>;
}

#[derive(Debug)]
pub struct Add;

impl Operation for Add {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let a = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let b = stack.pop().ok_or(OperationError::StackUnderflow)?;
        stack.push(a + b);
        Ok(())
    }
}

pub fn get_operations() -> HashMap<String, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.insert("+".to_string(), Box::new(Add) as Box<dyn Operation>);
    ops
}
