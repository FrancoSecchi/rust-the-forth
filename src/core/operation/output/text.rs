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

#[test]
fn test_print_text_basic() {
    let mut stack: Vec<i16> = vec![];
    let mut output = String::new();
    let tokens = vec![
        "1".to_string(),
        ".\" ".to_string(),
        "holaa\"".to_string(),
    ];
    
    PrintText.apply(&mut stack, &mut output, &tokens).unwrap();
    
    assert_eq!(stack, vec![]);  
    assert_eq!(output, "holaa"); 
}

#[test]
fn test_print_text_multiple_words() {
    let mut stack: Vec<i16> = vec![2, 3];
    let mut output = String::new();
    let tokens = vec![
        ".\" ".to_string(),
        "hello".to_string(),
        "world\"".to_string(),
        "other".to_string(),  
    ];
    
    PrintText.apply(&mut stack, &mut output, &tokens).unwrap();
    
    assert_eq!(stack, vec![2, 3]);  
    assert_eq!(output, "hello world");  
}

#[test]
fn test_print_text_empty() {
    let mut stack: Vec<i16> = vec![];
    let mut output = String::new();
    let tokens = vec![
        ".\" ".to_string(),
        "\"".to_string(),  
    ];
    
    PrintText.apply(&mut stack, &mut output, &tokens).unwrap();
    
    assert_eq!(stack, vec![]);
    assert_eq!(output, "");  
}

#[test]
fn test_print_text_with_numbers_before() {
    let mut stack: Vec<i16> = vec![42];
    let mut output = String::new();
    let tokens = vec![
        "123".to_string(),
        ".\" ".to_string(),
        "test\"".to_string(),
    ];
    
    PrintText.apply(&mut stack, &mut output, &tokens).unwrap();
    
    assert_eq!(stack, vec![42]); 
    assert_eq!(output, "test");  
}