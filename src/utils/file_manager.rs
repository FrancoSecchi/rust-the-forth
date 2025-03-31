use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::str::Chars;


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

pub fn save_stack(stack: &[i16]) -> io::Result<()> {
    let stack_str = stack
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    write_to_file("stack.fth", stack_str)
}

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
