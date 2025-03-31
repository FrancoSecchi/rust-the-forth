use std::collections::HashMap;
pub mod cr;
pub mod dot;

pub use cr::Cr;
pub use dot::Dot;

use super::{OperationOutput, OperationType};

pub fn get_operations() -> HashMap<OperationType, Box<dyn OperationOutput>> {
    let mut ops = HashMap::new();
    ops.insert(
        OperationType::Dot,
        Box::new(Dot) as Box<dyn OperationOutput>,
    );
    ops.insert(OperationType::Cr, Box::new(Cr) as Box<dyn OperationOutput>);
    ops
}
