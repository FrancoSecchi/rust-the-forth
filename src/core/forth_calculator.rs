use core::error;
use std::collections::HashMap;

use super::operation::{Add, Operation};

#[derive(Debug)]
pub struct ForthCalculator {
    stack: Vec<i16>,
    output: String,
    content: String,
    
}


impl ForthCalculator {
    pub fn new (content: String, stack_size: i16)-> Self {
        ForthCalculator { 
            stack: Vec::with_capacity(stack_size.max(0) as usize), 
            output: String::new(), 
            content: content
            
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
                        if token == "+" {                        
                            match Add.apply(&mut self.stack) {
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

        println!("{:#?}", self.stack);
    }
} 

