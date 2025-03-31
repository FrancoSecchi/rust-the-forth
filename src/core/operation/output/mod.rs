use std::collections::HashMap;
pub mod cr;
pub mod dot;
pub mod text;
pub mod emit;

pub use cr::Cr;
pub use dot::Dot;
pub use text::PrintText;
pub use emit::Emit;

use super::{OperationOutput, OperationType};

pub fn get_operations() -> HashMap<OperationType, Box<dyn OperationOutput>> {
    let mut ops = HashMap::new();
    ops.insert(
        OperationType::Dot,
        Box::new(Dot) as Box<dyn OperationOutput>,
    );
    ops.insert(OperationType::Cr, Box::new(Cr) as Box<dyn OperationOutput>);
    ops.insert(OperationType::Emit, Box::new(Emit) as Box<dyn OperationOutput>);
    ops
}
