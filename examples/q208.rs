/**
 * @lc app=leetcode.cn id=208 lang=rust
 *
 * [208] 实现 Trie (前缀树)
 *
 * https://leetcode-cn.com/problems/implement-trie-prefix-tree/description/
 *
 * algorithms
 * Medium (55.21%)
 * Total Accepted:    4.5K
 * Total Submissions: 7.9K
 * Testcase Example:  '["Trie","insert","search","search","startsWith","insert","search"]\n[[],["apple"],["apple"],["app"],["app"],["app"],["app"]]'
 *
 * 实现一个 Trie (前缀树)，包含 insert, search, 和 startsWith 这三个操作。
 *
 * 示例:
 *
 * Trie trie = new Trie();
 *
 * trie.insert("apple");
 * trie.search("apple");   // 返回 true
 * trie.search("app");     // 返回 false
 * trie.startsWith("app"); // 返回 true
 * trie.insert("app");
 * trie.search("app");     // 返回 true
 *
 * 说明:
 *
 *
 * 你可以假设所有的输入都是由小写字母 a-z 构成的。
 * 保证所有输入均为非空字符串。
 *
 *
 */

use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}

#[derive(Debug)]
struct TrieNode {
    map: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn insert(&mut self, word: &Vec<char>, idx: usize) {
        if idx == word.len() {
            return;
        }

        match self.map.get_mut(&word[idx]) {
            None => {
                let mut node = TrieNode::new();
                node.is_word = idx == word.len() - 1;
                node.insert(word, idx + 1);
                self.map.insert(word[idx], node);
            }
            Some(n) => {
                if idx == word.len() - 1 {
                    n.is_word = true;
                }
                n.insert(word, idx + 1);
            }
        }
    }
    fn new() -> TrieNode {
        TrieNode {
            map: HashMap::new(),
            is_word: false,
        }
    }

    fn search(&self, word: Vec<char>, idx: usize) -> bool {
        match self.map.get(&word[idx]) {
            Some(v) => {
                if idx == word.len() - 1 {
                    return v.is_word;
                }
                v.search(word, idx + 1)
            }
            None => false,
        }
    }

    fn starts_with(&self, prefix: Vec<char>, idx: usize) -> bool {
        match self.map.get(&prefix[idx]) {
            Some(v) => {
                if idx == prefix.len() - 1 {
                    return true;
                }
                v.starts_with(prefix, idx + 1)
            }
            None => false,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        self.root.insert(&word.chars().collect(), 0);
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        if word.is_empty() {
            return false;
        }
        self.root.search(word.chars().collect(), 0)
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        if prefix.is_empty() {
            return false;
        }
        self.root.starts_with(prefix.chars().collect(), 0)
    }
}

//**
// * Your Trie object will be instantiated and called as such:
// * let obj = Trie::new();
// * obj.insert(word);
// * let ret_2: bool = obj.search(word);
// * let ret_3: bool = obj.starts_with(prefix);
// */
fn main() {
    let mut trie = Trie::new();
    dbg!(trie.insert("apple".to_string()));
    dbg!(trie.search("apple".to_string()));
    dbg!(trie.search("app".to_string()));
    dbg!(trie.starts_with("app".to_string()));
    dbg!(trie.insert("app".to_string()));
    dbg!(trie.search("app".to_string()));
}
