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
