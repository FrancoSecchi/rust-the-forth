/// Contains the main components of the Forth interpreter.
///
/// This module includes the core logic and abstractions required to execute
/// Forth code. It defines the calculator engine, supported operations,
/// and error handling mechanisms.
pub mod core;

/// Provides utility functions and helpers used across the application.
///
/// This module contains reusable components that support file I/O,
/// argument parsing, and any other non-core logic needed to run the interpreter.
pub mod utils;
