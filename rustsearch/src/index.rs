pub mod index6;
pub mod index7;
pub mod index8_0;
pub mod index8_1;
pub mod index8_2;
pub mod index8_3;
pub mod index8_4;
pub mod index9_0;
pub mod index9_1;
pub mod index10_0;
pub mod index10_1;
pub mod index11_0;

pub mod gen_query;

pub struct Index<T> {
    database: T,
    article_titles: ArticleTitles,
}

// #[derive(Debug)]
// pub struct ArticleTitles {
//     pub titles: Vec<String>
// }
type ArticleTitles = Vec<String>;

pub struct Query {
    pub search_string: String,
    pub search_type: SearchType
}
#[derive(Clone)]
pub enum SearchType {
    SingleWordSearch,
    BooleanSearch(String),
    PrefixSearch,
    ExactSearch(String),
    FuzzySearch,
}

impl std::fmt::Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchType::SingleWordSearch => write!(f, "SingleWordSearch"),
            SearchType::BooleanSearch(x) => write!(f, "BooleanSearch ({})", x),
            SearchType::PrefixSearch => write!(f, "PrefixSearch"),
            SearchType::ExactSearch(x) => write!(f, "ExactSearch ({})", x),
            SearchType::FuzzySearch => write!(f, "FuzzySearch"),
        }
    }
}

pub trait Search {
    fn search(&self, query: &Query) -> ArticleTitles;
}






























// pub enum Indices {
//     ISingleWord(Box<dyn SingleWordSearch>),
//     IBoolean(Box<dyn SingleAndBool>),
//     IPrefix(Box<dyn SingleAndPrefix>),
//     IExact(Box<dyn SingleAndExact>),
// }


// pub trait SingleWordSearch {
//     fn single_word_search(&self, query: &String) -> ArticleTitles;
// }

// pub trait BooleanSearch {
//     fn boolean_search(&self, query: &String) -> ArticleTitles;
// }

// pub trait PrefixSearch {
//     fn prefix_search(&self, query: &String) -> ArticleTitles;
// }

// pub trait ExactSearch {
//     fn exact_search(&self, query: &String) -> ArticleTitles;
// }

// pub trait FuzzySearch {
//     fn fuzzy_search(&self, query: &String) -> ArticleTitles;
// }

// pub trait SingleAndBool: SingleWordSearch + BooleanSearch {}

// pub trait SingleAndPrefix: SingleWordSearch + PrefixSearch {}

// pub trait SingleAndExact: SingleWordSearch + ExactSearch {}
