use super::index9_0::{Index9ExtraVariables, Trie, TrieNode};
use crate::index::Index;

impl Trie {
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

    pub fn find_1(&self, string_val: &String) -> Option<Vec<usize>> {
        let mut current = &self.root;
        for c in string_val.chars() {
            if c == '*' {
                // When reading a *, return the subtree from this node
                return Some(self.get_subtree_match(current).to_vec());
            }
            if !current.children_map.contains_key(&c) {
                return None;
            }
            current = current.children_map.get(&c).unwrap();
        }
        // At the end of the string, the last current node is final
        Some(self.articlevec_to_bitvec(current.article_vec.as_ref().unwrap()))
    }
}

impl Index<Trie, Index9ExtraVariables> {
    pub fn trie_search_1(&self, query: &String) -> Option<Vec<String>> {
        Some(self.bitvec_to_articlelist(self.database.find_1(query)?))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::helpers::*;

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
}
