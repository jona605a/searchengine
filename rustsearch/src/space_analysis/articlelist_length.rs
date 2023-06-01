#[cfg(test)]
mod tests {
    use csv::Writer;
    use std::{collections::HashMap, fs};

    use crate::{
        helpers::Config,
        index::Index,
    };
    #[test]
    #[ignore]
    fn articlelist_length_count() {
        let files = fs::read_dir("data/");
        
        let mut stats: HashMap<String, Vec<usize>> = HashMap::new();

        for dir in files.unwrap() {
            if dir.as_ref().unwrap().path().is_dir() {
                continue;
            }
            let file_path = dir.unwrap().path().into_os_string().into_string().unwrap();
            if &file_path[0..9] != "data/West" {
                continue;
            }
            
            let filesize = match file_path.rsplit_once('_') {
                Some((_, suffix)) => suffix.split_once('.').unwrap().0,
                None => continue,
            };
            dbg!("Running on file {filesize}...");
            
            let config = Config {
                file_path: file_path.to_owned(),
                indexno: "8_0".to_string(),
            };
            
            let index = Index::index8(&config).unwrap();

            //let mut file_stat = vec![0;index.get_article_titles().len()];
            let mut file_stat = vec![0;20001];
            
            let database = index.get_database_lin();

            for articlelist in database.values(){
                let n = articlelist.len();
                file_stat[n-1] += 1
            }
            file_stat[20000] = index.get_article_titles().len();
            
            stats.insert(filesize.to_string(), file_stat);
        }
        
        let mut wtr = Writer::from_path("articlelist_length.csv").unwrap();
        for (k, v) in stats {
            let mut s: Vec<String> = v
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            s.push(k);
            wtr.write_record(s).unwrap();
        }
    }

    #[test]
    #[ignore]
    fn mean_lenght_article_list() {
        let files = fs::read_dir("data/");
        
        for dir in files.unwrap() {
            if dir.as_ref().unwrap().path().is_dir() {
                continue;
            }
            let file_path = dir.unwrap().path().into_os_string().into_string().unwrap();
            if &file_path[0..9] != "data/West" {
                continue;
            }
            
            let filesize = match file_path.rsplit_once('_') {
                Some((_, suffix)) => suffix.split_once('.').unwrap().0,
                None => continue,
            };
            dbg!(filesize);
            
            let config = Config {
                file_path: file_path.to_owned(),
                indexno: "8_0".to_string(),
            };
            
            let index = Index::index8(&config).unwrap();
            
            let database = index.get_database_lin();
            let sum_of_length: usize  = database.values().map(|x|x.len()).sum();
            let number_of_list: usize = database.keys().len();
            let mean: f64 = sum_of_length as f64 /number_of_list as f64 ;
            let a:usize  = index.get_article_titles().len();
            let threshold: f64  = a as f64 /64 as f64 ;
            dbg!(mean,threshold);
        }
        
    }
}
