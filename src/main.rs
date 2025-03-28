fn main() {
    let mut number_stack: Vec<i16> = vec![];
    let string: &str = "1 2 +";

    for token in string.split_whitespace() {        
        match token.parse::<i16>() {
            Ok(token) => number_stack.push(token),
            Err(_) => {
                if token.len() == 1 {
                    if token == "+" {
                        if let (Some(a_val), Some(b_val)) = (number_stack.pop(), number_stack.pop()) {
                            number_stack.push(b_val + a_val); 
                        }                     
                    }
                }
            },
        }
    }
    
    // Imprimir los números
    for number in number_stack {
        println!("{number}");
    }
}