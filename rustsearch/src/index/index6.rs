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
    // use super::*;

    // #[test]
    // fn
}
