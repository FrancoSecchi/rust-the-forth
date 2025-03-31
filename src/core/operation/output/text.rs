use crate::core::error::OperationError;
use crate::core::operation::OperationOutput;

pub struct PrintText;

impl OperationOutput for PrintText {
    fn apply(&self, _stack: &mut Vec<i16>, output: &mut String, tokens: &Vec<String>) -> Result<(), OperationError> {
        let mut in_string = false;
        let mut collected_text = String::new();
        
        for token in tokens {
            if token == ".\" " {
                in_string = true;
                continue;
            }
            
            if in_string {
                if token.ends_with('"') {
                    collected_text.push_str(&token[..token.len()-1]);
                    break;
                } else {
                    collected_text.push_str(token);
                    collected_text.push(' ');  
                }
            }
        }
        
        output.push_str(&collected_text);        
        Ok(())
    }
}

