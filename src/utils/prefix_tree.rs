use ahash::{AHashMap, AHashSet};

#[derive(Debug)]
pub struct PrefixTree {
    children: AHashMap<char, Box<PrefixTree>>,
    is_leaf: bool,
}

impl PrefixTree {
    pub fn new() -> Self {
        Self { children: AHashMap::new(), is_leaf: false }
    }

    pub fn from_vec(items: Vec<&str>) -> Self {
        let mut tree = Self::new();
        for item in items {
            tree.insert(item);
        }

        tree
    }

    pub fn insert(&mut self, str: &str) {
        if str.len() == 0 {
            self.is_leaf = true;
            return;
        }

        let head = str.chars().next().unwrap();
        if let Some(child_tree) = self.children.get_mut(&head) {
            child_tree.insert(&str[1..]);
        } else {
            let mut child_tree = PrefixTree::new();
            child_tree.insert(&str[1..]);
            self.children.insert(head, Box::new(child_tree));
        }
    }

    pub fn contains(&self, str: &str) -> bool {
        if str.len() == 0 {
            return self.is_leaf;
        }

        let head = str.chars().next().unwrap();
        if let Some(child_tree) = self.children.get(&head) {
            return child_tree.contains(&str[1..]);
        }

        false
    }

    pub fn prefixes_of(&self, str: &str) -> AHashSet<String> {
        let mut prefixes = AHashSet::new();
        for i in 1..=str.len() {
            let sub_str = &str[0..i];
            if self.contains(sub_str) {
                prefixes.insert(sub_str.to_string());
            }
        }
        prefixes
    }
}