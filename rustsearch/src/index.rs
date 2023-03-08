pub mod index6;
pub mod index7;
pub mod index8;
pub mod index8_1;
pub mod index8_2;


pub struct Index<T,G> {
    database: T,
    extra_variables: Option<G>,
}





















// pub struct Article {
//     title: String,
//     contents: regex::Split,
// }

// use regex::Regex;
// impl Article {
//     pub fn build_from_filecontents(filecontents: &String) -> Vec<Article> {
//         let articles = filecontents.split("---END.OF.DOCUMENT---");
//         let re = Regex::new(r"\. |\.\n|\n\n|; |[\[\]\{\}\\\n\(\) ,:/=?!*]").unwrap();
        
//         articles
//             .map(|a| {
//                 let (title, contents) = a.split_once("\n").expect("There should be a newline character in every article");
//                 Article {
//                     title: title.to_string(),
//                     contents: re.split(contents),
//                 }
//             }).collect()
//     }
// }
