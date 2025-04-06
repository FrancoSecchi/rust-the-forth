/// Handles command-line argument parsing and validation.
///
/// This module is responsible for interpreting the arguments passed to the program,
/// validating the input format, and extracting configuration parameters such as stack size
/// and input file path.
///
/// It provides error handling through the `CommandArgsError` enum, ensuring that users
/// receive informative messages when arguments are incorrect or missing.
pub mod cli_manager;

/// Provides file input/output operations for the Forth interpreter.
///
/// This module manages reading Forth code from a file and saving the final state of the
/// stack to an output file. It ensures that file operations are performed safely and consistently,
/// returning appropriate errors when issues occur.
///
/// It is used primarily during program initialization and finalization to load input
/// and persist results.
pub mod file_manager;
