pub mod drop;
pub mod dup;
pub mod over;
pub mod rot;
pub mod swap;

use std::collections::HashMap;

pub use drop::Drop;
pub use dup::Dup;
pub use over::Over;
pub use rot::Rot;
pub use swap::Swap;

use super::{Operation, OperationType};


pub fn get_operations() -> HashMap<OperationType, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.insert(OperationType::Drop, Box::new(Drop) as Box<dyn Operation>);
    ops.insert(OperationType::Dup, Box::new(Dup) as Box<dyn Operation>);
    ops.insert(
        OperationType::Over,
        Box::new(Over) as Box<dyn Operation>,
    );
    ops.insert(OperationType::Rot, Box::new(Rot) as Box<dyn Operation>);
    ops.insert(OperationType::Swap, Box::new(Swap) as Box<dyn Operation>);
    ops
}