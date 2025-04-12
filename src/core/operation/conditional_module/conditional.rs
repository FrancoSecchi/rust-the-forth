use crate::core::{
    error::OperationError,
    types::{BranchResult, BranchSlices},
};

/// Provides static utilities for handling conditional logic in tokenized Forth-like programs.
///
/// `Conditional` is a stateless utility struct used to encapsulate logic related
/// to control flow structures such as `if`, `else`, and `then`. All methods are static
/// and are intended to be used directly without instantiating the struct.
///
/// Example usage:
/// ```text
/// let (then_index, if_branch, else_branch) =
///     Conditional::extract_branch(&tokens, start_index)?;
/// ```
pub struct Conditional;

impl Conditional {
    /// Returns the indices of the matching `then` and optional `else` tokens starting from an `if`.
    ///
    /// # Parameters
    /// - `tokens`: A slice of token strings representing the full program.
    /// - `start`: The index of the `if` token.
    ///
    /// # Returns
    /// - `Ok((then_index, Some(else_index)))` if both `else` and `then` are found.
    /// - `Ok((then_index, None))` if only `then` is found.
    /// - `Err(OperationError::InvalidWord)` if the branching structure is invalid.
    pub fn get_branch_indices(
        tokens: &[String],
        start: usize,
    ) -> Result<(usize, Option<usize>), OperationError> {
        let mut branch_nesting = 0;
        let mut else_index = None;
        let mut then_index = None;

        let mut j = start;
        while j < tokens.len() {
            match tokens[j].as_str() {
                "if" => branch_nesting += 1,
                "then" => {
                    if branch_nesting == 0 {
                        return Err(OperationError::InvalidWord);
                    }
                    branch_nesting -= 1;
                    if branch_nesting == 0 {
                        then_index = Some(j);
                        break;
                    }
                }
                "else" => {
                    if branch_nesting == 1 && else_index.is_none() {
                        else_index = Some(j);
                    }
                }
                _ => {}
            }
            j += 1;
        }

        match then_index {
            Some(ti) => Ok((ti, else_index)),
            None => Err(OperationError::InvalidWord),
        }
    }

    /// Extracts the branches of a conditional (`if`, optional `else`, and `then`) from a token slice.
    ///
    /// # Parameters
    /// - `tokens`: A slice of tokens representing the full program.
    /// - `start`: The index of the `if` token.
    ///
    /// # Returns
    /// - `Ok((then_index, if_branch, else_branch))` with slices of tokens.
    /// - `Err(OperationError)` if branch structure is invalid.
    pub fn extract_branch(tokens: &[String], start: usize) -> BranchResult<'_> {
        let (then_index, else_index) = Self::get_branch_indices(tokens, start)?;
        let (if_branch, else_branch) =
            Self::get_branch_slices(tokens, start, then_index, else_index)?;
        Ok((then_index, if_branch, else_branch))
    }

    /// Extracts slices of tokens representing the `if` and optional `else` branches.
    ///
    /// # Parameters
    /// - `tokens`: A slice of tokens.
    /// - `start`: Index of the `if` token.
    /// - `then_index`: Index of the `then` token.
    /// - `else_index`: Optional index of the `else` token.
    ///
    /// # Returns
    /// A `BranchSlices` result with:
    /// - A slice of the `if` branch.
    /// - An optional slice of the `else` branch.    
    pub fn get_branch_slices(
        tokens: &[String],
        start: usize,
        then_index: usize,
        else_index: Option<usize>,
    ) -> BranchSlices<'_> {
        let if_start = start + 1;
        let if_end = else_index.unwrap_or(then_index);

        let if_branch = &tokens[if_start..if_end];
        let else_branch = else_index.map(|else_pos| &tokens[else_pos + 1..then_index]);

        Ok((if_branch, else_branch))
    }
}
