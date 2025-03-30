use std::env;
use rust_the_forth::utils::{file_manager, validation};

fn main() {
    let args: Vec<String> = env::args().collect();

    let vec_size: i16 = 128;


    if let Err(e) = validation::validate_command_args(&args) {
        println!("Error: {}", e);
        return;
    }

    /* let path = &args[1];
    open_file(path); */
}