use crate::core::error::OperationError;
use std::collections::HashMap;
pub mod arithmetic;
pub mod boolean;
pub mod output;
pub mod stack_manipulation;

pub trait Operation {
    fn apply(&self, stack: &mut Vec<i16>) -> Result<(), OperationError>;
}

pub trait OperationOutput {
    fn apply(&self, stack: &mut Vec<i16>, string_output: &mut String, tokens: &Vec<String>)
        -> Result<(), OperationError>;
}

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
    Emit
}

impl OperationType {
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
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
            ".\" " => Some(OperationType::PrintText),
            _ => None,
        }
    }
}

pub fn get_all_operations() -> HashMap<OperationType, Box<dyn Operation>> {
    let mut ops = HashMap::new();
    ops.extend(arithmetic::get_operations());
    ops.extend(boolean::get_operations());
    ops
}

pub fn get_output_operations() -> HashMap<OperationType, Box<dyn OperationOutput>> {
    let mut ops = HashMap::new();
    ops.extend(output::get_operations());
    ops
}
