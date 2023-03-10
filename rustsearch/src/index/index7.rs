use std::collections::HashMap;
use std::error::Error;
use regex::Regex;

use crate::index::Index;
use crate::helpers::*;
use crate::parsing::*;

#[derive(Debug)]
pub struct Index7ExtraVariables {
    article_titles: Vec<String>,
}


#[allow(dead_code)]
impl Index<HashMap<String,Vec<u64>>,Index7ExtraVariables> {

    pub fn index7(config: &Config) -> Result<Index<HashMap<String,Vec<u64>>,Index7ExtraVariables>, Box<dyn Error>> {
        let mut database: HashMap<String,Vec<u64>> = HashMap::new();
        
        let filecontents = read_file_to_string(&config.file_path)?;
        let re = Regex::new(r"\. |\.\n|\n\n|; |[\[\]\{\}\\\n\(\) ,:/=?!*]").unwrap();

        let articles_iter = filecontents.split("---END.OF.DOCUMENT---")
            .map(|a| {
                let (title, contents) = a.trim().split_once(".\n").unwrap_or(("empty title",""));
                // dbg!(title, contents);
                (title.to_string(), re.split(contents))
            });
        let mut article_titles: Vec<String> = Vec::new();
        
        let mut n_titles = 0;
        let mut v_len = 1;

        for (title, contents) in articles_iter {
            if title == ""{
                ()
            }
            else{
                article_titles.push(title.to_string());
                n_titles += 1;
                if n_titles > v_len*64 {
                    // Extend the length of all vectors by 1
                    for v in database.values_mut() {
                        v.push(0);
                    }
                    v_len += 1;
                }
                for word in contents {
                    let v = database.entry(word.to_string()).or_default();
                    while v.len() < v_len {v.push(0)}
                    let title_bit: u64 = 1 << ((n_titles-1)%64);
                    v[(n_titles-1)/64] = v[(n_titles-1)/64] | title_bit;
                }
            }
        }

        Ok(Index {
            database, 
            extra_variables: Some(Index7ExtraVariables{article_titles})
        })
    }

    pub fn bitvec_to_articleset(&self, bitvecs: Vec<u64>) -> Option<Vec<String>> {
        let mut output: Vec<String> = Vec::new();
        let titles = &self.extra_variables.as_ref().unwrap().article_titles;
        for i in 0..bitvecs.len() {
            for bit in 0..64 {
                if (1<<(bit)) & bitvecs[i] > 0 {
                    if titles.len() <= i*64+bit {
                        continue;
                        panic!("Error, looked-up word refers to an article with a larger index ({}) than there are titles: {}. Bitvec len: {}, bit {}, bitvec: {:b},",i*64+bit, titles.len(), bitvecs.len(),bit, bitvecs[i])
                    }
                    output.push(titles[i*64+bit].clone());
                }
            }
        }
        Some(output)
    }

    pub fn boolean_search(&self, exp: &String) -> Option<Vec<String>>{
        match Expr::from_string(&exp) {
            Ok(Expr(ExprData::HasNodes(node))) => self.bitvec_to_articleset(self.evaluate_syntax_tree(node)),
            _ => None // Either an error or the expression has no nodes
        }
    }

    pub fn evaluate_syntax_tree(&self, node: AstNode)-> Vec<u64> {
        match node{
            AstNode::Invert(child) => self.evaluate_syntax_tree(*child).iter().map(|bv| !bv).collect() ,
            AstNode::Binary(BinaryOp::And,left_child,right_child) => self.evaluate_syntax_tree(*left_child).iter()
                                                                                                    .zip(self.evaluate_syntax_tree(*right_child).iter())
                                                                                                    .map(|(l,r)| l&r).collect(),
            AstNode::Binary(BinaryOp::Or,left_child,right_child) => self.evaluate_syntax_tree(*left_child).iter()
                                                                                                    .zip(self.evaluate_syntax_tree(*right_child).iter())
                                                                                                    .map(|(l,r)| l|r).collect(),
            AstNode::Name(word) => self.database.get(&word).unwrap_or(&vec![0;self.extra_variables.as_ref().unwrap().article_titles.len()]).to_vec()
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn setup_real() -> Index<HashMap<String,Vec<u64>>,Index7ExtraVariables> {
        let config = Config::build(&["".to_string(),"data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),"7".to_string()]).unwrap();
        Index::index7(&config).unwrap()
    }

    fn setup_test() -> Index<HashMap<String,Vec<u64>>,Index7ExtraVariables> {
        let mut database: HashMap<String,Vec<u64>> = HashMap::new();
        database.insert("word1".to_string(), vec![0b0000_0001]);
        database.insert("word2".to_string(), vec![0b1111_1111]);
        database.insert("word3".to_string(), vec![0b0101_0101]);
        database.insert("word4".to_string(), vec![0b0000_1110]);
        let mut article_titles: Vec<String> = Vec::new();
        for i in 0..100 {
            article_titles.push(format!("article {}", i).to_string());
        };
        Index {
            database,
            extra_variables: Some(Index7ExtraVariables{
                article_titles
            })
        }
    }

    #[test]
    fn bitvec_to_articleset_works() {
        let test_index = setup_test();

        let bitvec: Vec<u64> = vec![0b0000_0011];

        let hs = vec!["article 0".to_string(),"article 1".to_string()];
        assert_eq!(test_index.bitvec_to_articleset(bitvec).unwrap() , hs)
    }

    #[should_panic]
    #[test]
    fn bitvec_to_articleset_panics_when_out_of_range() {
        let test_index = setup_test();

        let bitvec: Vec<u64> = vec![0 , 0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000];

        test_index.bitvec_to_articleset(bitvec).unwrap();
    }

    fn search_match (
        index: &Index<HashMap<String,Vec<u64>>,Index7ExtraVariables>, 
        query: &str, 
        titles: Vec<&str>
    ) {
        dbg!(&query.to_string());
        let index_result: HashSet<String> = HashSet::from_iter(index.boolean_search(&query.to_string()).unwrap_or(Vec::default()));
        assert_eq!(index_result, HashSet::from_iter(titles.iter().map(|s| s.to_string())))
    }

    #[test]
    fn boolean_search_for_words_in_wiki100_kb() {
        let index = setup_real();
        
        search_match(&index, "the | autism", vec!["Anarchism","Autism","A","Albedo"]);
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
        search_match(&index, "word1 | word4", vec!["article 0","article 1","article 2","article 3"]);
    }

    #[test]
    fn or_and_and() {
        let index = setup_test();
        search_match(&index, "word1 | (word3 & word4)", vec!["article 0","article 2"]);
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
}
