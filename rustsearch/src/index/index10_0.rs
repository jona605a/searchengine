use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::ops::Deref;

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
        for art_no in art_intersect.iter().map(|i| *i) {
            let least_frequent_word = *query_words
                .iter()
                .map(|w| (w, self.database.get(&w.to_string()).unwrap()))
                .min_by(|x, y| x.1.get(art_no).unwrap().cmp(y.1.get(art_no).unwrap()))
                .unwrap()
                .0;
            
        }

        Some(vec!["boobies".to_string()])
    }
}
