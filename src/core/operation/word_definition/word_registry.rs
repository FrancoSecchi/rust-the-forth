use std::collections::HashMap;

use super::Word;

/// A registry for storing word definitions and their versions.
///
/// This structure holds a list of all words and their definitions (`words`) and a mapping
/// from the word name to a list of indices representing all versions of that word (`current_definition`).
/// Each time a word is redefined, a new definition is added to `words`, and the `current_definition`
/// keeps track of all versions of that word.
#[derive(Debug)]
pub struct WordRegistry {
    /// A vector that stores all word definitions, including older versions.
    pub words: Vec<Word>,
    /// A mapping from the word name to the indices of its current definition(s) in `words`.
    /// This allows for easy retrieval of the latest definition of a word.
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

    /// Checks if a word has been defined in the registry.
    ///
    /// # Parameters
    /// - `key`: The name of the word to check.    
    pub fn contains_key(&self, key: &str) -> bool {
        self.current_definition.contains_key(key)
    }

    /// Retrieves the current version index of the last defined word.
    ///
    /// # Returns
    /// The version index of the most recently added word
    pub fn get_version(&self) -> usize {
        if self.words.is_empty() {
            0
        } else {
            self.words.len() - 1
        }
    }

    /// Retrieves the versions of a word, i.e., all indices in `words` where the word has been defined.
    ///
    /// # Parameters
    /// - `key`: The name of the word for which to fetch versions.
    ///
    /// # Returns
    /// An `Option` containing a reference to a vector of indices representing all versions of the word,
    /// or `None` if the word has not been defined.
    pub fn get_word_versions(&self, key: &str) -> Option<&Vec<usize>> {
        self.current_definition.get(key)
    }

    /// Checks if a specific version of a word exists in the registry.
    ///
    /// # Parameters
    /// - `name`: The name of the word to check.
    /// - `version`: The version index to check for.
    ///     
    pub fn has_version(&self, name: &str, version: usize) -> bool {
        if let Some(versions) = self.current_definition.get(name) {
            versions.contains(&version)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_registry_is_empty() {
        let registry = WordRegistry::new();
        assert!(registry.words.is_empty());
        assert!(registry.current_definition.is_empty());
    }

    #[test]
    fn test_define_single_word() {
        let mut registry = WordRegistry::new();
        registry.define_word("sum".to_string(), vec!["+".to_string()]);

        assert_eq!(registry.words.len(), 1);
        assert_eq!(registry.words[0].name, "sum");
        assert_eq!(registry.words[0].body, vec!["+"]);

        let versions = registry.get_word_versions("sum").unwrap();
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0], 0);
    }

    #[test]
    fn test_define_multiple_versions_of_same_word() {
        let mut registry = WordRegistry::new();

        registry.define_word("print".to_string(), vec![".".to_string()]);
        registry.define_word("print".to_string(), vec![".s".to_string()]);
        registry.define_word("print".to_string(), vec![".x".to_string()]);

        assert_eq!(registry.words.len(), 3);

        let versions = registry.get_word_versions("print").unwrap();
        assert_eq!(versions.len(), 3);
        assert_eq!(versions, &vec![0, 1, 2]);

        assert_eq!(registry.words[versions[0]].body, vec!["."]);
        assert_eq!(registry.words[versions[1]].body, vec![".s"]);
        assert_eq!(registry.words[versions[2]].body, vec![".x"]);
    }

    #[test]
    fn test_contains_key_existing_word() {
        let mut registry = WordRegistry::new();
        registry.define_word("dup".to_string(), vec!["1".to_string()]);
        assert!(registry.contains_key("dup"));
        assert!(!registry.contains_key("swap"));
    }

    #[test]
    fn test_get_version_returns_latest_index() {
        let mut registry = WordRegistry::new();
        assert_eq!(registry.get_version(), 0);

        registry.define_word("a".to_string(), vec!["1".to_string()]);
        assert_eq!(registry.get_version(), 0);

        registry.define_word("b".to_string(), vec!["2".to_string()]);
        assert_eq!(registry.get_version(), 1);

        registry.define_word("a".to_string(), vec!["3".to_string()]);
        assert_eq!(registry.get_version(), 2);
    }

    #[test]
    fn test_get_word_versions_returns_none_for_undefined_word() {
        let registry = WordRegistry::new();
        assert_eq!(registry.get_word_versions("missing"), None);
    }

    #[test]
    fn test_has_version_true_and_false_cases() {
        let mut registry = WordRegistry::new();
        registry.define_word("w".to_string(), vec!["x".to_string()]);
        registry.define_word("w".to_string(), vec!["y".to_string()]);

        assert!(registry.has_version("w", 0));
        assert!(registry.has_version("w", 1));
        assert!(!registry.has_version("w", 2));
        assert!(!registry.has_version("nonexistent", 0));
    }
}
