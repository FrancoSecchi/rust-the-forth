pub mod arithmetic;
pub mod conditional;
pub mod stack_manipulation;
use crate::core::error::OperationError;

pub trait Operation {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError>;
}
