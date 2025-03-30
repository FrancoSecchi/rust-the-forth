use std::env;
use rust_the_forth::utils::{file_manager, cli_manager};
use rust_the_forth::core::forth_calculator::ForthCalculator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = cli_manager::validate_command_args(&args) {
        println!("Error: {}", e);
        return;
    }
    let content;
    let error = match file_manager::read_to_string(&args[1]) {
        Ok(content_str) => {
            content = content_str;
            false
        },
        Err(_) => {
            content = String::new();  
            true
        }
    };

    if error {
        println!("Error al abrir el archivo.");
        return;
    }
    
    let forth_calculator = ForthCalculator::new(content, cli_manager::get_size_of_stack(&args));
    println!("{:#?}", forth_calculator);

    forth_calculator.run();
    
}