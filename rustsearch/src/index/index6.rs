use std::collections::{HashMap, HashSet};
use std::error::Error;

use crate::helpers::*;
use crate::index::Index;

use super::*;

impl Index<HashMap<String, HashSet<String>>> {
    pub fn index6(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut database: HashMap<String, HashSet<String>> = HashMap::new();

        let articles_iter = read_and_clean_file_to_iter(config)?;
        let mut article_titles: Vec<String> = Vec::new();

        for (title, contents) in articles_iter {
            if title != "" {
                article_titles.push(title.to_string());
                for word in contents {
                    database
                        .entry(word.to_string())
                        .or_default()
                        .insert(title.clone());
                }
            }
        }

        Ok(Index {
            database,
            article_titles,
        })
    }

    pub fn single_search(&self, word: &String) -> ArticleTitles {
        Vec::from_iter(
            self.database
                .get(word)
                .unwrap_or(&HashSet::new())
                .to_owned(),
        )
    }
}

impl Search for Index<HashMap<String, HashSet<String>>> {
    fn search(&self, query: &Query) -> ArticleTitles {
        match query.search_type {
            SearchType::SingleWordSearch => self.single_search(&query.search_string),
            _ => panic!(
                "Searchtype {0} not implemented for index6. ",
                query.search_type
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet};

    fn setup_test() -> Index<HashMap<String, Vec<usize>>> {
        let mut database: HashMap<String, Vec<usize>> = HashMap::new();
        database.insert("word1".to_string(), vec![0]);
        database.insert("word2".to_string(), vec![0, 1, 2, 3, 4, 5, 6, 7]);
        database.insert("word3".to_string(), vec![0, 2, 4, 6]);
        database.insert("word4".to_string(), vec![1, 2, 3]);
        let mut article_titles: Vec<String> = Vec::new();
        for i in 0..100 {
            article_titles.push(format!("article {}", i).to_string());
        }
        Index {
            database,
            article_titles,
        }
    }

    #[test]
    fn vec_to_articlelist_works() {
        let test_index = setup_test();

        let vec: Vec<usize> = vec![0, 1];

        let hs = vec!["article 0".to_string(), "article 1".to_string()];
        assert_eq!(test_index.vec_to_articlelist(vec), hs)
    }

    #[should_panic]
    #[test]
    fn vec_to_articlelist_panics_when_out_of_range() {
        let test_index = setup_test();

        let vec: Vec<usize> = vec![100000000];

        test_index.vec_to_articlelist(vec);
    }

    fn search_match(index: &Index<HashMap<String, Vec<usize>>>, query: &str, titles: Vec<&str>) {
        dbg!(&query.to_string());
        let index_result: HashSet<String> =
            HashSet::from_iter(index.boolean_search_naive(&query.to_string()));
        assert_eq!(
            index_result,
            HashSet::from_iter(titles.iter().map(|s| s.to_string()))
        )
    }

    #[test]
    fn find_a_word() {
        let index = setup_test();
        search_match(&index, "  word1 ", vec!["article 0"]);
    }


    #[test]
    fn word_not_in_database() {
        let index = setup_test();
        search_match(&index, "nowhere", vec![]);
    }

    #[test]
    fn the_empty_query() {
        let index = setup_test();
        search_match(&index, "", vec![]);
    }
}

