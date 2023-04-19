//use super::index9_0::{Index9ExtraVariables, Trie, TrieNode};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

use crate::helpers::*;
use crate::index::Index;

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
    pub root: TrieNode,
    pub n_titles: usize,
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
        if v.len() == 0 || v[v.len() - 1] != article_number {
            v.push(article_number)
        }
    }

    pub fn find(&self, string_val: &String) -> Option<Vec<usize>> {
        let mut current = &self.root;

        for c in string_val.chars() {
            if c == '*' {
                // When reading a *, return the subtree from this node
                eprintln!("Start in word");
                return Some(self.get_subtree_match(current).to_vec());
            }
        
            current = match current.children_map.get(&c){
                Some(child) => child,
                None => return None
            };
        }
        // At the end of the string, the last current node is final
        Some(self.articlevec_to_bitvec(current.article_vec.as_ref().unwrap()))
    }

    fn get_subtree_match(&self, node: &TrieNode) -> Vec<usize> {
        match &node.article_vec {
            Some(articles) => node.children_map.values().fold(
                self.articlevec_to_bitvec(articles),
                |acc: Vec<usize>, child: &TrieNode| {
                    self.or_bitvec(acc, self.get_subtree_match(child))
                },
            ),
            None => {
                let mut children = node.children_map.values();
                let first_child = children.next().unwrap();
                children.fold(self.get_subtree_match(first_child), |acc, child| {
                    self.or_bitvec(acc, self.get_subtree_match(child))
                })
            }
        }
    }

    fn articlevec_to_bitvec(&self, articlevec: &Vec<usize>) -> Vec<usize> {
        let arch_bits = usize::BITS as usize;
        let mut bitvec: Vec<usize> = vec![0; (self.n_titles - 1) / arch_bits + 1];

        for n in articlevec {
            let title_bit: usize = 1 << (n % arch_bits);
            bitvec[n / arch_bits] = bitvec[n / arch_bits] | title_bit;
        }
        bitvec
    }

    fn or_bitvec(&self, articlevec1: Vec<usize>, articlevec2: Vec<usize>) -> Vec<usize> {
        articlevec1
            .iter()
            .zip(articlevec2.iter())
            .map(|(l, r)| l | r)
            .collect()
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

        database.n_titles = article_titles.len();

        Ok(Index {
            database,
            extra_variables: Some(Index9ExtraVariables { article_titles }),
        })
    }

    pub fn trie_search_1(&self, query: &String) -> Option<Vec<String>> {
        Some(self.bitvec_to_articlelist(self.database.find(query)?))
    }

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

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs, iter::zip};
    use crate::{helpers::*, index::{gen_query::{gen_a_lot_of_runs_bool, gen_a_lot_of_runs_tries}, self}};

    use super::*;

    fn setup_real() -> Index<Trie, Index9ExtraVariables> {
        let config = Config::build(&[
            "".to_string(),
            "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),
            "9_1".to_string(),
        ])
        .unwrap();
        Index::index9(&config).unwrap()
    }

    fn insert_list_to_trie(trie: &mut Trie, word: &String, a_list: Vec<usize>) {
        for a in a_list {
            trie.insert(word, a)
        }
    }

    fn setup_test() -> Index<Trie, Index9ExtraVariables> {
        let mut database = Trie::new();
        database.insert(&"word1".to_string(), 0);
        insert_list_to_trie(&mut database, &"word1".to_string(), vec![0]);
        insert_list_to_trie(&mut database, &"word2".to_string(), vec![0, 1, 2, 3]);
        insert_list_to_trie(&mut database, &"world".to_string(), vec![0, 2, 4, 6, 7]);
        insert_list_to_trie(&mut database, &"would".to_string(), vec![99, 5, 6, 7]);
        insert_list_to_trie(
            &mut database,
            &"boob".to_string(),
            vec![0, 1, 2, 3, 4, 5, 6, 7],
        );
        insert_list_to_trie(&mut database, &"booby".to_string(), vec![0, 5, 6, 7]);
        insert_list_to_trie(&mut database, &"booty".to_string(), vec![1, 2, 3]);
        let mut article_titles: Vec<String> = Vec::new();
        for i in 0..100 {
            article_titles.push(format!("article {}", i).to_string());
        }
        database.n_titles = 100;
        Index {
            database,
            extra_variables: Some(Index9ExtraVariables { article_titles }),
        }
    }

    fn search_match(index: &Index<Trie, Index9ExtraVariables>, query: &str, titles: Vec<&str>) {
        dbg!(&query.to_string());
        let index_result: HashSet<String> = HashSet::from_iter(
            index
                .trie_search_1(&query.to_string())
                .unwrap_or(Vec::default()),
        );
        assert_eq!(
            index_result,
            HashSet::from_iter(titles.iter().map(|s| s.to_string()))
        )
    }

    #[test]
    fn find_a_word() {
        let index = setup_test();
        search_match(&index, "word1", vec!["article 0"]);
    }

    #[test]
    fn find_a_prefix1() {
        let index = setup_test();
        search_match(
            &index,
            "word*",
            vec!["article 0", "article 1", "article 2", "article 3"],
        );
    }

    #[test]
    fn find_a_prefix2() {
        let index = setup_test();
        search_match(
            &index,
            "wo*",
            vec![
                "article 0",
                "article 1",
                "article 2",
                "article 3",
                "article 4",
                "article 5",
                "article 6",
                "article 7",
                "article 99",
            ],
        );
    }

    #[test]
    fn find_a_prefix3() {
        let index = setup_test();
        search_match(
            &index,
            "boo*",
            vec![
                "article 0",
                "article 1",
                "article 2",
                "article 3",
                "article 4",
                "article 5",
                "article 6",
                "article 7",
            ],
        );
    }

    #[test]
    fn find_prefix_real1() {
        let index = setup_real();
        search_match(&index, "start*", vec!["Autism", "Anarchism"]);
    }

    #[test]
    fn find_prefix_real2() {
        let index = setup_real();
        search_match(&index, "let*", vec!["A", "Anarchism"]);
    }

    #[test]
    fn find_prefix_real3() {
        let index = setup_real();
        search_match(&index, "a*", vec!["A", "Anarchism", "Autism", "Albedo"]);
    }

    #[test]
    fn find_prefix_real4() {
        let index = setup_real();
        search_match(&index, ". *", vec![]);
    }

    #[test]
    fn check_index9_and_index8_get_the_same_results() {
        let files = fs::read_dir("../../data.nosync/");

        for dir in files.unwrap() {
            let file = dir.unwrap().path().into_os_string().into_string().unwrap();
            let filesize = &file[46..file.len()-4];

            if (filesize != "1MB") & (filesize != "2MB"){
                continue;
            }

            let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 10);
            let word_vec = gen_a_lot_of_runs_tries(file.clone(), 10,false);

            let index8 = index::Index::index8(&Config {file_path : file.clone(), indexno : "8".to_string()}).unwrap();
            let index9_0 = index::Index::index9_0(&Config {file_path : file.clone(), indexno : "9".to_string()}).unwrap();
            let index9_1 = index::Index::index9(&Config {file_path : file.clone(), indexno : "9".to_string()}).unwrap();

            let depth_vec = &ast_vec[0];

            for (ast,word) in zip(depth_vec,word_vec)  {
                    let articleList8_0 = index8.vec_to_articlelist(index8.evaluate_syntex_tree_naive(*ast.clone()));
                    let articleList9_0 = index9_0.trie_search(&word).unwrap_or([].to_vec());
                    let articleList9_1 = index9_1.trie_search_1(&word).unwrap_or([].to_vec());

                    assert_eq!(articleList9_0,articleList8_0);
                    assert_eq!(articleList9_1,articleList8_0);

                };

        }
    }
}
