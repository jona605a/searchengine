#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::{
        helpers::Config,
        index::{Index, gen_query::gen_a_lot_of_runs_full_text, Query, SearchType, Search},
    };
    #[test]
    #[ignore]
    fn BadCharGoodSuf_shift_Count() {
        let file_path = "data/WestburyLab.wikicorp.201004_5MB.txt".to_string();
    
        let config = Config {
            file_path: file_path.to_owned(),
            indexno: "10_1".to_string(),
        };
        
        let index = Index::index10(&config).unwrap();
        
        let full_text_queries = gen_a_lot_of_runs_full_text(file_path.clone(), 10);

        for sentence in &full_text_queries {
            println!("{}", sentence);
            let query = Query {
                search_string: sentence.to_owned(),
                search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
            };

            index.search(&query);
        }

    }
}
