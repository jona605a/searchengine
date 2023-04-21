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
        let mut result: Vec<usize> = vec![];
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
            if self.kmp(file_contents, &query_words,&T) {
                result.push(*art_no);
            }
        }
        // Result to article names
        Some(vec!["boobies".to_string()])
    }

    #[allow(non_snake_case)]
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

    pub fn kmp(&self, file_contents: String, query_words: &Vec<&str>, T: &Vec<i32>) -> bool {
        // Output
        let mut P: Vec<usize> = vec![];
        let mut nP = 0;

        // Local variables
        let mut j = 0;
        let mut k = 0;

        let file_vec: Vec<&str> = file_contents.split(' ').collect();
        while j < file_vec.len() {
            if query_words[k] == file_vec[j] {
                j += 1; k+=1;
                if k == query_words.len() {
                    // Occurence found
                    P.push(j-k);
                    nP += 1;
                    k = T[k] as usize;
                }
            } else {
                match T[k] {
                    -1 => {
                        j += 1; k+=1
                    },
                    x => k = x as usize
                }
            }
        }



        // algorithm kmp_search:
        // input:
        //     an array of characters, S (the text to be searched)
        //     an array of characters, W (the word sought)
        // output:
        //     an array of integers, P (positions in S at which W is found)
        //     an integer, nP (number of positions)

        // define variables:
        //     an integer, j ← 0 (the position of the current character in S)
        //     an integer, k ← 0 (the position of the current character in W)
        //     an array of integers, T (the table, computed elsewhere)

        // let nP ← 0

        // while j < length(S) do
        //     if W[k] = S[j] then
        //         let j ← j + 1
        //         let k ← k + 1
        //         if k = length(W) then
        //             (occurrence found, if only first occurrence is needed, m ← j - k  may be returned here)
        //             let P[nP] ← j - k, nP ← nP + 1
        //             let k ← T[k] (T[length(W)] can't be -1)
        //     else
        //         let k ← T[k]
        //         if k < 0 then
        //             let j ← j + 1
        //             let k ← k + 1

        todo!()
    }

    // pub fn linear_search_in_file_for_string_given_search_word(
    //     &self,
    //     file_contents: String,
    //     query_words: &Vec<&str>,
    //     least_frequent_word: &str,
    // ) -> bool {
    //     let lfw_idx = query_words
    //         .iter()
    //         .position(|&w| w == least_frequent_word)
    //         .unwrap();
    //     let mut prev_k_words = vec![""; lfw_idx];
    //     let mut contents = file_contents.split(' ');
    //     for i in 0..lfw_idx {
    //         prev_k_words[i] = contents.next().unwrap();
    //     }
    //     let mut i = 0;
    //     let mut prev_match = false;
    //     let mut p = 1;
    //     for word in contents {
    //         if prev_match {
    //             if p > query_words.len() - lfw_idx - 1 {
    //                 // We're done!
    //                 return true;
    //             }
    //             if word == query_words[lfw_idx + p] {
    //                 p += 1;
    //             } else {
    //                 prev_match = false;
    //             }
    //         } else if word == least_frequent_word {
    //             // Match with prev_k_words
    //             prev_match = true;
    //             p = 1;
    //             for j in 1..=lfw_idx {
    //                 if prev_k_words[(i - j) % lfw_idx] != query_words[lfw_idx - j] {
    //                     prev_match = false;
    //                     break;
    //                 }
    //             }
    //         }
    //         prev_k_words[i] = word;
    //         i = (i + 1) % lfw_idx;
    //     }
    //     todo!()
    // }
}
