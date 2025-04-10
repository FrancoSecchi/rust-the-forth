use std::collections::HashMap;

use super::Word;

#[derive(Debug)]

pub struct WordRegistry {
    /// A vector with all word definitions (including older versions).
    pub words: Vec<Word>,
    /// A mapping from a word name to the index in `words` that corresponds to its current definition.
    pub current_definition: HashMap<String, Vec<usize>>,
}

impl Default for WordRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl WordRegistry {
    /// Creates a new, empty registry.
    pub fn new() -> Self {
        WordRegistry {
            words: Vec::new(),
            current_definition: HashMap::new(),
        }
    }

    /// Defines a new word.
    ///
    /// If the word has been defined previously, this method adds a new definition and updates the
    /// mapping for that name, leaving older definitions intact. This way, any words that referenced
    /// a previous definition retain their behavior.
    pub fn define_word(&mut self, name: String, body: Vec<String>) {
        self.words.push(Word {
            name: name.to_string(),
            body,
        });

        let new_index = self.words.len() - 1;

        let exists = self.current_definition.contains_key(&name);

        if exists {
            if let Some(vec) = self.current_definition.get_mut(&name) {
                vec.push(new_index);
            }
        } else {
            self.current_definition.insert(name, vec![new_index]);
        }

    }
    pub fn contains_key(&self, key: &str) -> bool {
        self.current_definition.contains_key(key)
    }

    pub fn get_version(&self) -> usize {
        if self.words.len() == 0 {
            0
        } else {
            self.words.len() - 1
        }
    }

    pub fn get_word_versions(&self, key: &str) -> Option<&Vec<usize>> {
        self.current_definition.get(key)
    }

    pub fn has_version(&self, name: &str, version: usize) -> bool {
        if let Some(versions) = self.current_definition.get(name) {
            versions.contains(&version)
        } else {
            false
        }
    }

}
