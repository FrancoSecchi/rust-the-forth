#[derive(Debug)]
pub struct ForthCalculator {
    stack: Vec<i16>,
    output: String,
    content: String
}

impl ForthCalculator {
    pub fn new (content: String, stack_size: i16)-> Self {
        ForthCalculator { 
            stack: Vec::with_capacity(stack_size.max(0) as usize), 
            output: String::new(), 
            content: content}
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
                            if let (Some(a_val), Some(b_val)) = (self.stack.pop(), self.stack.pop()) {
                                self.stack.push(b_val + a_val); 
                            }                     
                        }
                    }
                }
            }
        }
    }
} 

