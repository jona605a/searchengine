use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

use crate::helpers::*;
use crate::index::Index;

#[derive(Debug)]
pub struct Index11ExtraVariables {
    pub article_titles: Vec<String>,
}

#[allow(dead_code)]
impl Index<HashMap<(String, String, String), Vec<usize>>, Index11ExtraVariables> {
    pub fn index11(
        config: &Config,
    ) -> Result<
        Index<HashMap<(String, String, String), Vec<usize>>, Index11ExtraVariables>,
        Box<dyn Error>,
    > {
        let mut database: HashMap<(String, String, String), Vec<usize>> = HashMap::new();

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

        for (title, mut contents) in articles_iter {
            if title != "" {
                article_titles.push(title.to_string());

                let mut prv1 = contents.next().unwrap();
                let mut prv2 = contents.next().unwrap();

                for word in contents {
                    let v = database
                        .entry((prv1.to_string(), prv2.to_string(), word.to_string()))
                        .or_default();
                    if (v.len() == 0) || (v[v.len() - 1] != article_titles.len() - 1) {
                        v.push(article_titles.len() - 1);
                    }

                    prv1 = prv2;
                    prv2 = word;
                }
            }
        }

        Ok(Index {
            database,
            extra_variables: Some(Index11ExtraVariables { article_titles }),
        })
    }

    pub fn vec_to_articlelist(&self, vec: Vec<usize>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let titles = &self.extra_variables.as_ref().unwrap().article_titles;
        for i in vec {
            output.push(titles[i].clone());
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_real() -> Index<HashMap<(String, String, String), Vec<usize>>, Index11ExtraVariables> {
        let config = Config::build(&[
            "".to_string(),
            "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),
            "11".to_string(),
        ])
        .unwrap();
        Index::index11(&config).unwrap()
    }

    fn setup_test() -> Index<HashMap<(String, String, String), Vec<usize>>, Index11ExtraVariables> {
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
            extra_variables: Some(Index11ExtraVariables { article_titles }),
        }
    }

    #[test]
    fn vec_to_articlelist_works() {
        let test_index = setup_test();

        let vec: Vec<usize> = vec![0, 1];

        let hs = vec!["article 0".to_string(), "article 1".to_string()];
        assert_eq!(test_index.vec_to_articlelist(vec), hs)
    }

    #[test]
    fn find_word() {
        let index = setup_test();
        let result = index
            .database
            .get(&(
                "word2".to_string(),
                "word3".to_string(),
                "word4".to_string(),
            ))
            .unwrap();
        assert_eq!(*result, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn the_empty_query() {
        let index = setup_test();
        let result = index
            .database
            .get(&("".to_string(), "".to_string(), "".to_string()));
        assert_eq!(result, None);
    }

    #[test]
    fn erroneous_query_finds_nothing() {
        let index = setup_test();
        let result = index.database.get(&(
            "word4".to_string(),
            "word5".to_string(),
            "word3".to_string(),
        ));
        assert_eq!(result, None);
    }

    #[test]
    fn find_word_real_1() {
        let index = setup_real();
        let result = index
            .database
            .get(&(
                "Etymology".to_string(),
                "and".to_string(),
                "terminology".to_string(),
            ))
            .unwrap();
        assert_eq!(*result, vec![0]);
    }

    #[test]
    fn find_word_real_2() {
        let index = setup_real();
        let result = index
            .database
            .get(&("one".to_string(), "of".to_string(), "the".to_string()))
            .unwrap();
        assert_eq!(*result, vec![0, 1, 2]);
    }

    #[test]
    fn find_word_real_3() {
        let index = setup_real();
        let result = index
            .database
            .get(&("it".to_string(), "can".to_string(), "be".to_string()))
            .unwrap();
        assert_eq!(*result, vec![1, 2]);
    }
    #[test]
    fn erroneous_query_finds_nothing_real() {
        let index = setup_real();
        let result = index.database.get(&(
            "cantbefound".to_string(),
            "cantbefound".to_string(),
            "cantbefound".to_string(),
        ));
        assert_eq!(result, None);
    }
}
