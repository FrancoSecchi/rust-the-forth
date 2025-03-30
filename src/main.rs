use std::env;
use rust_the_forth::utils::cli_manager;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = cli_manager::validate_command_args(&args) {
        println!("Error: {}", e);
        return;
    }

    let vec_size: i16 = cli_manager::get_stack_size_arg(&args);

    println!("El tamaño del stack es: {}", vec_size)

    /* let path = &args[1];
    open_file(path); */
}