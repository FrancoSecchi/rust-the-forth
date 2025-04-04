use rust_the_forth::core::error::OperationError;
use rust_the_forth::core::forth_calculator::ForthCalculator;
use rust_the_forth::utils::{cli_manager, file_manager};
use std::env;

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
        }
        Err(_) => {
            content = String::new();
            true
        }
    };

    if error {
        if let Err(_e) = file_manager::save_stack(&Vec::new()) {
            println!("{}", OperationError::FailWritingFile);
        }
        println!("{}", OperationError::FailWritingFile);
        return;
    }

    let mut forth_calculator = ForthCalculator::new(cli_manager::get_size_of_stack(&args));
    forth_calculator.run(content);
}
