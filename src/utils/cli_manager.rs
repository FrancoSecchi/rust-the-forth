
/// Tamaño en bytes de un tipo i16 (2 bytes)
const I16_SIZE: i16 = 2;

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

/// Valida el argumento de tamaño de stack (formato "stack-size=N")
///
/// # Argumentos
/// * `stack_size_arg` - Cadena con el argumento a validar
///
/// # Retorno
/// `Result<(), String>` con el resultado de la validación
///
/// # Ejemplo
/// ```
/// let validacion = validate_stack_size_arg(&String::from("stack-size=10"));
/// ```
fn validate_stack_size_arg(stack_size_arg: &str) -> Result<(), String> {
    let string_split: Vec<&str> = stack_size_arg.split('=').collect();

    if string_split.len() >= 2 {
        match string_split[1].parse::<i16>() {
            Ok(size) => {
                if convert_bytes_to_elements_amount(size) <= 0 {
                    return Err("El tamaño especificado no puede ser nulo o menor o igual a uno".into());                                    
                } else {                    
                    return Ok(());                    
                }
            }
            Err(_) => {
                return Err("Error al parsear el numero de stack-size".into());                                                    
            }
        }
    } 

    Err("Formato inválido: falta el '='".into())       
}

/// Valida los argumentos pasados al programa
///
/// # Argumentos
/// * `args` - Vector con los argumentos del programa
///
/// # Retorno
/// `Result<(), String>` con el resultado de la validación
///
/// # Ejemplo
/// ```
/// let args: Vec<String> = env::args().collect();
/// let validacion = validate_command_args(&args);
/// ```
pub fn validate_command_args(args: &[String]) -> Result<(), String> {
    if args.len() <= 1 {
        return Err("No se ha especificado el archivo".into());
    }

    let arg_file = &args[1]; 
    if !arg_file.contains(".") {
        return Err("El primer parametro debe ser el archivo a lee".into());        
    }
    
    if args.len() <= 2 {        
        return Ok(());
    }    
    
    validate_stack_size_arg(&args[2])
}


/// Devuelve la cantidad de elementos que permite el valor del argumento del stack-size
///
/// # Argumentos
/// * `args` - Vector con los argumentos del programa
///
/// # Retorno
/// `i16` el valor del stack-size
///
/// # Ejemplo
/// ```
/// let args: Vec<String> = env::args().collect();
/// let vec_len = get_stack_size_arg(&args);
/// ```
pub fn get_stack_size_arg(args : &[String]) -> i16 {
    let default_len = convert_bytes_to_elements_amount(128);
    if args.len() <= 2 {
        return default_len;
    }

    let string_split: Vec<&str> = args[2].split('=').collect();
    
    if string_split.len() >= 2 {
        match string_split[1].parse::<i16>() {
            Ok(size) => {
                return convert_bytes_to_elements_amount(size);                
            }
            Err(_) => {
                return default_len;
            }
        }
    } 
    default_len

}