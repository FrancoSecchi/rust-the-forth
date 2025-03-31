use crate::core::operation::Operation;
use std::collections::HashMap;

pub mod add;
pub mod div;
pub mod mul;
pub mod sub;

pub use add::Add;
pub use div::Div;
pub use mul::Mul;
pub use sub::Sub;

pub fn get_operations() -> HashMap<String, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.insert("+".to_string(), Box::new(Add) as Box<dyn Operation>);
    ops.insert("-".to_string(), Box::new(Sub) as Box<dyn Operation>);
    ops.insert("*".to_string(), Box::new(Mul) as Box<dyn Operation>);
    ops.insert("/".to_string(), Box::new(Div) as Box<dyn Operation>);
    ops
}
