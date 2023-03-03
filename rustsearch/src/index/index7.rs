use std::collections::{HashMap,HashSet};
use std::error::Error;
use std::process::Command;
use std::string;
use regex::Regex;

use crate::index::Index;
use crate::helpers::*;

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
            while v.len() < v_len {v.push(0)}
            let title_bit: u64 = 1 << ((n_titles-1)%64);
            v[(n_titles-1)/64] = v[(n_titles-1)/64] | title_bit;
        }


        let index : Index<HashMap<String, Vec<u64>>, Index7ExtraVariables> = 
            Index {
                database, 
                extra_variables: Some(Index7ExtraVariables{article_titles})
            };
        dbg!(&index.extra_variables.as_ref().unwrap().article_titles);
        Ok(index)
    }

    pub fn bitvec_to_articleset(&self, bitvecs: &Vec<u64>) -> Option<HashSet<String>> {
        let mut output: HashSet<String> = HashSet::new();
        let titles = &self.extra_variables.as_ref().unwrap().article_titles;
        for i in 0..bitvecs.len() {
            for bit in 0..64 {
                if (1<<(bit)) & bitvecs[i] > 0 {
                    if titles.len() <= i*64+bit {
                        panic!("Error, looked-up word refers to an article with a larger index than there are titles: {}",i*64+bit)
                    }
                    output.insert(titles[i*64+bit].clone());
                }
            }
        }
        Some(output)
    }

    pub fn search(&self, word: &String) -> Option<HashSet<String>> {
        let bitvecs = self.database.get(word)?;
        self.bitvec_to_articleset(bitvecs)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    fn setup_real() -> Index<HashMap<String,Vec<u64>>,Index7ExtraVariables> {
        let config = Config::build(&["".to_string(),"data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),"7".to_string()]).unwrap();
        Index::index7(&config).unwrap()
    }

    fn setup_test() -> Index<HashMap<String,Vec<u64>>,Index7ExtraVariables> {
        let mut article_titles: Vec<String> = Vec::new();
        for i in 1..101 {
            article_titles.push(format!("Article {}", i).to_string());
        }
        Index {
            database: HashMap::new(),
            extra_variables: Some(Index7ExtraVariables{
                article_titles
            })
        }
    }

    #[test]
    fn bitvec_to_articleset_works() {
        let test_index = setup_test();

        let bitvec: Vec<u64> = vec![0b0000_0011];

        let hs: HashSet<String> = HashSet::from_iter(vec!["Article 1".to_string(),"Article 2".to_string()]);
        assert_eq!(test_index.bitvec_to_articleset(&bitvec).unwrap() , hs)
    }

    #[should_panic]
    #[test]
    fn bitvec_to_articleset_panics() {
        let test_index = setup_test();

        let bitvec: Vec<u64> = vec![0,0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000];

        test_index.bitvec_to_articleset(&bitvec).unwrap();
    }

    #[test]
    fn searches_for_words_in_wiki100_kb() {
        let index = setup_real();
        let search_match = |word: &str, titles: Vec<String>| {
            dbg!(&word.to_string());
            assert_eq!(index.search(&word.to_string()).unwrap_or(HashSet::default()), HashSet::from_iter(titles))
        };
        search_match("the", vec!["Anarchism".to_string(),"Autism".to_string(),"A".to_string(),"Albedo".to_string()]);
        search_match("autism", vec!["Autism".to_string()]); // A word that should only be in one article
        search_match("\"&amp;#65;\"", vec!["A".to_string()]); // A word that has special characters
        search_match("bi-hemispherical", vec!["Albedo".to_string()]); // Check for no splitting of 'bi-hemispherical'
    }
}

