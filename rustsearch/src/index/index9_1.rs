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
    pub article_vec: Option<Vec<usize>>
}

impl TrieNode {
    pub fn new() -> TrieNode {
        TrieNode { children_map: HashMap::new(), article_vec: None }
    }

    pub fn insert_child(&mut self, char: char) {
        self.children_map.insert(char, TrieNode::new());
    }
}

pub struct Trie {
    root: TrieNode, 
    n_titles: usize
}

impl Trie {
    pub fn insert(&mut self, string_val: String, article_number: usize) {
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
        current.article_vec.unwrap().push(article_number);
    }
    
    
    pub fn find(&self, string_val: String) -> &Option<Vec<usize>> {
        let mut current = &self.root;
        for c in string_val.chars() {
            if !current.children_map.contains_key(&c) {
                return &None;
            }
            current = current.children_map.get(&c).unwrap();
        }
        // At the end of the string, the last current node is final
        &current.article_vec
    }

    pub fn find_prefix(&self, string_val: String) -> Option<Vec<usize>> {
        let mut current = &self.root;
        for c in string_val.chars() {
            if !current.children_map.contains_key(&c) {
                return None;
            }
            current = current.children_map.get(&c).unwrap();
        }
        Some(self.get_subtree(current).to_vec())
    }

    // fn get_subtree<'a>(&'a self, node: &'a TrieNode) -> &Vec<usize> {
    //     node.children_map.values().fold(self.to_bitvec(&node.article_vec), 
    //         |acc: &Option<Vec<usize>>, child| 
    //             &Some(self.get_subtree(child).iter()
    //             .zip(
    //                 acc.expect("Vi tog fejl2").iter()
    //             )
    //             .map(|(l, r)| l | r)
    //             .collect())
    //         ).as_ref().unwrap()
    // }


    fn to_bitvec(&self, articlevec: &Vec<usize>) -> Vec<usize> {
        let arch_bits = usize::BITS as usize;
        let mut bitvec: Vec<usize> = vec![0; (self.n_titles - 1) / arch_bits + 1];

        for n in articlevec {
            let title_bit: usize = 1 << (n % arch_bits);
            bitvec[n / arch_bits] = bitvec[n / arch_bits] | title_bit;
        }
        bitvec
    }

    fn or_bitvec(&self, articlevec1: Vec<usize>,articlevec2: Vec<usize>) -> Vec<usize>{
        articlevec1.iter().zip(articlevec2.iter()).map(|(l, r)| l | r).collect()
    }

    fn get_subtree(&self, node: &TrieNode) -> Vec<usize> {

        match &node.article_vec {
            Some(articles) => node.children_map.values().fold(
                self.to_bitvec(articles),|acc:Vec<usize> ,child:&TrieNode| (self.or_bitvec(acc, self.get_subtree(child)))),
            None => {
                let mut children = node.children_map.values();
                let firstChild = children.next().unwrap();
                children.fold(self.get_subtree(firstChild),|acc,child| self.or_bitvec(acc, self.get_subtree(child)))
            }
        }
    }



}


#[allow(dead_code)]
impl Index<HashMap<String, Vec<usize>>, Index9ExtraVariables> {
    pub fn index9(
        config: &Config,
    ) -> Result<Index<HashMap<String, Vec<usize>>, Index9ExtraVariables>, Box<dyn Error>> {
        let mut database: HashMap<String, Vec<usize>> = HashMap::new();

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
                    let v = database.entry(word.to_string()).or_default();
                    if (v.len() == 0) || (v[v.len() - 1] != article_titles.len() - 1) {
                        v.push(article_titles.len() - 1)
                    }
                }
            }
        }

        Ok(Index {
            database,
            extra_variables: Some(Index9ExtraVariables { article_titles }),
        })
    }
}
