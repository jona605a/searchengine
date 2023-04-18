#![allow(dead_code)]

use crate::helpers::Config;
use crate::parsing::{AstNode, BinaryOp};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::Index;


fn get_search_word(
    database_words: &Vec<&String>,
    rng: &mut StdRng
) -> String{
    return match rng.gen_range(1..=10) {
        1 => "icantbefound".to_string(),
        _ => database_words[rng.gen_range(1..database_words.len())].to_string()
    };
}

fn get_search_prefix(
    database_words: &Vec<&String>,
    rng: &mut StdRng
) -> String{
    return match rng.gen_range(1..=10) {
        1 => ". *".to_string(),
        _ => {let word = database_words[rng.gen_range(1..database_words.len())].to_string();
              word.get(0..3).unwrap_or(&word).to_string()+"*"}

    };
}

fn boolean_ast_gen(
    database_words: &Vec<&String>,
    depth: usize,
    rng: &mut StdRng
) -> Box<AstNode> {
    if depth == 0 {
        // return Name randomly from database
        return match rng.gen_range(1..=10) {
            1 => Box::new(AstNode::Name("icantbefound".to_string())),
            _ => Box::new(AstNode::Name(
                database_words[rng.gen_range(1..database_words.len())].to_string(),
            )),
        };
    };
    match rng.gen_range(1..=3) {
        1 => Box::new(AstNode::Invert(boolean_ast_gen(
            database_words,
            depth - 1,
            rng,
        ))),
        2 => Box::new(AstNode::Binary(
            BinaryOp::And,
            boolean_ast_gen(database_words, depth - 1, rng),
            boolean_ast_gen(database_words, depth - 1, rng),
        )),
        3 => Box::new(AstNode::Binary(
            BinaryOp::Or,
            boolean_ast_gen(database_words, depth - 1, rng),
            boolean_ast_gen(database_words, depth - 1, rng),
        )),
        y => panic!("Should not be possible to generate this number: {}", y),
    }
}


pub fn gen_a_lot_of_runs_bool(file_path: String, number : usize) -> Vec<Vec<Box<AstNode>>> {
    let mut rng = StdRng::seed_from_u64(8008135);

    let config: Config = Config::build(&["".to_string(),file_path.clone(),"7".to_string()]).unwrap();
    
    let index8 = Index::index8(&config).unwrap();

    let mut database_words = index8.database.keys().collect::<Vec<&String>>();
    database_words.sort();
    
    
    let boolean_queries = (0..=7).map(|depth| {
        (1..=number).map(|_| boolean_ast_gen(&database_words, depth, &mut rng)).collect::<Vec<Box<AstNode>>>()
    }).collect::<Vec<Vec<Box<AstNode>>>>();
    
    boolean_queries
}

pub fn gen_a_lot_of_runs_tries(file_path: String, number : usize, prefix: bool) -> Vec<String> {
    let mut rng = StdRng::seed_from_u64(8008135);

    let config: Config = Config::build(&["".to_string(),file_path.clone(),"7".to_string()]).unwrap();
    
    let index8 = Index::index8(&config).unwrap();

    let mut database_words = index8.database.keys().collect::<Vec<&String>>();
    database_words.sort();

    let search_queries = match prefix {
        true => (1..=number).map(|_|{get_search_prefix(&database_words, &mut rng)}).collect::<Vec<String>>(),
        false => (1..=number).map(|_|{get_search_word(&database_words, &mut rng)}).collect::<Vec<String>>()
    };
    search_queries
}

#[cfg(test)]
mod bool_tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashMap;
    use std::fs;
    use std::iter::zip;

    use crate::index::index8_0::Index8ExtraVariables;
    use crate::index::Index;
    use crate::parsing::{Expr, ExprData};

    fn setup_test8() -> Index<HashMap<String, Vec<usize>>, Index8ExtraVariables> {
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
    fn ast_gen_can_be_seeded() {
        let mut rng = StdRng::seed_from_u64(8008135);

        let index = setup_test8();
        let mut database_words = index.database.keys().collect::<Vec<&String>>();
        database_words.sort();

        let ast = boolean_ast_gen(&database_words, 3, &mut rng);

        let should_be =
            match Expr::from_string(&"!((word3 | icantbefound) and (word2 and word4))".to_string())
                .unwrap()
            {
                Expr(ExprData::HasNodes(node)) => Box::new(node),
                _ => panic!(),
            };

        assert_eq!(ast, should_be)
    }

    #[test]
    fn get_search_word_can_be_seeded() {
        let mut rng = StdRng::seed_from_u64(8008135);

        let index = setup_test8();
        let mut database_words = index.database.keys().collect::<Vec<&String>>();
        database_words.sort();

        let result1 = get_search_word(&database_words, &mut rng);
        let result2 = get_search_word(&database_words, &mut rng);
        let result3 = get_search_word(&database_words, &mut rng);
        let result4 = get_search_word(&database_words, &mut rng);

        assert_eq!(result1, "icantbefound");
        assert_eq!(result2, "word4");
        assert_eq!(result3, "word3");
        assert_eq!(result4, "word2");
    }

    #[test]
    fn get_search_prefix_can_be_seeded() {
        let mut rng = StdRng::seed_from_u64(8008135);

        let index = setup_test8();
        let mut database_words = index.database.keys().collect::<Vec<&String>>();
        database_words.sort();

        let result1 = get_search_prefix(&database_words, &mut rng);
        let result2 = get_search_prefix(&database_words, &mut rng);
        let result3 = get_search_prefix(&database_words, &mut rng);
        let result4 = get_search_prefix(&database_words, &mut rng);

        assert_eq!(result1, ". *");
        assert_eq!(result2, "wor*");
        assert_eq!(result3, "wor*");
        assert_eq!(result4, "wor*");
    }

    fn astnode_to_string(ast:Box<AstNode>) -> String{
        return match *ast{
            AstNode::Name(s) => s,
            AstNode => "AstNode does not have depth 0".to_string()
        }
    }

    #[test]
    fn gen_a_lot_of_runs_bool_and_gen_a_lot_of_runs_tries_gives_the_same_words(){
        let files = fs::read_dir("../../data.nosync/");

        for dir in files.unwrap() {
            let file = dir.unwrap().path().into_os_string().into_string().unwrap();
            let filesize = &file[46..file.len()-4];

            if (filesize != "1MB") & (filesize != "2MB"){
                continue;
            }

            let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 10);
            let word_vec = gen_a_lot_of_runs_tries(file.clone(), 10,false);

            let depth_vec = &ast_vec[0];

            for (ast,word) in zip(depth_vec,word_vec)  {
                    print!("THE WORD THAT FAILED IS {}\n", &word);
                    assert_eq!(astnode_to_string(ast.clone()),word);
                };

        }

    }
}
