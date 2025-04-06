use crate::core::error::CommandArgsError;

/// Size in bytes of an i16 type (2 bytes)
const I16_SIZE: i16 = 2;

/// Converts a byte size to the number of i16 elements
///
/// # Arguments
/// * `size` - Byte size to convert
///
/// # Example
/// ```text
/// use rust_the_forth::utils::convert_bytes_to_elements_amount;
/// let elements = convert_bytes_to_elements_amount(10); // Returns 5
/// ```
fn convert_bytes_to_elements_amount(size: i16) -> i16 {
    size / I16_SIZE
}

/// Validates the stack size argument (format: "stack-size=N")
///
/// # Arguments
/// * `stack_size_arg` - String containing the argument to validate
///
/// # Example
/// ```text
/// use rust_the_forth::utils::validate_stack_size_arg;
/// let validation = validate_stack_size_arg(&String::from("stack-size=10"));
/// ```
fn validate_stack_size_arg(stack_size_arg: &str) -> Result<(), CommandArgsError> {
    let string_split: Vec<&str> = stack_size_arg.split('=').collect();

    if string_split.len() >= 2 {
        match string_split[1].parse::<i16>() {
            Ok(size) => {
                if convert_bytes_to_elements_amount(size) <= 0 {
                    return Err(CommandArgsError::InvalidStackSize);
                } else {
                    return Ok(());
                }
            }
            Err(_) => {
                return Err(CommandArgsError::FailParseStackSize);
            }
        }
    }

    Err(CommandArgsError::InvalidFormat)
}

/// Validates the arguments passed to the program
///
/// # Arguments
/// * `args` - Vector of program arguments
///
/// # Example
/// ```text
/// use std::env;
/// use rust_the_forth::utils::validate_command_args;
/// let args: Vec<String> = env::args().collect();
/// let validation = validate_command_args(&args);
/// ```
pub fn validate_command_args(args: &[String]) -> Result<(), CommandArgsError> {
    if args.len() <= 1 {
        return Err(CommandArgsError::FileNotSpecified);
    }

    let arg_file = &args[1];
    if !arg_file.contains(".") {
        return Err(CommandArgsError::InvalidFileFormat);
    }

    if args.len() <= 2 {
        return Ok(());
    }

    validate_stack_size_arg(&args[2])
}

/// Returns the number of elements allowed by the `stack-size` argument
///
/// # Arguments
/// * `args` - Vector of program arguments
///
/// # Example
/// ```text
/// use std::env;
/// use rust_the_forth::utils::get_size_of_stack;
/// let args: Vec<String> = env::args().collect();
/// let vec_len = get_size_of_stack(&args);
/// ```
pub fn get_size_of_stack(args: &[String]) -> i16 {
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

/// Tests unitarios
#[cfg(test)]
mod cli_manager_tests {
    use super::*;
    #[test]
    fn test_convert_bytes_to_elements() {
        assert_eq!(convert_bytes_to_elements_amount(10), 5);
        assert_eq!(convert_bytes_to_elements_amount(0), 0);
    }

    #[test]
    fn test_validate_stack_size_arg() {
        assert!(validate_stack_size_arg("stack-size=10").is_ok());

        assert!(validate_stack_size_arg("stack-size=0").is_err());
        assert!(validate_stack_size_arg("stack-size=abc").is_err());
        assert!(validate_stack_size_arg("formato-incorrecto").is_err());
    }

    #[test]
    fn test_get_size_of_stack() {
        let args = vec![
            "program_name".to_string(),
            "file.fth".to_string(),
            "stack-size=20".to_string(),
        ];

        assert_eq!(get_size_of_stack(&args), 10);

        let args_min = vec!["program_name".to_string()];
        assert_eq!(get_size_of_stack(&args_min), 64);
    }

    #[test]
    fn test_validate_command_args() {
        let valid_args = vec![
            "program".to_string(),
            "script.fth".to_string(),
            "stack-size=128".to_string(),
        ];

        let invalid_file_args = vec!["program".to_string(), "nofile".to_string()];

        assert!(validate_command_args(&valid_args).is_ok());
        assert!(validate_command_args(&invalid_file_args).is_err());
    }
}
