use std::collections::{HashMap};
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

    pub fn index8(config: &Config) -> Result<Index<HashMap<String,Vec<u64>>,Index7ExtraVariables>, Box<dyn Error>> {
        let mut database: HashMap<String,Vec<u64>> = HashMap::new();
        
        let filecontents = read_file_to_string(&config.file_path)?;
        let re = Regex::new(r"\. |\.\n|\n\n|; |[\[\]\{\}\\\n\(\) ,:/=?!*]").unwrap();

        let mut prev_word = String::from("---END.OF.DOCUMENT---");
        let mut article_titles: Vec<String> = Vec::new();
        
        let mut x = re.split(&filecontents);
        x.next();

        let mut n_titles = 0;
        let mut v_len = 1;

        for word in  x{
            if word == "---END.OF.DOCUMENT---" {
                prev_word = word.to_string();
                continue;
            }
            // Update title
            if prev_word == "---END.OF.DOCUMENT---" {
                article_titles.push(word.to_string());
                prev_word = String::new();
                n_titles += 1;
            }
            if n_titles > v_len*64 {
                // Extend the length of all vectors by 1
                for v in database.values_mut() {
                    v.push(0);
                }
                v_len += 1;
            }

            // dbg!(word);

            let v = database.entry(word.to_string())
                .or_default();
            v.push(n_titles-1)
        }


        
        Ok(Index {
            database, 
            extra_variables: Some(Index7ExtraVariables{article_titles})
        })
    }

    pub fn vec_to_articleset(&self, vec: Vec<u64>) -> Option<Vec<String>> {
        let mut output: Vec<String> = Vec::new();
        let titles = &self.extra_variables.as_ref().unwrap().article_titles;
        for i in vec {
                    output.push(titles[i as usize].clone());
                }
        Some(output)
    }

    pub fn boolean_search(&self, exp: &String) -> Option<Vec<String>>{
        let ast: Expr = Expr::from_string(&exp).unwrap();
        let result = match ast {
            Expr(nodes) => match nodes {
                    ExprData::HasNodes(node) => Some(self.recursive_tree(node)),
                    ExprData::Empty => return None,
            }
        };
        self.vec_to_articleset(result.unwrap())
    }

    fn recursive_tree(&self, node:AstNode)-> Vec<u64> {
        match node{
            AstNode::Invert(child) => self.invert(self.recursive_tree(*child)),
            AstNode::Binary(BinaryOp::And,left_child,right_child) => self.and(self.recursive_tree(*left_child),self.recursive_tree(*right_child)),
            AstNode::Binary(BinaryOp::Or,left_child,right_child) => self.or(self.recursive_tree(*left_child),self.recursive_tree(*right_child)),
            AstNode::Name(word) => self.database.get(&word).unwrap_or(&vec![]).to_vec(),
        }
    }

    fn and(&self, left_child:Vec<u64>,right_child:Vec<u64>)-> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        let mut l = 0;
        let mut r = 0;
    

        while (left_child.len() >= l) & (right_child.len()>= r) {
            if left_child[l] > right_child[r]{
                r = r+1;
            }
            else if left_child[l] < right_child[r]{
                l = l+1;
            }
            else{
                result.push(left_child[l]);
                l = l +1;
                r = r +1;
            }
        }

        result
    }

    fn or(&self, left_child:Vec<u64>,right_child:Vec<u64>)-> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        let mut l = 0;
        let mut r = 0;

        while (left_child.len() > l) || (right_child.len() > r) {
            
            if l == left_child.len(){
                result.push(right_child[r]);
                r += 1;
            }

            else if r == right_child.len(){
                result.push(left_child[l]);
                l += 1;
            }

            else if left_child[l] > right_child[r]{
                result.push(right_child[r]);
                r += 1;
            }
            else if left_child[l] < right_child[r]{
                result.push(left_child[l]);
                l += 1;
            }
            else{
                result.push(left_child[l]);
                l += 1;
                r += 1;
            }
        }
        result
    }

    fn invert(&self, child:Vec<u64>) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        let mut p = 0;

        for i in 0u64 .. self.extra_variables.as_ref().unwrap().article_titles.len() as u64{
            
            if (p >= child.len()) || (i<child[p]){
                result.push(i)
            }
            else{
                p = p + 1;

            }

        }

        result
    }
        

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn setup_real() -> Index<HashMap<String,Vec<u64>>,Index7ExtraVariables> {
        let config = Config::build(&["".to_string(),"data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),"8".to_string()]).unwrap();
        Index::index8(&config).unwrap()
    }

    fn setup_test() -> Index<HashMap<String,Vec<u64>>,Index7ExtraVariables> {
        let mut article_titles: Vec<String> = Vec::new();
        for i in 0..100 {
            article_titles.push(format!("Article {}", i).to_string());
        }
        Index {
            database: HashMap::new(), // Empty database
            extra_variables: Some(Index7ExtraVariables{
                article_titles
            })
        }
    }

    #[test]
    fn bitvec_to_articleset_works() {
        let test_index = setup_test();

        let bitvec: Vec<u64> = vec![0,1];

        let hs = vec!["Article 0".to_string(),"Article 1".to_string()];
        assert_eq!(test_index.vec_to_articleset(bitvec).unwrap() , hs)
    }

    #[should_panic]
    #[test]
    fn bitvec_to_articleset_panics_when_out_of_range() {
        let test_index = setup_test();

        let bitvec: Vec<u64> = vec![100000000];

        test_index.vec_to_articleset(bitvec).unwrap();
    }

    fn search_match (
        index: &Index<HashMap<String,Vec<u64>>,Index7ExtraVariables>, 
        word: &str, 
        titles: Vec<String>
    ) {
        // dbg!(&word.to_string());
        let index_result: HashSet<String> = HashSet::from_iter(index.boolean_search(&word.to_string()).unwrap_or(Vec::default()));
        dbg!(&index_result);
        assert_eq!(index_result, HashSet::from_iter(titles))
    }

    #[test]
    fn boolean_search_for_words_in_wiki100_kb() {
        let index = setup_real();
        
        search_match(&index, "the | autism", vec!["Anarchism".to_string(),"Autism".to_string(),"A".to_string(),"Albedo".to_string()]);
        search_match(&index, "autism", vec!["Autism".to_string()]); // A word that should only be in one article
        search_match(&index, "bi-hemispherical", vec!["Albedo".to_string()]); // Check for no splitting of 'bi-hemispherical'
        // search_match(&index, "\"&amp;#65;\"", vec!["A".to_string()]); // A word that has special characters
    }

    #[test]
    fn boolean_correctness() {
        let mut database: HashMap<String,Vec<u64>> = HashMap::new();
        
    }





}

