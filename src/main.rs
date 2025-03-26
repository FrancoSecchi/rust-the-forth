fn main() {
    let mut number_stack: Vec<i16> = vec![];
    let string: &str = "1 2 3";

    for number_str in string.split_whitespace() {        
        match number_str.parse::<i16>() {
            Ok(number) => number_stack.push(number),
            Err(_) => println!("No se pudo convertir: {}", number_str),
        }
    }
    
    // Imprimir los números
    for number in number_stack {
        println!("{number}");
    }
}