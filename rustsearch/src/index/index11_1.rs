#![allow(non_snake_case)]
use crate::index::index10_1::{boyer_moore, boyer_moore_preprocess};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

use super::Index;

impl Index<HashMap<(String, String, String), Vec<usize>>> {
    pub fn exact_triples_search(&self, query: &String) -> Vec<String> {
        // Split sentence into words
        // Get article set for each word, and find intersection
        let mut words_iter = query.split_ascii_whitespace();
        let mut prv1 = match words_iter.next() {
            None => return vec![],
            Some(w) => w,
        };
        let mut prv2 = match words_iter.next() {
            None => return vec![],
            Some(w) => w,
        };
        let mut art_lists = vec![];

        for word in words_iter {
            let triple = (prv1.to_owned(), prv2.to_owned(), word.to_owned());
            match dbg!(self.database.get(dbg!(&triple))) {
                None => return vec![],
                Some(al) => art_lists.push(al),
            }
            prv1 = prv2;
            prv2 = word;
        }
        dbg!(&art_lists);

        let mut art_iter = art_lists.iter();
        let first_art = match art_iter.next() {
            None => return vec![],
            Some(&x) => x,
        };

        let art_intersect = art_lists.iter().fold(
            HashSet::from_iter(first_art),
            |acc: HashSet<&usize>, &art_lis| {
                HashSet::from_iter(art_lis)
                    .intersection(&acc)
                    .map(|i| *i)
                    .collect()
            },
        );

        // With the intersection, we can now go through each article that "pass the test" of having the correct triples,
        // and actually linear search through them to find the correct answers
        let p: Vec<char> = query.chars().collect();

        let mut result: Vec<usize> = Vec::new();
        let (L_prime, l_prime, R) = boyer_moore_preprocess(&p);


        // TODO: improve generality of this section **************************************
        for art_no in art_intersect {
            // Read the file
            let t: Vec<char> =
                fs::read_to_string(format!("data/individual_articles/{:05}.txt", art_no))
                    .expect(
                        format!(
                            "Article number {} not found in data/individual_articles/",
                            art_no
                        )
                        .as_str(),
                    )
                    .chars()
                    .collect();
            match boyer_moore(&p, &t, (&L_prime, &l_prime, &R)) {
                x if x == Vec::<usize>::new() => (), // Empty vector
                _ => result.push(*art_no),           // There was at least one occurence
            }
        }
        // Result to article names
        result
            .iter()
            .map(|a_no| self.article_titles[*a_no].to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        helpers::Config,
        index::{Query, Search, SearchType},
    };

    use super::*;

    fn setup_real() -> Index<HashMap<(String, String, String), Vec<usize>>> {
        let config = Config::build(&[
            "".to_string(),
            "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),
            "11".to_string(),
        ])
        .unwrap();
        Index::index11(&config).unwrap()
    }

    fn setup_test() -> Index<HashMap<(String, String, String), Vec<usize>>> {
        let mut database: HashMap<(String, String, String), Vec<usize>> = HashMap::new();
        database.insert(
            (
                "word1".to_string(),
                "word2".to_string(),
                "word3".to_string(),
            ),
            vec![0],
        );
        database.insert(
            (
                "word2".to_string(),
                "word3".to_string(),
                "word4".to_string(),
            ),
            vec![0, 1, 2, 3, 4, 5, 6, 7],
        );
        database.insert(
            (
                "word3".to_string(),
                "word4".to_string(),
                "word5".to_string(),
            ),
            vec![0, 2, 4, 6],
        );
        database.insert(
            (
                "word4".to_string(),
                "word5".to_string(),
                "word6".to_string(),
            ),
            vec![1, 2, 3],
        );
        let mut article_titles: Vec<String> = Vec::new();
        for i in 0..100 {
            article_titles.push(format!("article {}", i).to_string());
        }
        Index {
            database,
            article_titles,
        }
    }

    fn search_match(
        index: Index<HashMap<(String, String, String), Vec<usize>>>,
        query: Query,
        expected: Vec<String>,
    ) {
        assert_eq!(
            HashSet::from_iter(index.search(&query)),
            HashSet::<String>::from_iter(expected)
        )
    }

    #[test]
    fn find_triple() {
        let index = setup_test();

        let query = Query {
            search_string: "word2 word3 word4".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };

        search_match(
            index,
            query,
            vec![
                "article 0".to_string(),
                "article 1".to_string(),
                "article 2".to_string(),
                "article 3".to_string(),
                "article 4".to_string(),
                "article 5".to_string(),
                "article 6".to_string(),
                "article 7".to_string(),
            ],
        );
    }

    #[test]
    fn the_empty_query() {
        let index = setup_test();

        let query = Query {
            search_string: "".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };
        let result = index.search(&query);

        assert_eq!(*result, Vec::<String>::new());

        let query = Query {
            search_string: "hej".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };
        let result = index.search(&query);

        assert_eq!(*result, Vec::<String>::new());

        let query = Query {
            search_string: "hej med".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };
        let result = index.search(&query);

        assert_eq!(*result, Vec::<String>::new());
    }

    #[test]
    fn query_not_found() {
        let index = setup_test();

        let query = Query {
            search_string: "word4 word5 word3".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };
        let result = index.search(&query);

        assert_eq!(*result, Vec::<String>::new());
    }

    #[test]
    fn more_than_three_words() {
        let index = setup_test();

        let query = Query {
            search_string: "word2 word3 word4 word5".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };

        search_match(
            index,
            query,
            vec![
                "article 0".to_string(),
                "article 2".to_string(),
                "article 4".to_string(),
                "article 6".to_string(),
            ],
        );
    }

    #[test]
    fn fuzzy_search_gives_the_wrong_answer_real() {
        let index = setup_real();

        let query = Query {
            search_string: "Sinope and the United".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };

        search_match(index, query, vec![]);
    }

    #[test]
    fn find_word_real_1() {
        let index = setup_real();

        let query = Query {
            search_string: "Etymology and terminology".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };

        search_match(index, query, vec!["Anarchism".to_string()]);
    }

    #[test]
    fn find_word_real_2() {
        let index = setup_real();

        let query = Query {
            search_string: "one of the".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };

        search_match(
            index,
            query,
            vec![
                "Autism".to_string(),
                "Albedo".to_string(),
                "Anarchism".to_string(),
            ],
        );
    }

    #[test]
    fn find_word_real_3() {
        let index = setup_real();

        let query = Query {
            search_string: "it can be".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };

        search_match(
            index,
            query,
            vec!["Albedo".to_string(), "Autism".to_string()],
        );
    }
    #[test]
    fn erroneous_query_finds_nothing_real() {
        let index = setup_real();

        let query = Query {
            search_string: "cantbefound cantbefound cantbefound".to_string(),
            search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
        };

        search_match(index, query, Vec::<String>::new());
    }
}
