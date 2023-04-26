use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;

use crate::helpers::*;
use crate::index::Index;

use super::*;

impl Index<HashMap<String, HashSet<String>>> {
    pub fn index6(config: &Config) -> Result<Self, Box<dyn Error>> {
        let mut database: HashMap<String, HashSet<String>> = HashMap::new();

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
    // use super::*;

    // #[test]
    // fn
}
