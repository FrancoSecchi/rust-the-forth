use crate::core::error::OperationError;

pub trait Operation  {
    fn apply(&self,stack: &mut Vec<i16>) -> Result<(), OperationError>;
}

pub struct Add;

impl Operation for Add {
    fn apply (&self, stack: &mut Vec<i16>) -> Result<(), OperationError> {
        let a = stack.pop().ok_or(OperationError::StackUnderflow)?;
        let b = stack.pop().ok_or(OperationError::StackUnderflow)?;    
        stack.push(a + b);
        Ok(())
    }
}


