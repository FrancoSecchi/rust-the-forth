use crate::core::error::OperationError;
use std::collections::HashMap;
/// The `arithmetic` module provides basic arithmetic operations
/// such as addition, subtraction, multiplication, and division.
/// Each operation implements the `Operation` trait and modifies
/// the stack accordingly.
pub mod arithmetic;

/// The `boolean` module defines logical operations including equality (`=`),
/// greater than (`>`), less than (`<`), logical `AND`, `OR`, and `NOT`.
/// Each operation adheres to the `Operation` trait and operates on values
/// from the stack to return boolean results.
pub mod boolean;

/// The `output` module includes operations related to standard output
/// such as printing values or characters from the stack.
/// It enables user-visible output through the `.` (dot), `cr`, `."`, and `emit` operations.
pub mod output;

/// The `stack_manipulation` module provides fundamental operations
/// to manipulate the state of the stack.
/// This includes `dup`, `drop`, `swap`, `rot`, and `over`,
/// all of which implement the `Operation` trait.
pub mod stack_manipulation;

/// Defines a trait for stack-based operations.
///
/// Implementors of this trait must define the `apply` method, which takes
/// a mutable reference to a stack of `i16` values and performs an operation on it.
///
/// # Errors
/// Returns an `OperationError` if the operation fails.
pub trait Operation {
    /// Applies the operation to the given stack.
    ///
    /// # Arguments
    /// * `stack` - A mutable reference to a vector of `i16` values representing the stack.
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError>;
}

/// Defines a trait for operations that produce output.
///
/// These operations not only modify the stack but also generate output in
/// a string buffer.
///
/// # Errors
/// Returns an `OperationError` if the operation fails.
pub trait OperationOutput {
    /// Applies the operation to the given stack and modifies the output string.
    ///
    /// # Arguments
    /// * `stack` - A mutable reference to a vector of `i16` values representing the stack.
    /// * `string_output` - A mutable reference to a `String` where output will be stored.
    /// * `text_to_print` - A string slice representing additional text to print.
    fn apply(
        &self,
        stack: &mut Vec<i16>,
        string_output: &mut String,
        text_to_print: &str,
    ) -> Result<(), OperationError>;
}

/// Represents the different types of operations supported by the Forth interpreter.
/// Each variant corresponds to a recognized operation token in the source code.
/// This enum is used as a key in the operations dispatch table.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Eq,
    Greater,
    Less,
    Not,
    Or,
    Drop,
    Dup,
    Over,
    Rot,
    Swap,
    Dot,
    Cr,
    PrintText,
    Emit,
}

impl OperationType {
    pub fn from_token(token: &str) -> Option<Self> {
        if token.starts_with(".\"") && token.chars().nth(2) == Some(' ') {
            return Some(OperationType::PrintText);
        }
        let token_lower = token.to_lowercase();
        match token_lower.as_str() {
            //Arithmetic
            "+" => Some(OperationType::Add),
            "-" => Some(OperationType::Sub),
            "*" => Some(OperationType::Mul),
            "/" => Some(OperationType::Div),
            //Boolean
            "=" => Some(OperationType::Eq),
            "<" => Some(OperationType::Less),
            ">" => Some(OperationType::Greater),
            "and" => Some(OperationType::And),
            "or" => Some(OperationType::Or),
            "not" => Some(OperationType::Not),
            //Stack manipulation
            "drop" => Some(OperationType::Drop),
            "over" => Some(OperationType::Over),
            "rot" => Some(OperationType::Rot),
            "swap" => Some(OperationType::Swap),
            "dup" => Some(OperationType::Dup),
            "." => Some(OperationType::Dot),
            "cr" => Some(OperationType::Cr),
            "emit" => Some(OperationType::Emit),
            _ => None,
        }
    }
}

/// Retrieves all standard operations and returns them in a `HashMap`.
///
/// This function aggregates different types of operations (e.g., arithmetic, boolean)
/// and returns them as a collection where each `OperationType` is mapped
/// to a boxed `Operation` trait object.
///
/// # Examples
/// ```text
/// let operations = get_all_standar_operations();
/// ```
pub fn get_all_standar_operations() -> HashMap<OperationType, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.extend(arithmetic::get_operations());
    ops.extend(boolean::get_operations());
    ops.extend(stack_manipulation::get_operations());
    ops
}

/// Retrieves all output-related operations and returns them in a `HashMap`.
///
/// This function collects operations that produce output (e.g., printing),
/// mapping each `OperationType` to a boxed `OperationOutput` trait object.
///
/// # Examples
/// ```text
/// let output_operations = get_output_operations();
/// ```
pub fn get_output_operations() -> HashMap<OperationType, Box<dyn OperationOutput>> {
    let mut ops = HashMap::new();
    ops.extend(output::get_operations());
    ops
}
