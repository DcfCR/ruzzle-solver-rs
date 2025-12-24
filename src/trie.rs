pub struct TrieNode {
    ch: Option<char>, // Some(c) for nodes, None for root.
    children: Vec<TrieNode>,
    pub is_terminal: bool,
}

impl TrieNode {
    fn from_char(character: char) -> TrieNode {
        TrieNode {
            ch: Some(character),
            children: Vec::new(),
            is_terminal: false,
        }
    }

    pub fn new_root() -> TrieNode {
        TrieNode {
            ch: None,
            children: Vec::new(),
            is_terminal: false,
        }
    }

    pub fn find_in_children(&self, key: char) -> Option<&TrieNode> {
        self.children.iter().find(|c| c.ch == Some(key))
    }

    fn mut_find_in_children(&mut self, key: char) -> Option<&mut TrieNode> {
        self.children.iter_mut().find(|c| c.ch == Some(key))
    }

    pub fn node_count(&self) -> usize {
        1 + self.children.iter().map(|c| c.node_count()).sum::<usize>()
    }
    pub fn leaf_count(&self) -> usize {
        if self.children.is_empty() {
            1
        } else {
            self.children.iter().map(|c| c.leaf_count()).sum::<usize>()
        }
    }

    pub fn max_depth(&self) -> usize {
        let child_depth = self.children.iter().map(|c| c.max_depth()).max();
        1 + child_depth.unwrap_or(0)
    }

    pub fn add_word(&mut self, word: &str) {
        let mut chars: std::str::Chars = word.chars();
        if let Some(head) = chars.next() {
            let tail: &str = chars.as_str();
            if let Some(child) = self.mut_find_in_children(head) {
                child.add_word(tail);
            } else {
                let mut next_node = TrieNode::from_char(head);
                next_node.add_word(tail);
                self.children.push(next_node);
            }
        } else {
            // None case. i.e. Case for "".
            self.is_terminal = true;
        }
    }

    pub fn contains_word(&self, word: &str) -> bool {
        let mut chars = word.chars();
        match chars.next() {
            Some(head) => match self.find_in_children(head) {
                Some(child) => child.contains_word(chars.as_str()),
                None => false,
            },
            None => self.is_terminal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_words() {
        let mut root = TrieNode::new_root();
        root.add_word("hello");
        root.add_word("he");
        root.add_word("hell");

        assert!(root.contains_word("he"));
        assert!(root.contains_word("hell"));
        assert!(root.contains_word("hello"));

        assert!(!root.contains_word("helloo"));
        assert!(!root.contains_word("hel"));
        assert!(!root.contains_word("h"));
    }

    #[test]
    fn handles_empty_string() {
        let mut root = TrieNode::new_root();
        assert!(!root.contains_word(""));

        root.add_word("");
        assert!(root.contains_word(""));
    }

    #[test]
    fn trie_statistics() {
        let mut root = TrieNode::new_root();
        assert_eq!(root.node_count(), 1);
        assert_eq!(root.leaf_count(), 1);
        assert_eq!(root.max_depth(), 1);

        root.add_word("rust");
        root.add_word("rusty");
        root.add_word("trie");
        root.add_word("tree");
        assert_eq!(root.node_count(), 12); // root node counts.
        assert_eq!(root.leaf_count(), 3); // final 'e's in trie and tree, 'y' in rusty.
        assert_eq!(root.max_depth(), 6); // "rusty". root node counts.
    }
}
