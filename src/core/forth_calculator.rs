use super::operation::OperationOutput;
use super::operation::OperationType;
use crate::core::error::OperationError;
use crate::core::operation::word_definition::word;
use crate::core::operation::word_definition::WordRegistry;
use crate::core::operation::Operation;
use crate::core::operation::{get_all_standar_operations, get_output_operations};
use crate::utils::file_manager;
use std::collections::HashMap;

const CANONIC_SUBFIX: &str = "c";

/// A stack-based calculator implementing a subset of the Forth language.
/// This calculator supports arithmetic operations, boolean operations,
/// stack manipulation, and output operations.
pub struct ForthCalculator {
    /// Maximum allowed stack size.
    max_stack_size: i16,
    /// Stack that stores numeric values for operations.
    stack: Vec<i16>,
    /// Mapping of standard operations (e.g., arithmetic, boolean) to their implementations.
    operations: HashMap<OperationType, Box<dyn Operation>>,
    /// Mapping of output-related operations (e.g., printing, emitting characters).
    output_operations: HashMap<OperationType, Box<dyn OperationOutput>>,

    word_registry: WordRegistry,

    output: String,
}

impl ForthCalculator {
    /// Creates a new instance of `ForthCalculator` with a given stack size.
    ///
    /// # Arguments
    ///
    /// * `stack_size` - The maximum number of elements allowed in the stack.
    pub fn new(stack_size: i16) -> Self {
        ForthCalculator {
            max_stack_size: stack_size,
            stack: Vec::new(),
            operations: get_all_standar_operations(),
            output_operations: get_output_operations(),
            output: String::new(),
            word_registry: WordRegistry::new(),
        }
    }

    pub fn get_output(&self) -> &String {
        &self.output
    }

    /// Returns a reference to the current stack.
    pub fn get_stack(&self) -> &Vec<i16> {
        &self.stack
    }

    /// Checks if a word definition in the token list is valid.
    ///
    /// A valid word definition must:
    /// - Contain exactly one colon `:` and one semicolon `;`.
    /// - Have no other tokens that partially include `:` or `;`.
    ///     
    fn is_valid_word_definition(&mut self, tokens: &[String]) -> bool {
        let total_semicolons = tokens.iter().filter(|token| token == &";").count();
        let total_colons = tokens.iter().filter(|token| token == &":").count();

        let all_colons_valid = tokens.iter().filter(|t| t.contains(':')).all(|t| t == ":");
        let all_semicolons_valid = tokens.iter().filter(|t| t.contains(';')).all(|t| t == ";");

        total_colons == total_semicolons && all_colons_valid && all_semicolons_valid
    }

    /// Validates that the provided tokens are syntactically and semantically correct.
    ///
    /// This function performs two checks:
    /// 1. If there is a custom word definition (indicated by a colon `:`),
    ///    it must be valid as defined by `is_valid_word_definition`.
    /// 2. All tokens must either be:
    ///    - A valid integer
    ///    - A known predefined operation
    ///    - A user-defined word
    ///     
    fn are_valid_tokens(&mut self, tokens: &mut Vec<String>) -> Result<(), OperationError> {
        self.extract_words(tokens)?;
        for token in tokens {
            if let Err(_error) = token.parse::<i16>() {
                let parts: Vec<&str> = token.split('_').collect();
                let word_exists = if parts.len() == 2 {
                    let name = parts[0];
                    if parts[1] != CANONIC_SUBFIX {
                        if let Ok(index) = parts[1].parse::<usize>() {
                            self.word_registry.contains_key(name)
                                && self.word_registry.has_version(name, index)
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                } else {
                    self.word_registry.contains_key(token)
                };
                if !word_exists {
                    return Err(OperationError::WordNotFound);
                }
            }
        }

        Ok(())
    }

    /// Extracts and registers custom word definitions (also known as `words`) from the provided token list
    /// into the `WordRegistry`. It also removes the word definitions from the original token list
    /// to ensure only executable tokens remain.
    ///
    /// The function looks for definition patterns in the form:
    /// `: word_name body_tokens ;`, where:
    /// - `:` marks the beginning of a word definition.
    /// - `word_name` is the name of the custom word.
    /// - `body_tokens` are the tokens that form the implementation of the word.
    /// - `;` ends the definition.
    ///
    /// # Parameters
    ///
    /// * `tokens` - A mutable reference to a `Vec<String>` containing the tokenized input source.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if all definitions were valid and registered successfully.
    /// * `Err(OperationError::InvalidWordFormat)` if any definition is incorrectly formatted.
    ///
    /// # Example
    ///
    /// ```text
    /// let mut tokens = vec![
    ///     "1".to_string(), ":", "foo".to_string(), "2".to_string(), ";".to_string(), "3".to_string(), "foo".to_string()
    /// ];
    /// let mut calc = ForthCalculator::new();
    /// calc.extract_words(&mut tokens)?;
    /// assert_eq!(tokens, vec!["1", "3", "foo"]);
    /// ```
    ///
    pub fn extract_words(&mut self, tokens: &mut Vec<String>) -> Result<(), OperationError> {
        let total_colons = tokens.iter().filter(|token| token == &":").count();
        if total_colons != 0 && !self.is_valid_word_definition(tokens) {
            return Err(OperationError::InvalidWordFormat);
        }

        let mut tokens_iter = tokens.iter_mut().peekable();
        let mut i: usize = 0;
        let mut vec_drain: Vec<(usize, usize)> = vec![];

        while let Some(token) = tokens_iter.next() {
            if token == ":" {
                let start = i;
                i += 1;

                if let Some(word_name) = tokens_iter.next() {
                    i += 1;

                    if let Ok(_) = word_name.parse::<i16>() {
                        return Err(OperationError::InvalidWord);
                    }
                    let mut body = vec![];

                    while let Some(def_token) = tokens_iter.next() {
                        i += 1;
                        if def_token == ";" {
                            break;
                        } else {
                            self.append_word_version_suffix(def_token);
                            body.push(def_token.to_string());
                        }
                    }

                    self.word_registry.define_word(word_name.to_string(), body);

                    if self.word_registry.contains_key(&word_name) {
                        word_name.push_str(&format!("_{}", self.word_registry.get_version()));
                    }

                    let end = i - 1;
                    vec_drain.push((start, end));
                }
            } else {
                self.append_word_version_suffix(token);
                i += 1;
            }
        }

        vec_drain.sort_by(|a, b| b.0.cmp(&a.0));
        for (start, end) in vec_drain {
            tokens.drain(start..=end);
        }

        Ok(())
    }

    fn append_word_version_suffix(&self, token: &mut String) {
        if OperationType::from_token(token).is_some() {
            if let Some(word_versions) = self.word_registry.get_word_versions(token) {
                if let Some(last_index) = word_versions.last() {
                    token.push_str(&format!("_{}", last_index));
                }
            } else {
                token.push_str(&format!("_{}", CANONIC_SUBFIX));
            }
        } else if let Some(word_versions) = self.word_registry.get_word_versions(token) {
            if let Some(last_index) = word_versions.last() {
                token.push_str(&format!("_{}", last_index));
            }
        }
    }

    /// Processes a given file input string by tokenizing it and executing the corresponding operations.
    ///
    /// # Arguments
    ///
    /// * `content` - The input string containing operations and numbers.
    ///
    /// The function iterates over the tokens, executing either number insertion or operations.
    /// If an error occurs, it is logged and execution stops.
    pub fn run(&mut self, content: String) {
        let mut output: String = String::new();
        let mut input_tokens = file_manager::tokenize(&content);

        if let Err(error) = self.are_valid_tokens(&mut input_tokens) {
            self.add_string_output_error(&mut output, error);
        } else {
            self.process_tokens(&input_tokens, &mut output);
        }

        if let Err(_e) = file_manager::save_stack(&self.stack) {
            self.add_string_output_error(&mut output, OperationError::FailWritingFile);
        }
        self.output = output;
    }

    /// Appends an error message to the output string.
    ///
    /// # Arguments
    ///
    /// * `output` - The output string where the error message will be appended.
    /// * `error` - The error that occurred.
    fn add_string_output_error(&mut self, output: &mut String, error: OperationError) {
        output.push_str(&format!("{}", error));
    }

    /// Pushes a number onto the stack.
    ///
    /// # Arguments
    ///
    /// * `number` - The number to be pushed onto the stack.
    fn push_number(&mut self, number: i16) -> Result<(), OperationError> {
        if self.stack.len() == self.max_stack_size as usize {
            return Err(OperationError::StackOverflow);
        }

        self.stack.push(number);
        Ok(())
    }

    // Process the validated tokens
    fn process_tokens(&mut self, tokens: &[String], output: &mut String) {
        for token in tokens {
            let result = self.process_single_token(token, output);

            if let Err(_) = result {
                self.handle_word_lookup(token, output);
            }
        }
    }

    fn process_single_token(
        &mut self,
        token: &str,
        output: &mut String,
    ) -> Result<(), OperationError> {
        match token.parse::<i16>() {
            Ok(number) => self.push_number(number),
            Err(_) => self.execute_operation(token, output),
        }
    }
    /// Executes an operation based on the provided token.
    ///
    /// # Arguments
    ///
    /// * `token` - The string representation of the operation.
    /// * `output` - A mutable reference to the output string (used for operations that produce output).
    fn execute_operation(
        &mut self,
        token: &str,
        output: &mut String,
    ) -> Result<(), OperationError> {
        let token_parts: Vec<&str> = token.split('_').collect();
        if token_parts.len() > 1 {
            if token_parts[1] == CANONIC_SUBFIX {
                let original_token = token_parts[0];
                let operation_type = OperationType::from_token(original_token)
                    .ok_or(OperationError::WordNotFound)?;

                if let Some(operation) = self.operations.get(&operation_type) {
                    return operation.apply(&mut self.stack);
                }

                if let Some(operation) = self.output_operations.get(&operation_type) {
                    return operation.apply(&mut self.stack, output, original_token);
                }
            }
        }
        Err(OperationError::WordNotFound)
    }

    // Handle word lookup and execution when a token isn't a number or standard operation
    fn handle_word_lookup(&mut self, token: &str, output: &mut String) {
        let token_parts: Vec<&str> = token.split('_').collect();
        let word_token = token_parts[0];
        if !self.word_registry.contains_key(word_token) {
            self.add_string_output_error(output, OperationError::WordNotFound);
            return;
        }

        let index_word_token = token_parts[1].parse::<usize>();
        if index_word_token.is_err() {
            if let Err(error) = self.execute_operation(token, output) {
                self.add_string_output_error(output, error);
            }
        } else {
            if let Ok(wi) = index_word_token {
                if let Err(error) = self.execute_word_by_index(wi, word_token, output) {
                    self.add_string_output_error(output, error);
                }
            }
        }
    }

    fn execute_word_by_index(
        &mut self,
        word_index: usize,
        _token: &str,
        output: &mut String,
    ) -> Result<(), OperationError> {
        let tokens_to_process = self.get_word_tokens(word_index);
        self.process_word_tokens(&tokens_to_process, output)
    }

    // Extract tokens from a word definition by index
    fn get_word_tokens(&self, word_index: usize) -> Vec<String> {
        self.word_registry.words[word_index]
            .body
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    // Process all tokens from a word body
    fn process_word_tokens(
        &mut self,
        tokens: &[String],
        output: &mut String,
    ) -> Result<(), OperationError> {
        for word_token in tokens {
            if let Ok(number) = word_token.parse::<i16>() {
                self.push_number(number)?;
            } else {
                let word_token_parts: Vec<&str> = word_token.split("_").collect();
                let word_token_name = word_token_parts[0];
                if word_token_parts[1] != CANONIC_SUBFIX {
                    let index_word_token = word_token_parts[1].parse::<usize>();
                    if !self.word_registry.contains_key(word_token_name) {
                        return Err(OperationError::WordNotFound);
                    }
                    // Check if this is a nested word call
                    if let Ok(wi) = index_word_token {
                        if let Err(error) = self.execute_word_by_index(wi, word_token_name, output)
                        {
                            return Err(error);
                        }
                    }
                } else {
                    self.execute_operation(word_token, output)?;
                }
            }
        }
        Ok(())
    }
}
