use super::operation::conditional_module::ConditionalToken;
use super::operation::OperationOutput;
use super::operation::OperationType;
use crate::core::error::OperationError;
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
        //println!("{:#?}", tokens);
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

        let mut transformed_tokens: Vec<String> = Vec::new();
        let mut tokens_iter = tokens.iter_mut().peekable();

        while let Some(token) = tokens_iter.next() {
            let mut token = token.to_lowercase();
            if token == ":" {
                if let Some(word_name) = tokens_iter.next() {
                    if word_name.parse::<i16>().is_ok() {
                        return Err(OperationError::InvalidWord);
                    }
                    let mut body = vec![];

                    for def_token in tokens_iter.by_ref() {
                        let mut def_token = def_token.to_lowercase();
                        if def_token == ";" {
                            break;
                        } else {
                            //println!("antes {def_token}");
                            self.append_word_version_suffix(&mut def_token);
                            body.push(def_token.to_string());
                        }
                    }

                    self.word_registry
                        .define_word(word_name.to_lowercase().to_string(), body);
                }
            } else {
                self.append_word_version_suffix(&mut token);
                transformed_tokens.push(token);
            }
        }
        tokens.drain(..);
        tokens.extend(transformed_tokens);
        Ok(())
    }

    fn append_word_version_suffix(&self, token: &mut String) {
        if let Some(word_versions) = self.word_registry.get_word_versions(token) {
            if let Some(last_index) = word_versions.last() {
                token.push_str(&format!("_{}", last_index));
            }
        } else if OperationType::from_token(token).is_some() {
            if let Some(word_versions) = self.word_registry.get_word_versions(token) {
                if let Some(last_index) = word_versions.last() {
                    token.push_str(&format!("_{}", last_index));
                }
            } else {
                token.push_str(&format!("_{}", CANONIC_SUBFIX));
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

    /// Processes the validated tokens, iterating through each token and handling it.
    ///
    /// This function processes a list of tokens, calling `process_single_token` for each one. If the processing of a token fails (returns an error),
    /// it attempts to handle the token as a word lookup by calling `handle_word_lookup`.
    ///
    /// # Arguments
    /// * `tokens` - A slice of `String` containing the tokens to be processed.
    /// * `output` - A mutable reference to a `String` where output will be written.
    fn process_tokens(&mut self, tokens: &[String], output: &mut String) {
        for token in tokens {
            let result = self.process_single_token(token, output);

            if result.is_err() {
                self.handle_word_lookup(token, output);
            }
        }
    }

    /// Processes a single token by attempting to parse it as a number or an operation.
    ///
    /// This function tries to parse the given token as a number. If it succeeds, the number is pushed onto the stack.
    /// If it fails to parse the token as a number, it attempts to execute the token as an operation by calling `execute_operation`.
    ///
    /// # Arguments
    /// * `token` - A reference to a `str` representing the token to be processed.
    /// * `output` - A mutable reference to a `String` where output will be written.
    ///
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
        if token_parts.len() > 1 && token_parts[1] == CANONIC_SUBFIX {
            let original_token = token_parts[0];
            let operation_type =
                OperationType::from_token(original_token).ok_or(OperationError::WordNotFound)?;

            if let Some(operation) = self.operations.get(&operation_type) {
                return operation.apply(&mut self.stack);
            }

            if let Some(operation) = self.output_operations.get(&operation_type) {
                return operation.apply(&mut self.stack, output, original_token);
            }
        }
        Err(OperationError::WordNotFound)
    }

    /// Handles the execution of a token that may refer to a user-defined word.
    /// If the word is not found in the registry, an error is appended to the output.
    /// If the token ends in a valid index, it attempts to execute the corresponding word.
    /// If the token is a standard operation, it attempts to execute it.
    ///
    /// # Parameters
    /// - `token`: A string token that may refer to a user-defined word or an operation.
    /// - `output`: A mutable reference to a string where error messages or results are written.
    ///
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
        } else if let Ok(wi) = index_word_token {
            if let Err(error) = self.execute_word_by_index(wi, word_token, output) {
                self.add_string_output_error(output, error);
            }
        }
    }

    /// Executes a word by its index from the word registry and processes the associated tokens.
    ///
    /// # Parameters
    /// - `word_index`: The index of the word in the word registry.
    /// - `_token`: The original token (not used here but kept for signature compatibility).
    /// - `output`: A mutable reference to a string where error messages or results are written.
    ///
    fn execute_word_by_index(
        &mut self,
        word_index: usize,
        _token: &str,
        output: &mut String,
    ) -> Result<(), OperationError> {
        let tokens_to_process = self.get_word_tokens(word_index);
        self.process_word_tokens(&tokens_to_process, output)
    }

    /// Retrieves the tokens that represent the body of a word definition.
    ///
    /// # Parameters
    /// - `word_index`: The index of the word in the word registry.
    ///
    /// # Returns
    /// Returns a vector of strings representing the tokens of the word body.
    ///
    fn get_word_tokens(&self, word_index: usize) -> Vec<String> {
        self.word_registry.words[word_index]
            .body
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Processes the list of tokens that represent the body of a word definition.
    /// It handles conditional branching with `if`, `else`, and `then` constructs.
    ///
    /// # Parameters
    /// - `tokens`: A slice of strings representing the body of a word to be processed.
    /// - `output`: A mutable reference to a string where error messages or results are written.
    ///    
    fn process_word_tokens(
        &mut self,
        tokens: &[String],
        output: &mut String,
    ) -> Result<(), OperationError> {
        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].as_str() {
                "if" => {
                    let cond = self.stack.pop().ok_or(OperationError::StackUnderflow)?;                    
                    let (then_pos, if_branch, else_branch) =
                        ConditionalToken::extract_branch(tokens, i)?;

                    if cond != 0 {
                        self.process_word_tokens(if_branch, output)?;
                    } else if let Some(else_body) = else_branch {
                        self.process_word_tokens(else_body, output)?;
                    }
                    i = then_pos + 1;
                }
                token => {
                    if let Ok(number) = token.parse::<i16>() {
                        self.push_number(number)?;
                    } else {
                        let word_token_parts: Vec<&str> = token.split("_").collect();
                        let word_token_name = word_token_parts[0];
                        if word_token_parts[1] != CANONIC_SUBFIX {
                            let index_word_token = word_token_parts[1].parse::<usize>();
                            if !self.word_registry.contains_key(word_token_name) {
                                return Err(OperationError::WordNotFound);
                            }
                            if let Ok(wi) = index_word_token {
                                self.execute_word_by_index(wi, word_token_name, output)?;
                            }
                        } else {
                            self.execute_operation(token, output)?;
                        }
                    }
                    i += 1;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;    
    fn create_calculator() -> ForthCalculator {
        ForthCalculator::new(100)
    }

    #[test]
    fn test_extract_single_word() {
        let mut calc = create_calculator();
        let mut tokens = vec![
            ":".to_string(),
            "doble".to_string(),
            "2".to_string(),
            "*".to_string(),
            ";".to_string(),
        ];

        let result = calc.extract_words(&mut tokens);
        assert!(result.is_ok());
        assert_eq!(tokens.len(), 0);
        assert!(calc.word_registry.contains_key("doble"));
    }

    #[test]
    fn test_extract_multiple_words() {
        let mut calc = create_calculator();
        let mut tokens = vec![
            ":".to_string(),
            "cuadrado".to_string(),
            "dup".to_string(),
            "*".to_string(),
            ";".to_string(),
            ":".to_string(),
            "cuatro".to_string(),
            "2".to_string(),
            "2".to_string(),
            "+".to_string(),
            ";".to_string(),
        ];

        let result = calc.extract_words(&mut tokens);
        assert!(result.is_ok());
        assert!(calc.word_registry.contains_key("cuadrado"));
        assert!(calc.word_registry.contains_key("cuatro"));
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_invalid_word_definition_missing_semicolon() {
        let mut calc = create_calculator();
        let mut tokens = vec![
            ":".to_string(),
            "doble".to_string(),
            "2".to_string(),
            "*".to_string(),
            // falta el ";"
        ];

        let result = calc.extract_words(&mut tokens);
        assert_eq!(result, Err(OperationError::InvalidWordFormat));
    }

    #[test]
    fn test_invalid_word_name_is_number() {
        let mut calc = create_calculator();
        let mut tokens = vec![
            ":".to_string(),
            "123".to_string(),
            "1".to_string(),
            "+".to_string(),
            ";".to_string(),
        ];

        let result = calc.extract_words(&mut tokens);
        assert_eq!(result, Err(OperationError::InvalidWord));
    }

    #[test]
    fn test_preserves_other_tokens() {
        let mut calc = create_calculator();
        let mut tokens = vec![
            "3".to_string(),
            "4".to_string(),
            ":".to_string(),
            "sumar".to_string(),
            "+".to_string(),
            ";".to_string(),
            "sumar".to_string(),
        ];

        let result = calc.extract_words(&mut tokens);
        assert!(result.is_ok());

        assert_eq!(tokens, vec!["3", "4", "sumar_0"]);
    }

    #[test]
    fn test_version_suffix_added_properly() {
        let mut calc = create_calculator();
        let mut tokens = vec![
            ":".to_string(),
            "x".to_string(),
            "1".to_string(),
            "+".to_string(),
            ";".to_string(),
            ":".to_string(),
            "x".to_string(),
            "2".to_string(),
            "+".to_string(),
            ";".to_string(),
        ];

        let result = calc.extract_words(&mut tokens);
        assert!(result.is_ok());
        assert!(calc.word_registry.contains_key("x"));

        let versions = calc.word_registry.get_word_versions("x").unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_defined_word_gets_appended_with_version_suffix() {
        let mut calc = create_calculator();
        let mut tokens = vec![
            ":".to_string(),
            "doble".to_string(),
            "2".to_string(),
            "*".to_string(),
            ";".to_string(),
            "doble".to_string(),
        ];

        let result = calc.extract_words(&mut tokens);
        assert!(result.is_ok());
        assert_eq!(tokens, vec!["doble_0"]);
    }

    #[test]
    fn test_multiple_versions_get_correct_suffixes() {
        let mut calc = create_calculator();
        let mut tokens = vec![
            ":".to_string(),
            "x".to_string(),
            "1".to_string(),
            "+".to_string(),
            ";".to_string(),
            ":".to_string(),
            "x".to_string(),
            "2".to_string(),
            "+".to_string(),
            ";".to_string(),
            "x".to_string(),
        ];

        let result = calc.extract_words(&mut tokens);
        assert!(result.is_ok());
        assert_eq!(tokens, vec!["x_1"]);
    }

    #[test]
    fn test_defined_words_in_definition_are_versioned() {
        let mut calc = create_calculator();

        let mut base_def = vec![
            ":".to_string(),
            "uno".to_string(),
            "1".to_string(),
            ";".to_string(),
        ];
        assert!(calc.extract_words(&mut base_def).is_ok());

        let mut composed_def = vec![
            ":".to_string(),
            "duplicar_uno".to_string(),
            "uno".to_string(),
            "uno".to_string(),
            "+".to_string(),
            ";".to_string(),
            "duplicar_uno".to_string(),
        ];
        assert!(calc.extract_words(&mut composed_def).is_ok());

        assert_eq!(composed_def, vec!["duplicar_uno_1"]);
    }
}
