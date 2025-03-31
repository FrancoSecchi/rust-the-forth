use crate::core::operation::Operation;
use std::collections::HashMap;

pub mod and;
pub mod eq;
pub mod greater;
pub mod less;
pub mod not;
pub mod or;

pub use and::And;
pub use eq::Eq;
pub use greater::Greater;
pub use less::Less;
pub use not::Not;
pub use or::Or;

use super::OperationType;

pub fn get_operations() -> HashMap<OperationType, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.insert(OperationType::Eq, Box::new(Eq) as Box<dyn Operation>);
    ops.insert(OperationType::Less, Box::new(Less) as Box<dyn Operation>);
    ops.insert(
        OperationType::Greater,
        Box::new(Greater) as Box<dyn Operation>,
    );
    ops.insert(OperationType::And, Box::new(And) as Box<dyn Operation>);
    ops.insert(OperationType::Or, Box::new(Or) as Box<dyn Operation>);
    ops.insert(OperationType::Not, Box::new(Not) as Box<dyn Operation>);
    ops
}
