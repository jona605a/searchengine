use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

use crate::helpers::*;
use crate::index::Index;
use crate::parsing::*;

pub struct Index8ExtraVariables {
    pub article_titles: Vec<String>,
}

impl Index<HashMap<String, Vec<usize>>, Index8ExtraVariables> {
    pub fn index8(
        config: &Config,
    ) -> Result<Index<HashMap<String, Vec<usize>>, Index8ExtraVariables>, Box<dyn Error>> {
        let mut database: HashMap<String, Vec<usize>> = HashMap::new();

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
                    let v = database.entry(word.to_string()).or_default();
                    if (v.len() == 0) || (v[v.len() - 1] != article_titles.len() - 1) {
                        v.push(article_titles.len() - 1)
                    }
                }
            }
        }

        Ok(Index {
            database,
            extra_variables: Some(Index8ExtraVariables { article_titles }),
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

    pub fn boolean_search_naive(&self, exp: &String) -> Option<Vec<String>> {
        match Expr::from_string(&exp) {
            Ok(Expr(ExprData::HasNodes(node))) => {
                Some(self.vec_to_articlelist(self.evaluate_syntex_tree_naive(node)))
            }
            _ => None, // Either an error or the expression has no nodes
        }
    }

    pub fn evaluate_syntex_tree_naive(&self, node: AstNode) -> Vec<usize> {
        match node {
            AstNode::Invert(child) => self.invert(self.evaluate_syntex_tree_naive(*child)),
            AstNode::Binary(BinaryOp::And, left_child, right_child) => self.and(
                self.evaluate_syntex_tree_naive(*left_child),
                self.evaluate_syntex_tree_naive(*right_child),
            ),
            AstNode::Binary(BinaryOp::Or, left_child, right_child) => self.or(
                self.evaluate_syntex_tree_naive(*left_child),
                self.evaluate_syntex_tree_naive(*right_child),
            ),
            AstNode::Name(word) => self.database.get(&word).unwrap_or(&vec![]).to_vec(),
        }
    }

    pub fn and(&self, left_child: Vec<usize>, right_child: Vec<usize>) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut l = 0;
        let mut r = 0;

        while (left_child.len() > l) & (right_child.len() > r) {
            if left_child[l] > right_child[r] {
                r = r + 1;
            } else if left_child[l] < right_child[r] {
                l = l + 1;
            } else {
                result.push(left_child[l]);
                l = l + 1;
                r = r + 1;
            }
        }

        result
    }

    pub fn or(&self, left_child: Vec<usize>, right_child: Vec<usize>) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut l = 0;
        let mut r = 0;

        while (left_child.len() > l) || (right_child.len() > r) {
            if l == left_child.len() {
                result.push(right_child[r]);
                r += 1;
            } else if r == right_child.len() {
                result.push(left_child[l]);
                l += 1;
            } else if left_child[l] > right_child[r] {
                result.push(right_child[r]);
                r += 1;
            } else if left_child[l] < right_child[r] {
                result.push(left_child[l]);
                l += 1;
            } else {
                result.push(left_child[l]);
                l += 1;
                r += 1;
            }
        }
        result
    }

    pub fn invert(&self, child: Vec<usize>) -> Vec<usize> {
        let mut result: Vec<usize> = Vec::new();
        let mut p: usize = 0;

        for i in 0..self.extra_variables.as_ref().unwrap().article_titles.len() {
            if (p >= child.len()) || (i < child[p]) {
                result.push(i)
            } else {
                p = p + 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::index::{self, gen_query::gen_a_lot_of_runs_bool};

    use super::*;
    use std::{collections::HashSet, fs};

    fn setup_real() -> Index<HashMap<String, Vec<usize>>, Index8ExtraVariables> {
        let config = Config::build(&[
            "".to_string(),
            "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),
            "8".to_string(),
        ])
        .unwrap();
        Index::index8(&config).unwrap()
    }

    fn setup_test() -> Index<HashMap<String, Vec<usize>>, Index8ExtraVariables> {
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
            extra_variables: Some(Index8ExtraVariables { article_titles }),
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

    fn search_match(
        index: &Index<HashMap<String, Vec<usize>>, Index8ExtraVariables>,
        query: &str,
        titles: Vec<&str>,
    ) {
        dbg!(&query.to_string());
        let index_result: HashSet<String> = HashSet::from_iter(
            index
                .boolean_search_naive(&query.to_string())
                .unwrap_or(Vec::default()),
        );
        assert_eq!(
            index_result,
            HashSet::from_iter(titles.iter().map(|s| s.to_string()))
        )
    }

    #[test]
    fn boolean_search_for_words_in_wiki100_kb() {
        let index = setup_real();

        search_match(
            &index,
            "the | autism",
            vec!["Anarchism", "Autism", "A", "Albedo"],
        );
        search_match(&index, "autism", vec!["Autism"]); // A word that should only be in one article
        search_match(&index, "bi-hemispherical", vec!["Albedo"]); // Check for no splitting of 'bi-hemispherical'
                                                                  // search_match(&index, "\"&amp;#65;\"", vec!["A"]); // A word that has special characters
    }

    #[test]
    fn find_a_word() {
        let index = setup_test();
        search_match(&index, "  word1 ", vec!["article 0"]);
    }

    #[test]
    fn ands_two_words() {
        let index = setup_test();
        search_match(&index, "word1 & word3", vec!["article 0"]);
    }

    #[test]
    fn or_two_words() {
        let index = setup_test();
        search_match(
            &index,
            "word1 | word4",
            vec!["article 0", "article 1", "article 2", "article 3"],
        );
    }

    #[test]
    fn or_and_and() {
        let index = setup_test();
        search_match(
            &index,
            "word1 | (word3 & word4)",
            vec!["article 0", "article 2"],
        );
    }

    #[test]
    fn or_with_word_not_in_database() {
        let index = setup_test();
        search_match(&index, "word1 | nowhere", vec!["article 0"]);
    }

    #[test]
    fn and_with_word_not_in_database() {
        let index = setup_test();
        search_match(&index, "word1 & nowhere", vec![]);
    }

    #[test]
    fn word_not_in_database() {
        let index = setup_test();
        search_match(&index, "nowhere", vec![]);
    }

    #[test]
    fn words_in_database_together_not_in_database() {
        let index = setup_test();
        search_match(&index, "word1 & word4", vec![]);
    }

    #[test]
    fn the_empty_query() {
        let index = setup_test();
        search_match(&index, "", vec![]);
    }

    #[test]
    fn erroneous_query_finds_nothing() {
        let index = setup_test();
        search_match(&index, "word1((", vec![]);
    }

    #[test]
    fn boolean_search_with_iversions() {
        let index = setup_real();

        search_match(&index, "!the", vec![]);
        search_match(&index, "!letter", vec!["Anarchism", "Albedo", "Autism"]);

        search_match(&index, "letter & !the", vec![]);
        search_match(
            &index,
            "!letter & the",
            vec!["Anarchism", "Albedo", "Autism"],
        );
        search_match(&index, "!letter & political", vec!["Anarchism"]);
        search_match(&index, "!letter & !political", vec!["Albedo", "Autism"]);
        search_match(&index, "!(letter or political)", vec!["Albedo", "Autism"]);

        search_match(&index, "letter or !the", vec!["A"]);
        search_match(
            &index,
            "!letter or the",
            vec!["A", "Anarchism", "Albedo", "Autism"],
        );
        search_match(
            &index,
            "!letter or political",
            vec!["Anarchism", "Albedo", "Autism"],
        );
        search_match(
            &index,
            "!letter or !political",
            vec!["A", "Anarchism", "Albedo", "Autism"],
        );
        search_match(
            &index,
            "!(letter and political)",
            vec!["A", "Anarchism", "Albedo", "Autism"],
        );
    }

    #[test]
    fn check_index7_and_index8_get_the_same_results() {
        let files = fs::read_dir("../../data.nosync/");

        for dir in files.unwrap() {
            let file = dir.unwrap().path().into_os_string().into_string().unwrap();
            let filesize = &file[46..file.len() - 4];

            if (filesize != "1MB") & (filesize != "2MB") {
                continue;
            }

            let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 10);

            let index7 = index::Index::index7(&Config {
                file_path: file.clone(),
                indexno: "7".to_string(),
            })
            .unwrap();
            let index8 = index::Index::index8(&Config {
                file_path: file.clone(),
                indexno: "8".to_string(),
            })
            .unwrap();

            for depth_vec in ast_vec {
                for ast in &depth_vec {
                    let article_list7_0 =
                        index7.bitvec_to_articlelist(index7.evaluate_syntax_tree(*ast.clone()));
                    let article_list8_0 =
                        index8.vec_to_articlelist(index8.evaluate_syntex_tree_naive(*ast.clone()));
                    let article_list8_1: Vec<String> = index8
                        .vec_to_articlelist(index8.evaluate_syntex_tree_demorgan(*ast.clone()));
                    let article_list8_2 = index8.vec_to_articlelist(
                        index8.evaluate_syntex_tree_binary_search(*ast.clone()),
                    );
                    let article_list8_3 =
                        index8.vec_to_articlelist(index8.evaluate_syntex_tree_hybrid(*ast.clone()));
                    let article_list8_4 = index8.bitvec_to_articlelist(
                        index8.evaluate_syntax_tree_convert_to_bitvecs(*ast.clone()),
                    );

                    assert_eq!(article_list7_0, article_list8_0);
                    assert_eq!(article_list8_1, article_list8_2);
                    assert_eq!(article_list8_3, article_list8_4);
                    assert_eq!(article_list7_0, article_list8_1);
                    assert_eq!(article_list7_0, article_list8_3);
                }
            }
        }
    }
}
