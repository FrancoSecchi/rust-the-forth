//! Conditional module for parsing and slicing `if-else-then` control flow structures.
//!
//! This module provides utility functions to work with conditional expressions
//! in a Forth-like language. It allows extracting branch indices and slices of tokens
//! representing the `if`, `else`, and `then` parts of a conditional expression.
pub mod conditional;
pub use conditional::ConditionalToken;
