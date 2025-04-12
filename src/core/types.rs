use super::error::OperationError;

/// The `BranchResult` type is used to represent the result of extracting the branches of a conditional 
/// block (`if` / `else`). It returns a `Result` containing:
/// - The index of the `then` token (as a `usize` value).
/// - A slice containing the tokens for the `if` branch.
/// - An `Option` containing a slice of tokens for the `else` branch, or `None` if there is no `else` branch.
/// 
/// This type is used by functions that extract the branches of conditionals, providing a clear way to 
/// handle cases where the branches cannot be retrieved due to errors (e.g., out-of-bounds indexes).
/// 
pub type BranchResult<'a> = Result<
    (
        usize,                // then index
        &'a [String],         // 'if' branch tokens
        Option<&'a [String]>, // else branch tokens (optional)
    ),
    OperationError,
>;

/// Type alias for a result containing slices of the `if` and `else` branches.
/// 
/// The `BranchSlices` type is used to represent the outcome of extracting slices of the tokens 
/// that form the `if` and `else` branches in a conditional expression. It returns a tuple:
/// - The first element is a slice of tokens for the `if` branch.
/// - The second element is an `Option` containing a slice of tokens for the `else` branch, or `None` if there is no `else`.
/// 
/// This type alias simplifies the handling of the result of `get_branch_slices` and ensures consistent error handling.
pub type BranchSlices<'_slice_tokens> = Result<
    (
        &'_slice_tokens [String],         // slice of if branch tokens
        Option<&'_slice_tokens [String]>, // slice of else branch tokens
    ),
    OperationError,                    // Error if the branches cannot be retrieved
>;