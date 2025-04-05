use rust_the_forth::core::error::OperationError;
use rust_the_forth::core::forth_calculator::ForthCalculator;
use rust_the_forth::utils::{cli_manager, file_manager};
use std::env;
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    run_app(std::io::stdout())
}

pub fn run_app<W: Write>(mut out: W) -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = cli_manager::validate_command_args(&args) {
        write!(out, "Error: {}", e)?;
        return Ok(());
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
        if file_manager::save_stack(&Vec::new()).is_err() {
            write!(out, "{}", OperationError::FailWritingFile)?;
        }
        write!(out, "{}", OperationError::FailWritingFile)?;
        return Ok(());
    }

    let mut forth_calculator = ForthCalculator::new(cli_manager::get_size_of_stack(&args));
    forth_calculator.run(content);
    write!(out, "{}", forth_calculator.get_output())?;
    Ok(())
}
