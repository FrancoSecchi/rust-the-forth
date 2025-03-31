use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

/// Lee el contenido de un archivo como String
///
/// # Ejemplo
/// ``` text
/// let content = file_manager::read_to_string("ejemplo.fth")?;
/// ```
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn write_to_file<P: AsRef<Path>, C: AsRef<str>>(path: P, content: C) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_ref().as_bytes())?;
    Ok(())
}

pub fn save_stack(stack: &Vec<i16>) -> io::Result<()> {
    let stack_str = stack
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    write_to_file("stack.fth", stack_str)
}

pub fn tokenize(input: &str) -> Vec<String> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();
    let mut current = String::new();

    while let Some(c) = chars.next() {
        if c.is_whitespace() {
            if !current.is_empty() {
                tokens.push(current);
                current = String::new();
            }
        } else if c == '.' && chars.peek() == Some(&'"') {            
            current.push(c); 
            if let Some(next) = chars.next() { 
                current.push(next); 
            }

            if let Some(&next) = chars.peek() {
                if next.is_whitespace() {
                    current.push(' '); 
                    chars.next();
                }
            }

            tokens.push(current); 
            current = String::new();
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}
