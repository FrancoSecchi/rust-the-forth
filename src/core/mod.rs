/// This module defines custom error types used throughout the application.
///
/// It includes error enums and trait implementations for displaying
/// meaningful messages to the user and aiding debugging.
///
/// These errors are used for command-line argument validation,
/// runtime stack operations, file handling, and general interpreter failures.
pub mod error;

/// Implements the core logic of the Forth interpreter.
///
/// This module provides the `ForthCalculator` struct, which is responsible for parsing
/// input commands, executing supported operations, and maintaining the internal stack state.
/// It acts as the main entry point for running Forth-like code within the application.
pub mod forth_calculator;

/// Defines the abstraction and concrete implementations for stack operations.
///
/// This module introduces the `Operation` trait and groups different categories of operations,
/// including arithmetic, boolean, stack manipulation, and output-related behaviors.
///
/// It also provides a utility function `get_operations()` to retrieve all supported operations
/// in a structured map for execution.
pub mod operation;

/// Module containing type aliases used for control structure handling.
/// 
/// This file defines type aliases that are used to manage the results of operations
/// related to conditional blocks, such as `if` and `else`, in the processed code. These type aliases
/// simplify the handling of results and errors in a clear and consistent manner.
pub mod types;

