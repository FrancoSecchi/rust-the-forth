pub mod dot;

use std::collections::HashMap;

pub use dot::Dot;

use super::{OperationOutput, OperationType};


pub fn get_operations() -> HashMap<OperationType, Box<dyn OperationOutput>> {
    let mut ops = HashMap::new();
    ops.insert(OperationType::Dot, Box::new(Dot) as Box<dyn OperationOutput>);    
    ops
}