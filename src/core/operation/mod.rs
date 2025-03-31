use crate::core::error::OperationError;
use std::collections::HashMap;
pub mod arithmetic;
pub mod boolean;
pub mod stack_manipulation;

pub trait Operation {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError>;
}

pub fn get_all_operations() -> HashMap<String, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.extend(arithmetic::get_operations());
    /* ops.extend(boolean::get_operations()); */
    ops
}
