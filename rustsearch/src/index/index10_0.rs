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

    pub fn exact_search(&self, query: &String) -> Option<Vec<String>> {
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
        let mut result: Vec<(usize, Vec<usize>)> = vec![];
        let T = self.kmp_table(&query_words);

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
            result.push((
                *art_no, 
                self.kmp(file_contents, &query_words, &T)
            ));
        }
        // Result to article names
        // Some(vec!["boobies".to_string()]);
        todo!()
    }

    pub fn kmp_table(&self, query_words: &Vec<&str>) -> Vec<i32> {
        let mut T: Vec<i32> = vec![0; query_words.len()];
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

    pub fn kmp(&self, file_contents: String, query_words: &Vec<&str>, T: &Vec<i32>) -> Vec<usize> {
        // Output
        let mut P: Vec<usize> = vec![];

        // Local variables
        let mut j = 0;
        let mut k = 0;

        let file_vec: Vec<&str> = file_contents.split(' ').collect();
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
