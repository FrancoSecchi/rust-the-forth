use crate::core::operation::Operation;
use std::collections::HashMap;

pub mod add;
pub mod div;
pub mod mul;
pub mod sub;

use super::OperationType;
pub use add::Add;
pub use div::Div;
pub use mul::Mul;
pub use sub::Sub;

pub fn get_operations() -> HashMap<OperationType, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.insert(OperationType::Add, Box::new(Add) as Box<dyn Operation>);
    ops.insert(OperationType::Sub, Box::new(Sub) as Box<dyn Operation>);
    ops.insert(OperationType::Mul, Box::new(Mul) as Box<dyn Operation>);
    ops.insert(OperationType::Div, Box::new(Div) as Box<dyn Operation>);
    ops
}
