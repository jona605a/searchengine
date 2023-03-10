#![allow(dead_code)]

use crate::parsing::{AstNode, BinaryOp};
use rand::Rng;
use rand::rngs::StdRng;

pub fn boolean_ast_gen(database_words: &Vec<&String>, depth: usize, rng: &mut StdRng) -> Box<AstNode> {
    if depth == 0 {
        // return Name randomly from database
        return match rng.gen_range(1..=10) {
            1 => Box::new(AstNode::Name("icantbefound".to_string())),
            _ => Box::new(AstNode::Name(database_words[rng.gen_range(1..database_words.len())].to_string()))
        }
    };
    match rng.gen_range(1..=3) {
        1 => Box::new(AstNode::Invert(boolean_ast_gen(database_words, depth-1, rng))),
        2 => Box::new(AstNode::Binary(BinaryOp::And, boolean_ast_gen(database_words, depth-1, rng), boolean_ast_gen(database_words, depth-1, rng))),
        3 => Box::new(AstNode::Binary(BinaryOp::Or , boolean_ast_gen(database_words, depth-1, rng), boolean_ast_gen(database_words, depth-1, rng))),
        y => panic!("Should not be possible to generate this number: {}", y)
    }
}

#[cfg(test)]
mod bool_tests {
    use super::*;
    use rand::SeedableRng;
    use std::collections::HashMap;

    use crate::index::Index;
    use crate::index::index8_0::Index8ExtraVariables;
    use crate::parsing::{Expr, ExprData};

    fn setup_test8() -> Index<HashMap<String,Vec<usize>>,Index8ExtraVariables> {
        let mut database: HashMap<String,Vec<usize>> = HashMap::new();
        database.insert("word1".to_string(), vec![0]);
        database.insert("word2".to_string(), vec![0,1,2,3,4,5,6,7]);
        database.insert("word3".to_string(), vec![0,2,4,6]);
        database.insert("word4".to_string(), vec![1,2,3]);
        let mut article_titles: Vec<String> = Vec::new();
        for i in 0..100 {
            article_titles.push(format!("article {}", i).to_string());
        };
        Index {
            database,
            extra_variables: Some(Index8ExtraVariables{
                article_titles
            })
        }
    }


    #[test]
    fn ast_gen_can_be_seeded() {
        let mut rng = StdRng::seed_from_u64(8008135);
        
        let index = setup_test8();
        let mut database_words = index.database.keys().collect::<Vec<&String>>();
        database_words.sort();
        
        let ast = boolean_ast_gen(&database_words, 3, &mut rng);

        let should_be = match Expr::from_string(&"!((word3 | icantbefound) and (word2 and word4))".to_string()).unwrap() {
            Expr(ExprData::HasNodes(node)) => Box::new(node),
            _ => panic!()
        };
        
        assert_eq!(ast, should_be)
    }

}


