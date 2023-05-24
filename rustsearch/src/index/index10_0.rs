#![allow(non_snake_case)]
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use crate::helpers::*;
use crate::index::Index;

use super::{ArticleTitles, Query, Search, SearchType};

pub fn kmp_table_chars(query: &String) -> Vec<i32> {
    let query: Vec<char> = query.chars().collect();
    let mut T: Vec<i32> = vec![0; query.len() + 1];
    let mut pos: usize = 1;
    let mut cnd: i32 = 0;
    T[0] = -1;

    while pos < query.len() {
        if query[pos] == query[cnd as usize] {
            T[pos] = T[cnd as usize]
        } else {
            T[pos] = cnd;
            while cnd >= 0 && query[pos] != query[cnd as usize] {
                cnd = T[cnd as usize];
            }
        }
        pos += 1;
        cnd += 1;
    }
    T[pos] = cnd;
    T
}

pub fn kmp_table(query_words: &Vec<&str>) -> Vec<i32> {
    let mut T: Vec<i32> = vec![0; query_words.len() + 1];
    let mut pos: usize = 1;
    let mut cnd: i32 = 0;
    T[0] = -1;

    while pos < query_words.len() {
        if query_words[pos] == query_words[cnd as usize] {
            T[pos] = T[cnd as usize]
        } else {
            T[pos] = cnd;
            while cnd >= 0 && query_words[pos] != query_words[cnd as usize] {
                cnd = T[cnd as usize];
            }
        }
        pos += 1;
        cnd += 1;
    }
    T[pos] = cnd;
    T
}

pub fn kmp_truefalse(file_contents: String, query: &String, T: &Vec<i32>) -> Option<usize> {
    // let mut P: Vec<usize> = Vec::new();
    let mut j = 0;
    let mut k = 0;

    let file_vec: Vec<char> = file_contents.chars().collect();
    let p: Vec<char> = query.chars().collect();

    // dbg!(&T, &file_contents, &query_words);
    // let mut counter = 0;

    while j < file_vec.len() {
        // counter += 1;
        if p[k] == file_vec[j] {
            j += 1;
            k += 1;
            if k == p.len() {
                // Occurence found
                // println!("KMP ran {counter} iterations");
                return Some(j - k);
            }
        } else {
            match T[k] {
                -1 => {
                    j += 1;
                    k = 0
                }
                x => k = x as usize,
            }
        }
    }
    // println!("KMP ran {counter} iterations");
    return None;
}

pub fn kmp_allmatches(file_contents: String, query_words: &Vec<&str>, T: &Vec<i32>) -> Vec<usize> {
    let mut P: Vec<usize> = Vec::new();
    let mut j = 0;
    let mut k = 0;

    let file_vec: Vec<&str> = file_contents.split_ascii_whitespace().collect();

    // dbg!(&T, &file_contents, &query_words);

    while j < file_vec.len() {
        if query_words[k] == file_vec[j] {
            j += 1;
            k += 1;
            if k == query_words.len() {
                // Occurence found
                P.push(j - k);
                k = T[k] as usize;
            }
        } else {
            match T[k] {
                -1 => {
                    j += 1;
                    k = 0
                }
                x => k = x as usize,
            }
        }
    }
    P
}

impl Index<HashMap<String, HashSet<usize>>> {
    pub fn index10(config: &Config) -> Result<Self, Box<dyn Error>> {
        // Read all articles, run them through the regex and write each individual article to a file named in the format 0000xxxx.txt
        write_article_files(config);

        // Setup
        let mut database: HashMap<String, HashSet<usize>> = HashMap::new();

        let articles_iter = read_and_clean_file_to_iter(config)?;
        let mut article_titles: Vec<String> = Vec::new();
        let mut article_no = 0;

        // The actual indexing
        for (title, contents) in articles_iter {
            if title != "" {
                article_titles.push(title.to_string());
                for word in contents {
                    let articles = database.entry(word.to_string()).or_default();
                    articles.insert(article_no);
                }
            }
            article_no += 1;
        }

        Ok(Index {
            database,
            article_titles,
        })
    }

    pub fn single_search(&self, query: &String) -> ArticleTitles {
        let article_set = self
            .database
            .get(query)
            .unwrap_or(&HashSet::new())
            .to_owned();
        article_set
            .iter()
            .map(|a_no| self.article_titles[*a_no].to_owned())
            .collect()
    }

    pub fn kmp_search(&self, query: &String) -> ArticleTitles {
        // Split sentence into words
        // Get article set for each word, and find intersection
        let mut x = query
            .split(' ')
            .map(|w| self.database.get(w).unwrap_or(&HashSet::new()).to_owned());
        let keys = x.next().unwrap();
        let art_intersect: Vec<usize> = keys
            .into_iter()
            .filter(|ar_no| x.all(|hs_a| hs_a.contains(ar_no)))
            .collect();

        // let query_words: Vec<&str> = query.split(' ').collect();
        let mut result: Vec<usize> = Vec::new();
        // Do the KMP preprocessing
        let T = kmp_table_chars(&query);
        // dbg!(&art_intersect);

        for art_no in art_intersect {
            // Read the file
            let file_contents =
                fs::read_to_string(format!("data/individual_articles/{:08}.txt", art_no)).expect(
                    format!(
                        "Article number {} not found in data/individual_articles/",
                        art_no
                    )
                    .as_str(),
                );
            match kmp_truefalse(file_contents, &query, &T) {
                None => (),                     // No occurence
                Some(_) => result.push(art_no), // There was at least one occurence
            }
        }
        // Result to article names
        result
            .iter()
            .map(|a_no| self.article_titles[*a_no].to_owned())
            .collect()
    }

    pub fn dumidesearch(&self, query: &String) -> ArticleTitles {
        let mut x = query
            .split(' ')
            .map(|w| self.database.get(w).unwrap_or(&HashSet::new()).to_owned());
        let keys = x.next().unwrap();
        let art_intersect: Vec<usize> = keys
            .into_iter()
            .filter(|ar_no| x.all(|hs_a| hs_a.contains(ar_no)))
            .collect();
        let mut result: Vec<usize> = Vec::new();
        for art_no in art_intersect {
            // Read the file
            let file_contents =
                fs::read_to_string(format!("data/individual_articles/{:08}.txt", art_no)).expect(
                    format!(
                        "Article number {} not found in data/individual_articles/",
                        art_no
                    )
                    .as_str(),
                );
            // Ladies and gentlemen, behold the power of the .contains function
            if file_contents.contains(&query[..]) {
                result.push(art_no)
            }
        }
        // Result to article names
        result
            .iter()
            .map(|a_no| self.article_titles[*a_no].to_owned())
            .collect()
    }
}

impl Search for Index<HashMap<String, HashSet<usize>>> {
    fn search(&self, query: &Query) -> ArticleTitles {
        match &query.search_type {
            SearchType::SingleWordSearch => self.single_search(&query.search_string),
            SearchType::ExactSearch(x) if x == "KMP" => self.kmp_search(&query.search_string),
            SearchType::ExactSearch(x) if x == "BoyerMoore" => {
                self.boyer_moore_search(&query.search_string)
            },
            SearchType::ExactSearch(x) if x == "dumide" => {
                self.dumidesearch(&query.search_string)
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kmp_table_1() {
        let w: Vec<&str> = vec!["A", "B", "C", "D", "A", "B", "D"];
        let kmp_table = kmp_table(&w);
        assert_eq!(kmp_table, vec![-1, 0, 0, 0, -1, 0, 2, 0]);
    }

    #[test]
    fn kmp_table_2() {
        let w: Vec<&str> = vec!["A", "B", "A", "C", "A", "B", "A", "B", "C"];
        let kmp_table = kmp_table(&w);
        assert_eq!(kmp_table, vec![-1, 0, -1, 1, -1, 0, -1, 3, 2, 0]);
    }

    #[test]
    fn kmp_table_3() {
        let w: Vec<&str> = vec!["A", "B", "A", "C", "A", "B", "A", "B", "A"];
        let kmp_table = kmp_table(&w);
        assert_eq!(kmp_table, vec![-1, 0, -1, 1, -1, 0, -1, 3, -1, 3]);
    }

    #[test]
    fn kmp_table_4() {
        let w: Vec<&str> = vec![
            "P", "A", "R", "T", "I", "C", "I", "P", "A", "T", "E", " ", "I", "N", " ", "P", "A",
            "R", "A", "C", "H", "U", "T", "E",
        ];
        let kmp_table = kmp_table(&w);
        assert_eq!(
            kmp_table,
            vec![-1, 0, 0, 0, 0, 0, 0, -1, 0, 2, 0, 0, 0, 0, 0, -1, 0, 0, 3, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn kmp_table_5() {
        let w: Vec<&str> = vec![
            "WORDA", "WORDB", "WORDC", "WORDD", "WORDA", "WORDB", "WORDD",
        ];
        let kmp_table = kmp_table(&w);
        assert_eq!(kmp_table, vec![-1, 0, 0, 0, -1, 0, 2, 0]);
    }

    #[test]
    fn kmp_1() {
        let file_contents: String = "A".to_string();
        let query_words: Vec<&str> = vec!["A"];
        let T: Vec<i32> = kmp_table(&query_words);

        assert_eq!(kmp_allmatches(file_contents, &query_words, &T), vec![0]);
    }

    #[test]
    fn kmp_2() {
        let file_contents: String = "A B AB A B C A B".to_string();
        let query_words: Vec<&str> = vec!["A", "B"];
        let T: Vec<i32> = kmp_table(&query_words);

        assert_eq!(kmp_allmatches(file_contents, &query_words, &T), vec![0,3,6]);
    }

    #[test]
    fn kmp_3() {
        let file_contents: String =
            "When I find myself in times of trouble, Mother Mary comes to me Speaking words of wisdom, 
            let it be And in my hour of darkness she is standing right in front of me Speaking words of wisdom, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be And when the broken hearted people living in the world agree There will be an answer, let it be For though they may be parted, there is still a chance that they will see There will be an answer, let it be Let it be let it be let it be let it be There will be an answer, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be be And when the night is cloudy there is still a light that shines on me Shinin' until tomorrow, let it be I wake up to the sound of music, Mother Mary comes to me Speaking words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom, let it be"
                .to_string()
                .to_ascii_lowercase();
        let query_words: Vec<&str> = vec!["let", "it", "be"];
        let T: Vec<i32> = kmp_table(&query_words);

        assert_eq!(
            kmp_allmatches(file_contents, &query_words, &T),
            vec![
                17, 38, 41, 44, 47, 50, 57, 76, 99, 102, 105, 108, 111, 119, 122, 125, 128, 131,
                138, 141, 144, 147, 150, 157, 179, 199, 203, 206, 209, 212, 219, 223, 226, 229,
                232, 239
            ]
        );
    }

    // ==================== Test the actual index ====================
    fn setup_test() -> Index<HashMap<String, HashSet<usize>>> {
        {
            let mut database: HashMap<String, HashSet<usize>> = HashMap::new();

            database.insert(
                "word1".to_string(),
                HashSet::from_iter(vec![100, 101, 102, 103]),
            );
            database.insert(
                "word2".to_string(),
                HashSet::from_iter(vec![100, 101, 102, 103, 104, 105]),
            );
            database.insert(
                "word3".to_string(),
                HashSet::from_iter(vec![100, 101, 102, 103, 104, 105]),
            );
            database.insert(
                "word4".to_string(),
                HashSet::from_iter(vec![100, 101, 102, 103, 104, 105]),
            );

            let mut article_titles: Vec<String> = Vec::new();
            for i in 0..110 {
                article_titles.push(format!("article {}", i).to_string());
            }

            // Write the actual files
            fs::write(
                format!("data/individual_articles/{:08}.txt", 100),
                "word1 word1 word2 word2 word3 word3 word4 word4",
            )
            .unwrap();
            fs::write(
                format!("data/individual_articles/{:08}.txt", 101),
                "word1 word2 word3 word4 word4 word3 word2 word1",
            )
            .unwrap();
            fs::write(
                format!("data/individual_articles/{:08}.txt", 102),
                "word2 word4 word1 word3",
            )
            .unwrap();
            fs::write(
                format!("data/individual_articles/{:08}.txt", 103),
                "word1word2word3word4 word1 word4 word1 word2 word1 word3 word1",
            )
            .unwrap();
            fs::write(
                format!("data/individual_articles/{:08}.txt", 104),
                "word3 word3 word2 word3 word3 word3 word2 word3 word2 word2 word2 word3 word3 word3 word3 word3 word2 word3 word3 word3 word2",
            )
            .unwrap();
            fs::write(
                format!("data/individual_articles/{:08}.txt", 105),
                "word2 word3 word4",
            )
            .unwrap();

            Index {
                database,
                article_titles,
            }
        }
    }

    fn search_match(
        index: Index<HashMap<String, HashSet<usize>>>,
        query: Query,
        expected: Vec<String>,
    ) {
        assert_eq!(
            HashSet::from_iter(index.search(&query)),
            HashSet::<String>::from_iter(expected)
        )
    }

    #[test]
    fn find_a_word() {
        let index = setup_test();
        let query = Query {
            search_string: "word1 word2 word3 word4".to_string(),
            search_type: SearchType::ExactSearch("KMP".to_string()),
        };

        search_match(index, query, vec!["article 101".to_string()])
    }

    #[test]
    fn find_a_word2() {
        let index = setup_test();
        let query = Query {
            search_string: "word1 word2 word3 word4".to_string(),
            search_type: SearchType::ExactSearch("KMP".to_string()),
        };

        search_match(index, query, vec!["article 101".to_string()])
    }

    #[test]
    fn find_a_word3() {
        let index = setup_test();
        let query = Query {
            search_string: "word1 word2 word1 word3 word1".to_string(),
            search_type: SearchType::ExactSearch("KMP".to_string()),
        };

        search_match(index, query, vec!["article 103".to_string()])
    }

    #[test]
    fn no_words_found() {
        let index = setup_test();
        let query = Query {
            search_string: "cantbefound".to_string(),
            search_type: SearchType::ExactSearch("KMP".to_string()),
        };

        search_match(index, query, vec![])
    }

    #[test]
    #[ignore]
    fn measure_triples_in_100mb() {
        let config = Config::build(&[
            "".to_string(),
            "data/WestburyLab.wikicorp.201004_100MB.txt".to_string(),
            "10_0".to_string(),
        ])
        .unwrap();
        let index = Index::index10(&config).unwrap();
        dbg!(index.database.len());
    }
}
