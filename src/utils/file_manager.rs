use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::Chars;

// Reads the entire contents of a file into a `String`.
///
/// # Arguments
/// * `path` - A value that can be referenced as a path (e.g., a string slice or `Path`).
///
/// # Examples
/// ```text
/// let content = file_manager::read_to_string("example.fth")?;
/// ```
///
/// # Errors
/// Returns an `io::Error` if the file cannot be opened or read.
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
/// # Errors
/// Returns an `io::Error` if the file cannot be created or written to.
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
///
/// # Errors
/// Returns an `io::Error` if the file cannot be written.
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
/// let tokens = file_manager::tokenize("Hello .\"world\"");
/// // tokens: ["Hello", ".\"world\""]
/// ```
pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c == '.' {
            let mut candidate = String::new();
            candidate.push(c);
            chars.next();
            if let Some(&next_char) = chars.peek() {
                if next_char == '"' {
                    candidate.push('"');
                    chars.next();
                    let mut literal = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '"' {
                            break;
                        }
                        literal.push(ch);
                    }

                    tokens.push(format!(".\"{}\"", literal));
                    continue;
                } else {
                    tokens.push(candidate);
                    continue;
                }
            } else {
                tokens.push(candidate);
                continue;
            }
        }
        tokens.push(get_normal_token(&mut chars));
    }
    tokens
}

/// Extracts a normal token from the given character iterator.
///
/// The token is constructed by reading characters until a whitespace is encountered.
///
/// # Arguments
/// * `chars` - A mutable peekable iterator over characters.
///
/// # Returns
/// A `String` representing the token.
fn get_normal_token(chars: &mut std::iter::Peekable<Chars>) -> String {
    let mut token = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            break;
        }
        token.push(ch);
        chars.next();
    }
    token
}
