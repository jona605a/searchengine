use std::collections::HashMap;
use std::error::Error;

use crate::helpers::*;
use crate::index::Index;

use super::*;

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

    pub fn find_prefix(&self, string_val: &String) -> Vec<usize> {
        let mut current = &self.root;
        for c in string_val.chars() {
            if c == '*' {
                // When reading a *, return the subtree from this node
                return self.get_subtree_match(current).to_vec();
            }

            current = match current.children_map.get(&c) {
                Some(child) => child,
                None => return vec![],
            };
        }
        // At the end of the string, the last current node is final
        self.articlevec_to_bitvec(current.article_vec.as_ref().unwrap())
    }

    pub fn find_single(&self, string_val: &String) -> Vec<usize> {
        let mut current = &self.root;

        for c in string_val.chars() {
            current = match current.children_map.get(&c) {
                Some(child) => child,
                None => return vec![],
            };
        }
        // At the end of the string, the last current node is final
        self.articlevec_to_bitvec(current.article_vec.as_ref().unwrap())
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

impl Index<Trie> {
    pub fn index9_1(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut database = Trie::new();

        let articles_iter = read_and_clean_file_to_iter(config)?;
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
            article_titles,
        })
    }

    pub fn prefix_search(&self, query: &String) -> ArticleTitles {
        self.bitvec_to_articlelist(self.database.find_prefix(query))
    }

    pub fn single_search(&self, query: &String) -> ArticleTitles {
        self.bitvec_to_articlelist(self.database.find_single(query))
    }

    pub fn bitvec_to_articlelist(&self, bitvecs: Vec<usize>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let titles = &self.article_titles;
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

impl Search for Index<Trie> {
    fn search(&self, query: &Query) -> ArticleTitles {
        match query.search_type {
            SearchType::SingleWordSearch => self.single_search(&query.search_string),
            SearchType::PrefixSearch => self.prefix_search(&query.search_string),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        helpers::*,
        index::{
            self,
            gen_query::gen_a_lot_of_runs_tries,
        },
    };
    use std::{collections::HashSet, fs};

    use super::*;

    fn setup_real() -> Index<Trie> {
        let config = Config::build(&[
            "".to_string(),
            "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),
            "9_1".to_string(),
        ]);
        Index::index9_1(&config).unwrap()
    }

    fn insert_list_to_trie(trie: &mut Trie, word: &String, a_list: Vec<usize>) {
        for a in a_list {
            trie.insert(word, a)
        }
    }

    fn setup_test() -> Index<Trie> {
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
            article_titles,
        }
    }

    fn search_match(index: &Index<Trie>, query: &str, titles: Vec<&str>) {
        let index_result: HashSet<String> =
            HashSet::from_iter(index.prefix_search(&query.to_string()));
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
        search_match(&index, "start*", vec!["Anarchism"]);
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
        if files.is_err() {
            return
        }

        for dir in files.unwrap() {
            let file = dir.unwrap().path().into_os_string().into_string().unwrap();
            let filesize = &file[46..file.len() - 4];

            if (filesize != "1MB") & (filesize != "2MB") {
                continue;
            }

            let word_vec = gen_a_lot_of_runs_tries(file.clone(), 10, false);

            let index8 = index::Index::index8(&Config {
                file_path: file.clone(),
                indexno: "8".to_string(),
            })
            .unwrap();
            let index9_0 = index::Index::index9_0(&Config {
                file_path: file.clone(),
                indexno: "9".to_string(),
            })
            .unwrap();
            let index9_1 = index::Index::index9_1(&Config {
                file_path: file.clone(),
                indexno: "9".to_string(),
            })
            .unwrap();

            for word in word_vec {
                let query1 = Query {
                    search_string: word.clone(),
                    search_type: SearchType::PrefixSearch,
                };

                let query2 = Query {
                    search_string: word.clone(),
                    search_type: SearchType::BooleanSearch("Naive".to_string())
                };

                let article_list8_0 = index8.search(&query2);
                let article_list9_0 = index9_0.search(&query1);
                let article_list9_1 = index9_1.search(&query1);

                assert_eq!(article_list9_0, article_list8_0);
                assert_eq!(article_list9_1, article_list8_0);
            }
        }
    }
}
