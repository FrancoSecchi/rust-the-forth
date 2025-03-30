use crate::core::operation::{get_operations, Operation};
use std::collections::HashMap;

pub struct ForthCalculator {
    stack: Vec<i16>,
    output: String,
    content: String,
    operations: HashMap<String, Box<dyn Operation>>,
}

impl ForthCalculator {
    pub fn new(content: String, stack_size: i16) -> Self {
        ForthCalculator {
            stack: Vec::with_capacity(stack_size.max(0) as usize),
            output: String::new(),
            content,
            operations: get_operations(),
        }
    }

    pub fn run(mut self) {
        for token in self.content.split_whitespace() {
            match token.parse::<i16>() {
                Ok(number) => {
                    self.stack.push(number);
                }
                Err(_) => {
                    if token.len() == 1 {
                        if let Some(op) = self.operations.get(token) {
                            match op.apply(&mut self.stack) {
                                Ok(_) => {}
                                Err(error) => {
                                    println!("{error}");
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(
            "{:#?} las keys de as operaciones son {:#?}",
            self.stack,
            self.operations.keys()
        );
    }
}
