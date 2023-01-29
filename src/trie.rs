use std::collections::HashMap;

#[derive(Debug)]

pub struct TrieNode {
    pub children: HashMap<char, TrieNode>,
    pub is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            let next_node = current_node.children.entry(c).or_insert(TrieNode::new());
            current_node = next_node;
        }
        current_node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current_node = &self.root;

        for c in word.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        return current_node.is_word;
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = &self.root;

        for c in prefix.chars() {
            match current_node.children.get(&c) {
                Some(next_node) => current_node = next_node,
                None => return false,
            }
        }

        return true;
    }
}
