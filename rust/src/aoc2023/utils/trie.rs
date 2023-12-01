use std::collections::HashMap;

#[derive(Default, Debug)]
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;

        for c in word.chars() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_word = true;
    }

    pub fn contains_exact(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for ch in word.chars() {
            if let Some(node) = current_node.children.get(&ch) {
                current_node = node;
            } else {
                return false;
            }
        }

        current_node.is_word
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;

        for ch in word.chars() {
            if let Some(node) = current_node.children.get(&ch) {
                current_node = node;
            } else {
                return false;
            }
        }

        true
    }
}
