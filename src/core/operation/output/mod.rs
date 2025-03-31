use std::collections::HashMap;
pub mod cr;
pub mod dot;
pub mod emit;
pub mod text;

pub use cr::Cr;
pub use dot::Dot;
pub use emit::Emit;
pub use text::PrintText;

use super::{OperationOutput, OperationType};

pub fn get_operations() -> HashMap<OperationType, Box<dyn OperationOutput>> {
    let mut ops = HashMap::new();
    ops.insert(
        OperationType::Dot,
        Box::new(Dot) as Box<dyn OperationOutput>,
    );
    ops.insert(OperationType::Cr, Box::new(Cr) as Box<dyn OperationOutput>);
    ops.insert(
        OperationType::Emit,
        Box::new(Emit) as Box<dyn OperationOutput>,
    );
    ops.insert(
        OperationType::PrintText,
        Box::new(PrintText) as Box<dyn OperationOutput>,
    );
    ops
}
