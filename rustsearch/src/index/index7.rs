use std::collections::HashMap;
use std::error::Error;
use regex::Regex;

use crate::index::Index;
use crate::helpers::*;
use crate::index::Index7ExtraVariables;




impl Index<HashMap<String,Vec<u64>>,Index7ExtraVariables> {

    pub fn index7(config: &Config) -> Result<Index<HashMap<String,Vec<u64>>,Index7ExtraVariables>, Box<dyn Error>> {
        let mut database: HashMap<String,Vec<u64>> = HashMap::new();
        
        let filecontents = read_file_to_string(&config.file_path)?;
        let re = Regex::new(r"\. |\.\n|\n\n|[\[\]\{\}\\\n\() ,;:/=?!*&]").unwrap();

        let mut prev_word = String::from("---END.OF.DOCUMENT---");
        let mut cur_title = String::new();
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
                cur_title = word.to_string();
                article_titles.push(word.to_string());
                prev_word = String::new();
                n_titles += 1;
            }
            if n_titles > v_len*64 {
                // "Rehash" / Push
                for v in database.values_mut() {
                    v.push(0);
                }
                v_len += 1;
            }

            let v = database.entry(word.to_string())
                .or_default();
            while v.len() < v_len {v.push(0)}
            let title_bit = 1 << (n_titles-1);
            v[(n_titles-1)/64] = v[(n_titles-1)/64] | title_bit;
        }


        let index : Index<HashMap<String, Vec<u64>>, Index7ExtraVariables>  = Index {database, extra_variables: Some(Index7ExtraVariables{article_titles})};

        Ok(index)
    }


}
