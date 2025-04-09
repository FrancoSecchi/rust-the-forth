use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// Reads the entire contents of a file into a `String`.
///
/// # Arguments
/// * `path` - A value that can be referenced as a path (e.g., a string slice or `Path`).
///
/// # Examples
/// ```text
/// let content = file_manager::read_to_string("example.fth")?;
/// ```
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Writes the provided content to a file at the specified path.
///
/// # Arguments
/// * `path` - A value that can be referenced as a path.
/// * `content` - A value that can be referenced as a string.
///
/// # Examples
/// ```text
/// file_manager::write_to_file("output.txt", "Hello, world!")?;
/// ```
///
pub fn write_to_file<P: AsRef<Path>, C: AsRef<str>>(path: P, content: C) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_ref().as_bytes())?;
    Ok(())
}

/// Saves the stack of numbers to a file named "stack.fth".
///
/// The stack is represented as a slice of `i16` and the numbers are
/// written as a space-separated string.
///
/// # Arguments
/// * `stack` - A slice of `i16` integers representing the stack.
///
/// # Examples
/// ``` text
/// file_manager::save_stack(&[1, 2, 3])?;
/// ```
pub fn save_stack(stack: &[i16]) -> io::Result<()> {
    let stack_str = stack
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    write_to_file("stack.fth", stack_str)
}

/// Tokenizes the input string into a vector of tokens.
///
/// The function splits the input string by whitespace and also supports
/// tokens that begin with a literal marker (`."`) followed by text enclosed
/// in double quotes.
///
/// # Arguments
/// * `input` - The string input to tokenize.
///
/// # Examples
/// ``` text
/// let tokens = file_manager::tokenize("Hello .\" world\"");
/// // tokens: ["Hello", ".\" world\""]
/// ```
pub fn tokenize(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        if chars[i] == '.' {
            if is_print_text_format_valid(&chars, &mut i) {
                process_print_text_operation(&chars, &mut i, &mut tokens);
                continue;
            } else {
                tokens.push(".".to_string());
                i += 1;
                continue;
            }
        }

        let start = i;
        while i < chars.len() && !chars[i].is_whitespace() {
            i += 1;
        }
        tokens.push(chars[start..i].iter().collect());
    }
    tokens
}

/// Checks if the current character position matches the expected start of a print text operation
/// in the format: ."<space>
/// # Arguments
/// * `chars` - A reference to the vector of characters being parsed
/// * `i` - A mutable reference to the current index in the character vector
/// # Example
/// ```text
/// let chars: Vec<char> = ".\" Hello\"".chars().collect();
/// let mut index = 0;
/// assert!(is_print_text_format_valid(&chars, &mut index));
/// ```
fn is_print_text_format_valid(chars: &[char], i: &mut usize) -> bool {
    *i + 2 < chars.len() && chars[*i..*i + 3] == ['.', '"', ' ']
}

/// Processes a print text operation starting at the current index and appends the corresponding
/// token to the tokens vector.
///
/// This function assumes that the `.\" ` format has already been validated.
/// It will collect characters until the closing `"` is found and build a formatted string token.
///
/// # Arguments
/// * `chars` - A reference to the vector of characters being parsed
/// * `i` - A mutable reference to the current index in the character vector
/// * `tokens` - A mutable reference to the vector where the resulting token will be stored
///
/// # Example
/// ```text
/// let chars: Vec<char> = ".\" Hello world\"".chars().collect();
/// let mut index = 0;
/// let mut tokens = vec![];
/// if is_print_text_format_valid(&chars, &mut index) {
///     process_print_text_operation(&chars, &mut index, &mut tokens);
/// }
/// assert_eq!(tokens, vec![".\" Hello world\""]);
/// ```
fn process_print_text_operation(chars: &[char], i: &mut usize, tokens: &mut Vec<String>) {
    let mut literal = String::new();
    literal.push(' ');
    *i += 3;

    while *i < chars.len() && chars[*i] != '"' {
        literal.push(chars[*i]);
        *i += 1;
    }
    if *i < chars.len() && chars[*i] == '"' {
        *i += 1;
    }
    tokens.push(format!(".\"{}\"", literal));
}
