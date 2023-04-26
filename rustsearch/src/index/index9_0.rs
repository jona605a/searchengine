use regex::Regex;
use std::error::Error;

use crate::helpers::*;
use crate::index::Index;

use super::*;

pub struct TrieNodeLin {
    pub children_vec: Vec<(char, TrieNodeLin)>,
    pub article_vec: Option<Vec<usize>>,
}

impl TrieNodeLin {
    pub fn new() -> TrieNodeLin {
        TrieNodeLin {
            children_vec: Vec::new(),
            article_vec: None,
        }
    }

    pub fn insert_child(&mut self, char: char) {
        let child = TrieNodeLin::new();
        self.children_vec.push((char, child))
    }
}

pub struct TrieLin {
    pub root: TrieNodeLin,
    pub n_titles: usize,
}

impl TrieLin {
    pub fn new() -> TrieLin {
        TrieLin {
            root: TrieNodeLin::new(),
            n_titles: 0,
        }
    }

    pub fn insert(&mut self, string_val: &String, article_number: usize) {
        let mut current = &mut self.root;
        for c in string_val.chars() {
            let mut cld_idx = 0;
            for (cld_char, _) in &current.children_vec {
                if *cld_char == c {
                    break;
                }
                cld_idx += 1;
            }

            if cld_idx == current.children_vec.len() {
                // The char was not in the children, so add it
                current.insert_child(c)
            }

            current = &mut current.children_vec[cld_idx].1;
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
            let mut cld_idx = 0;
            for (cld_char, _) in &current.children_vec {
                if *cld_char == c {
                    break;
                }
                cld_idx += 1;
            }
            if cld_idx == current.children_vec.len() {
                return vec![];
            }
            current = &current.children_vec[cld_idx].1;
        }
        // At the end of the string, the last current node is final
        self.articlevec_to_bitvec(current.article_vec.as_ref().unwrap())
    }

    pub fn find_single(&self, string_val: &String) -> Vec<usize> {
        let mut current = &self.root;
        for c in string_val.chars() {
            let mut cld_idx = 0;
            for (cld_char, _) in &current.children_vec {
                if *cld_char == c {
                    break;
                }
                cld_idx += 1;
            }
            if cld_idx == current.children_vec.len() {
                return vec![];
            }
            current = &current.children_vec[cld_idx].1;
        }
        // At the end of the string, the last current node is final
        self.articlevec_to_bitvec(current.article_vec.as_ref().unwrap())
    }

    fn get_subtree_match(&self, node: &TrieNodeLin) -> Vec<usize> {
        match &node.article_vec {
            Some(articles) => node.children_vec.iter().fold(
                self.articlevec_to_bitvec(articles),
                |acc: Vec<usize>, (_, child)| self.or_bitvec(acc, self.get_subtree_match(child)),
            ),
            None => {
                let mut children = node.children_vec.iter();
                let first_child = &children.next().unwrap().1;
                children.fold(self.get_subtree_match(first_child), |acc, (_, child)| {
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

impl Index<TrieLin> {
    pub fn index9_0(
        config: &Config,
    ) -> Result<Self, Box<dyn Error>> {
        let mut database = TrieLin::new();

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
            article_titles
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

impl Search for Index<TrieLin> {
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
    use crate::helpers::*;
    use std::collections::HashSet;

    use super::*;

    fn setup_real() -> Index<TrieLin> {
        let config = Config::build(&[
            "".to_string(),
            "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),
            "9_0".to_string(),
        ])
        .unwrap();
        Index::index9_0(&config).unwrap()
    }

    fn insert_list_to_trie(trie: &mut TrieLin, word: &String, a_list: Vec<usize>) {
        for a in a_list {
            trie.insert(word, a)
        }
    }

    fn setup_test() -> Index<TrieLin> {
        let mut database = TrieLin::new();
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
            article_titles
        }
    }

    fn search_match(index: &Index<TrieLin>, query: &str, titles: Vec<&str>) {
        dbg!(&query.to_string());
        let index_result: HashSet<String> = HashSet::from_iter(
            index
                .prefix_search(&query.to_string())
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
}
