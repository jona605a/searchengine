pub mod index6;
pub mod index7;


pub struct Index<T,G> {
    database: T,
    extra_variables: Option<G>,
}

#[cfg(ignore)]
pub struct Article<'a> {
    title: String,
    contents: regex::Split<'a, 'a>,
}

// use regex::Regex;
// impl Article<'_> {
//     pub fn build_from_filecontents(filecontents: &String) -> Vec<Article> {
//         let articles = filecontents.split("---END.OF.DOCUMENT---");
//         let re = Regex::new(r"\. |\.\n|\n\n|; |[\[\]\{\}\\\n\(\) ,:/=?!*]").unwrap();
        
//         articles
//             .map(move |a| {
//                 let (title, contents) = a.split_once("\n").expect("There should be a newline character in every article");
//                 Article {
//                     title: title.to_string(),
//                     contents: re.split(contents),
//                 }
//             }).collect()
//     }
// }
