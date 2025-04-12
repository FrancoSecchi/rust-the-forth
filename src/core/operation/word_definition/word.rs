/// A structure representing a word in the registry.
///
/// A word consists of a name and a body. The `body` is typically a sequence of operations or
/// instructions associated with the word. This structure allows us to store and manipulate word definitions.
#[derive(Debug)]
pub struct Word {
    /// The name of the word (e.g., `dup`, `+`, `swap`, etc.).
    pub name: String,
    /// The body of the word, which is a sequence of operations or instructions that define its behavior.
    pub body: Vec<String>,
}
