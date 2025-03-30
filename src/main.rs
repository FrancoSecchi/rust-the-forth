use std::fs::File;
use std::env;
use std::io::Read;

/// Tamaño en bytes de un tipo i16 (2 bytes)
const I16_SIZE: i16 = 2;

/// Estructura para representar la respuesta de validación de comandos del `cargo run` 
/// 
/// # Campos
/// * `is_valid` - Indica si la validación fue exitosa
/// * `message` - Mensaje descriptivo del resultado de la validación
struct CommandValidationResponse {
    is_valid: bool,
    message: String,    
}

/// Convierte un tamaño en bytes a cantidad de elementos i16
///
/// # Argumentos
/// * `size` - Tamaño en bytes a convertir
///
/// # Retorno
/// Cantidad de elementos i16 que caben en ese tamaño
///
/// # Ejemplo
/// ```
/// let elementos = convert_bytes_to_elements_amount(10); // Retorna 5
/// ```
fn convert_bytes_to_elements_amount( size: i16) -> i16 {
    size / I16_SIZE
}

//
fn open_file(path: &String) {
    println!("El path es: {path}");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("No se puede abrir el archivo {}", why)
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("No se puede abrir el archivo {}", why),
        Ok(_) => println!("{}", s)
    }

}

/// Valida el argumento de tamaño de stack (formato "stack-size=N")
///
/// # Argumentos
/// * `stack_size_arg` - Cadena con el argumento a validar
///
/// # Retorno
/// `CommandValidationResponse` con el resultado de la validación
///
/// # Ejemplo
/// ```
/// let validacion = validate_stack_size_arg(&String::from("stack-size=10"));
/// ```
fn validate_stack_size_arg(stack_size_arg: &String) -> CommandValidationResponse {
    let string_split: Vec<&str> = stack_size_arg.split('=').collect();

    if string_split.len() >= 2 {
        match string_split[1].parse::<i16>() {
            Ok(size) => {
                if convert_bytes_to_elements_amount(size) <= 0 {
                    CommandValidationResponse {
                        is_valid: false,
                        message: String::from("El tamaño especificado no puede ser nulo o menor a cero"),
                    }
                } else {                    
                    CommandValidationResponse {
                        is_valid: true,
                        message: String::new(),
                    }
                }
            }
            Err(_) => CommandValidationResponse {
                is_valid: false,
                message: String::from("Error al parsear el numero de stack-size"),
            },
        }
    } else {        
        CommandValidationResponse {
            is_valid: false,
            message: String::from("Formato inválido: falta el '='"),
        }
    }
}

/// Valida los argumentos pasados al programa
///
/// # Argumentos
/// * `args` - Vector con los argumentos del programa
///
/// # Retorno
/// `CommandValidationResponse` con el resultado de la validación
///
/// # Ejemplo
/// ```
/// let args: Vec<String> = env::args().collect();
/// let validacion = validate_command_args(&args);
/// ```
fn validate_command_args(args: &Vec<String>) -> CommandValidationResponse {
    if args.len() <= 1  {        
        return CommandValidationResponse {
            is_valid: false,
            message: String::from("No se ha especificado el archivo"),
        };
    }

    let arg_file = &args[1]; 
    if !arg_file.contains(".") {
        return CommandValidationResponse {
            is_valid: false,
            message: String::from("El primer parametro debe ser el archivo a leer"),
        };
    }
    let stack_size_arg_valdation = validate_stack_size_arg(&args[2]);

    return CommandValidationResponse {
        is_valid: if stack_size_arg_valdation.is_valid {true} else {false},
        message: if stack_size_arg_valdation.is_valid {String::from("")} else {stack_size_arg_valdation.message},
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let vec_size: i16 = 128;

    let command_validation_response: CommandValidationResponse = validate_command_args(&args);
    if !command_validation_response.is_valid {
        println!("Error: {}", command_validation_response.message)
    }

    /* let path = &args[1];
    open_file(path); */
}