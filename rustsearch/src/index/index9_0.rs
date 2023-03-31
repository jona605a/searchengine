use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

use crate::helpers::*;
use crate::index::Index;
// use crate::parsing::*;

#[derive(Debug)]
pub struct Index9ExtraVariables {
    pub article_titles: Vec<String>,
}

pub struct TrieNode {
    pub children_map: HashMap<char, TrieNode>,
    pub article_vec: Option<Vec<usize>>,
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode {
            children_map: HashMap::new(),
            article_vec: None,
        }
    }

    pub fn insert_child(&mut self, char: char) {
        self.children_map.insert(char, TrieNode::new());
    }
}

pub struct Trie {
    root: TrieNode,
    n_titles: usize,
}

impl Trie {
    pub fn new() -> Trie {
        Trie {
            root: TrieNode::new(),
            n_titles: 0,
        }
    }

    pub fn insert(&mut self, string_val: &String, article_number: usize) {
        let mut current = &mut self.root;
        for c in string_val.chars() {
            if !current.children_map.contains_key(&c) {
                current.insert_child(c);
            }
            current = current.children_map.get_mut(&c).unwrap();
        }
        // At the end of the string, the last current node is final
        if current.article_vec.is_none() {
            current.article_vec = Some(vec![]);
        }
        let v = current.article_vec.as_mut().unwrap();
        if v[v.len()-1] != article_number {
            v.push(article_number)
        }
    }

    pub fn find(&self, string_val: &String) -> Option<Vec<usize>> {
        let mut current = &self.root;
        for c in string_val.chars() {
            if !current.children_map.contains_key(&c) {
                return None;
            }
            current = current.children_map.get(&c).unwrap();
        }
        // At the end of the string, the last current node is final
        Some(self.to_bitvec(&current.article_vec))
    }

    pub fn find_prefix(&self, string_val: &String) -> Option<Vec<usize>> {
        let mut current = &self.root;
        for c in string_val.chars() {
            if !current.children_map.contains_key(&c) {
                return None;
            }
            current = current.children_map.get(&c).unwrap();
        }
        Some(self.get_subtree(current).to_vec())
    }

    fn get_subtree(&self, node: &TrieNode) -> Vec<usize> {
        node.children_map.values().fold(
            self.to_bitvec(&node.article_vec),
            |acc: Vec<usize>, child| {
                self.get_subtree(child)
                    .iter()
                    .zip(acc.iter())
                    .map(|(l, r)| l | r)
                    .collect()
            },
        )
    }

    fn to_bitvec(&self, articlevec: &Option<Vec<usize>>) -> Vec<usize> {
        let arch_bits = usize::BITS as usize;
        let mut bitvec: Vec<usize> = vec![0; (self.n_titles - 1) / arch_bits + 1];
        if articlevec.is_none() {
            return bitvec;
        }
        for n in articlevec.as_ref().unwrap() {
            let title_bit: usize = 1 << (n % arch_bits);
            bitvec[n / arch_bits] = bitvec[n / arch_bits] | title_bit;
        }

        bitvec
    }
}

impl Index<Trie, Index9ExtraVariables> {
    pub fn index9(config: &Config) -> Result<Index<Trie, Index9ExtraVariables>, Box<dyn Error>> {
        let mut database = Trie::new();

        let filecontents = read_file_to_string(&config.file_path)?;
        let re = Regex::new(r"\. |\.\n|\n\n|; |[\[\]\{\}\\\n\(\) ,:/=?!*]").unwrap();

        // Articles are seperated by the delimiter "---END.OF.DOCUMENT---"
        // In each article, it is assumed that the first line is the title, ending in a '.'
        // The contents of each article is split according to the regular expression.
        let articles_iter = filecontents.split("---END.OF.DOCUMENT---").map(|a| {
            let (title, contents) = a.trim().split_once(".\n").unwrap_or(("", ""));
            (title.to_string(), re.split(contents))
        });
        let mut article_titles: Vec<String> = Vec::new();

        for (title, contents) in articles_iter {
            if title != "" {
                article_titles.push(title.to_string());
                for word in contents {
                    database.insert(&word.to_string(), article_titles.len() - 1);
                }
            }
        }

        Ok(Index {
            database,
            extra_variables: Some(Index9ExtraVariables { article_titles }),
        })
    }

    

    // Copied from index7
    pub fn bitvec_to_articlelist(&self, bitvecs: Vec<usize>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let titles = &self.extra_variables.as_ref().unwrap().article_titles;
        for i in 0..bitvecs.len() {
            for bit in 0..64 {
                if (1 << bit) & bitvecs[i] > 0 {
                    if titles.len() <= i * 64 + bit {
                        continue;
                    }
                    output.push(titles[i * 64 + bit].clone());
                }
            }
        }
        output
    }
}
