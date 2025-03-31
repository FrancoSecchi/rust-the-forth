pub mod arithmetic;
pub mod boolean;
pub mod stack_manipulation;
pub mod output;
use crate::core::error::OperationError;

pub trait Operation {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError>;
}
