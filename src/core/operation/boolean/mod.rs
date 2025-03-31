use crate::core::operation::Operation;
use std::collections::HashMap;

pub mod and;
pub mod or;
pub mod eq;
pub mod not;
pub mod greater;
pub mod less;

pub use and::And;
pub use or::Or;
pub use eq::Eq;
pub use greater::Greater;
pub use less::Less;
pub use not::Not;

pub fn get_operations() -> HashMap<String, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.insert("=".to_string(), Box::new(Eq) as Box<dyn Operation>);
    ops.insert("<".to_string(), Box::new(Less) as Box<dyn Operation>);
    ops.insert(">".to_string(), Box::new(Greater) as Box<dyn Operation>);
    ops.insert("and".to_string(), Box::new(And) as Box<dyn Operation>);
    ops.insert("or".to_string(), Box::new(Or) as Box<dyn Operation>);
    ops.insert("not".to_string(), Box::new(Not) as Box<dyn Operation>);
    ops
}
