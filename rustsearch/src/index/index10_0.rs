#![allow(non_snake_case)]
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

use crate::helpers::*;
use crate::index::Index;

#[derive(Debug)]
pub struct Index10ExtraVariables {
    pub article_titles: Vec<String>,
}

#[allow(dead_code)]
impl Index<HashMap<String, HashMap<usize, usize>>, Index10ExtraVariables> {
    pub fn index10(
        config: &Config,
    ) -> Result<Index<HashMap<String, HashMap<usize, usize>>, Index10ExtraVariables>, Box<dyn Error>>
    {
        // Setup
        let mut database: HashMap<String, HashMap<usize, usize>> = HashMap::new();

        let filecontents = read_file_to_string(&config.file_path)?;
        let re = Regex::new(r"\. |\.\n|\n\n|; |[\[\]\{\}\\\n\(\) ,:/=?!*]").unwrap();
        let articles_iter = filecontents.split("---END.OF.DOCUMENT---").map(|a| {
            let (title, contents) = a.trim().split_once(".\n").unwrap_or(("", ""));
            (title.to_string(), re.split(contents))
        });
        let mut article_titles: Vec<String> = Vec::new();
        let mut article_no = 0;

        // The actual indexing
        for (title, contents) in articles_iter {
            if title != "" {
                article_titles.push(title.to_string());
                for word in contents {
                    let article_freq = database.entry(word.to_string()).or_default();
                    *article_freq.entry(article_no).or_default() += 1;
                }
            }
            article_no += 1;
        }

        Ok(Index {
            database,
            extra_variables: Some(Index10ExtraVariables { article_titles }),
        })
    }

    pub fn exact_search(&self, query: &String) -> HashMap<usize,Vec<usize>> {
        // Split sentence into words
        // Get article set for each word, and find intersection
        let art_intersect = query
            .split(' ')
            .map(|w| match self.database.get(w) {
                Some(art_map) => HashSet::from_iter(art_map.keys()),
                None => HashSet::new(),
            })
            .fold(HashSet::new(), |acc, art_set| {
                art_set.intersection(&acc).map(|i| *i).collect()
            });

        // For each article in the intersection, identify the least frequent word and read through the article (linearly) to find all sentence matches
        let query_words: Vec<&str> = query.split(' ').collect();
        let mut result: HashMap<usize,Vec<usize>> = HashMap::new();
        let T = Index::kmp_table(&query_words);

        for art_no in art_intersect.iter().map(|i| *i) {
            // Read the file
            let file_contents =
                fs::read_to_string(format!("data/individual_articles/{:05}.txt", art_no)).expect(
                    format!(
                        "Article number {} not found in data/individual_articles/",
                        art_no
                    )
                    .as_str(),
                );
            let match_pos = Index::kmp(file_contents, &query_words, &T);
            result.insert(*art_no, match_pos);
        }
        // Result to article names
        result
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

    pub fn kmp(file_contents: String, query_words: &Vec<&str>, T: &Vec<i32>) -> Vec<usize> {
        // Output
        let mut P: Vec<usize> = vec![];

        // Local variables
        let mut j = 0;
        let mut k = 0;

        let file_vec: Vec<&str> = file_contents.split_ascii_whitespace().collect();

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
                        k += 1
                    }
                    x => k = x as usize,
                }
            }
        }
        P
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn kmp_table_1() {
        let w: Vec<&str> = vec!["A", "B", "C", "D", "A", "B", "D"];
        let kmp_table = Index::kmp_table(&w);
        assert_eq!(kmp_table, vec![-1, 0, 0, 0, -1, 0, 2, 0]);
    }

    #[test]
    fn kmp_table_2() {
        let w: Vec<&str> = vec!["A", "B", "A", "C", "A", "B", "A", "B", "C"];
        let kmp_table = Index::kmp_table(&w);
        assert_eq!(kmp_table, vec![-1, 0, -1, 1, -1, 0, -1, 3, 2, 0]);
    }

    #[test]
    fn kmp_table_3() {
        let w: Vec<&str> = vec!["A", "B", "A", "C", "A", "B", "A", "B", "A"];
        let kmp_table = Index::kmp_table(&w);
        assert_eq!(kmp_table, vec![-1, 0, -1, 1, -1, 0, -1, 3, -1, 3]);
    }

    #[test]
    fn kmp_table_4() {
        let w: Vec<&str> = vec![
            "P", "A", "R", "T", "I", "C", "I", "P", "A", "T", "E", " ", "I", "N", " ", "P", "A",
            "R", "A", "C", "H", "U", "T", "E",
        ];
        let kmp_table = Index::kmp_table(&w);
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
        let kmp_table = Index::kmp_table(&w);
        assert_eq!(kmp_table, vec![-1, 0, 0, 0, -1, 0, 2, 0]);
    }

    #[test]
    fn kmp_1() {
        let file_contents: String = "A".to_string();
        let query_words: Vec<&str> = vec!["A"];
        let T: Vec<i32> = Index::kmp_table(&query_words);

        assert_eq!(Index::kmp(file_contents, &query_words, &T), vec![0]);
    }

    #[test]
    fn kmp_2() {
        let file_contents: String = "A B AB A B C A B".to_string();
        let query_words: Vec<&str> = vec!["A", "B"];
        let T: Vec<i32> = Index::kmp_table(&query_words);

        assert_eq!(Index::kmp(file_contents, &query_words, &T), vec![0, 3, 6]);
    }

    #[test]
    fn kmp_3() {
        let file_contents: String =
            "When I find myself in times of trouble, Mother Mary comes to me Speaking words of wisdom, 
            let it be And in my hour of darkness she is standing right in front of me Speaking words of wisdom, let it be Let it be  let it be let it be let it be Whisper words of wisdom, let it be And when the broken hearted people living in the world agree There will be an answer, let it be For though they may be parted, there is still a chance that they will see There will be an answer, let it be Let it be let it be let it be let it be There will be an answer, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be Let it be let it be let it be let it be Whisper words of wisdom, let it be be And when the night is cloudy there is still a light that shines on me Shinin' until tomorrow, let it be I wake up to the sound of music, Mother Mary comes to me Speaking words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom, let it be And let it be let it be let it be let it be Whisper words of wisdom, let it be"
                .to_string()
                .to_ascii_lowercase();
        let query_words: Vec<&str> = vec!["let", "it", "be"];
        let T: Vec<i32> = Index::kmp_table(&query_words);

        assert_eq!(Index::kmp(file_contents, &query_words, &T), vec![17, 38, 41, 44, 47, 50, 57, 76, 99, 102, 105, 108, 111, 119, 122, 125, 128, 131, 138, 141, 144, 147, 150, 157, 179, 199, 203, 206, 209, 212, 219, 223, 226, 229, 232, 239]);
    }
}
