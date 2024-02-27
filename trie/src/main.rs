use hashbrown::HashMap;
use std::fs;
use serde_json::Result;

pub struct Trie {
    head: Node,
}

#[derive(Default)]
struct Node {
    end_of_word: bool,
    children: HashMap<char, Node>
}

impl Trie {
    pub fn new() -> Self {
        Self { 
            head: Node::default()
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.head;

        for c in word.chars() {
            node = node.children.entry(c).or_insert(Node::default());
        };
        node.end_of_word = true;
    }

    pub fn search(&mut self, word: String) -> bool {
        let mut node = &self.head;
        
        for c in word.chars() {
            node = match node.children.get(&c) {
                Some(n) => n,
                None => return false
            }
        };
        node.end_of_word
    }
}

fn test_json() -> Result<()> {
    let contents: String = fs::read_to_string("test.json").unwrap();
    let json: Vec<String> = serde_json::from_str(&contents)?;
    let mut trie: Trie = Trie::new();
    let half_size = json.len() / 2;

    for i in 0..half_size {
        trie.insert(json[i].to_string());
    };

    for j in 0..json.len() {
        let res = trie.search(json[j].to_string());
        if j < half_size {
            assert!(res);
        } else {
            assert_eq!(res, false);
        }
    }
    // println!("done");
    Ok(())
}

pub fn main() {
    let _ = test_json();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let mut trie = Trie::new();
        trie.insert("one".to_string());
        trie.insert("two".to_string());
        assert_eq!(trie.search("one".to_string()), true);
        assert_eq!(trie.search("two".to_string()), true);
        assert_eq!(trie.search("three".to_string()), false);
        assert_eq!(trie.search("".to_string()), false);
    }
    
    #[test]
    fn test_json() -> Result<()> {
        let contents: String = fs::read_to_string("test.json").unwrap();
        let json: Vec<String> = serde_json::from_str(&contents)?;
        let mut trie: Trie = Trie::new();
        let half_size = json.len() / 2;
    
        for i in 0..half_size {
            trie.insert(json[i].to_string());
        };

        for j in 0..json.len() {
            let res = trie.search(json[j].to_string());
            if j < half_size {
                assert!(res);
            } else {
                assert_eq!(res, false);
            }
        }
    
        Ok(())
    }
}