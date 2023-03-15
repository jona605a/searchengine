use std::collections::HashMap;

use crate::index::index8_0::Index8ExtraVariables;
use crate::index::Index;
use crate::parsing::*;

#[allow(dead_code)]
impl Index<HashMap<String, Vec<usize>>, Index8ExtraVariables> {
    pub fn boolean_search_demorgan(&self, exp: &String) -> Option<Vec<String>> {
        match Expr::from_string(&exp) {
            Ok(Expr(ExprData::HasNodes(node))) => {
                self.vec_to_articleset(self.evaluate_syntex_tree_demorgan(node))
            }
            _ => None, // Either an error or the expression has no nodes
        }
    }

    pub fn evaluate_syntex_tree_demorgan(&self, node: AstNode) -> Vec<usize> {
        match node {
            AstNode::Invert(child) => self.invert(self.evaluate_syntex_tree_demorgan(*child)),
            AstNode::Binary(BinaryOp::And, left_child, right_child) => {
                match (*left_child, *right_child) {
                    (AstNode::Invert(left_child), AstNode::Invert(right_child)) => {
                        self.invert(self.or(
                            self.evaluate_syntex_tree_demorgan(*left_child),
                            self.evaluate_syntex_tree_demorgan(*right_child),
                        ))
                    }
                    (left_child, right_child) => self.and(
                        self.evaluate_syntex_tree_demorgan(left_child),
                        self.evaluate_syntex_tree_demorgan(right_child),
                    ),
                }
            }
            AstNode::Binary(BinaryOp::Or, left_child, right_child) => {
                match (*left_child, *right_child) {
                    (AstNode::Invert(left_child), AstNode::Invert(right_child)) => {
                        self.invert(self.and(
                            self.evaluate_syntex_tree_demorgan(*left_child),
                            self.evaluate_syntex_tree_demorgan(*right_child),
                        ))
                    }
                    (left_child, right_child) => self.or(
                        self.evaluate_syntex_tree_demorgan(left_child),
                        self.evaluate_syntex_tree_demorgan(right_child),
                    ),
                }
            }
            AstNode::Name(word) => self.database.get(&word).unwrap_or(&vec![]).to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn setup_real() -> Index<HashMap<String, Vec<usize>>, Index8ExtraVariables> {
        let config = crate::helpers::Config::build(&[
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
    fn bitvec_to_articleset_works() {
        let test_index = setup_test();

        let bitvec: Vec<usize> = vec![0, 1];

        let hs = vec!["article 0".to_string(), "article 1".to_string()];
        assert_eq!(test_index.vec_to_articleset(bitvec).unwrap(), hs)
    }

    #[should_panic]
    #[test]
    fn bitvec_to_articleset_panics_when_out_of_range() {
        let test_index = setup_test();

        let bitvec: Vec<usize> = vec![100000000];

        test_index.vec_to_articleset(bitvec).unwrap();
    }

    fn search_match(
        index: &Index<HashMap<String, Vec<usize>>, Index8ExtraVariables>,
        query: &str,
        titles: Vec<&str>,
    ) {
        dbg!(&query.to_string());
        let index_result: HashSet<String> = HashSet::from_iter(
            index
                .boolean_search_demorgan(&query.to_string())
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
}
