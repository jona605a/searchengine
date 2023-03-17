use std::collections::HashMap;

use crate::index::index8_0::Index8ExtraVariables;
use crate::index::Index;
use crate::parsing::*;

impl Index<HashMap<String, Vec<usize>>, Index8ExtraVariables> {
    // Index8ExtraVariables allow it to inherit the indexing from index8

    pub fn evaluate_syntax_tree_convert_to_bitvecs(&self, node: AstNode) -> Vec<usize> {
        match node {
            AstNode::Invert(child) => self
                .evaluate_syntax_tree_convert_to_bitvecs(*child)
                .iter()
                .map(|bv| !bv)
                .collect(),
            AstNode::Binary(BinaryOp::And, left_child, right_child) => self
                .evaluate_syntax_tree_convert_to_bitvecs(*left_child)
                .iter()
                .zip(self.evaluate_syntax_tree_convert_to_bitvecs(*right_child).iter())
                .map(|(l, r)| l & r)
                .collect(),
            AstNode::Binary(BinaryOp::Or, left_child, right_child) => self
                .evaluate_syntax_tree_convert_to_bitvecs(*left_child)
                .iter()
                .zip(self.evaluate_syntax_tree_convert_to_bitvecs(*right_child).iter())
                .map(|(l, r)| l | r)
                .collect(),
            AstNode::Name(word) => self.to_bitvec(self.database.get(&word).unwrap_or(&vec![]).to_vec()),
        }
    }

    // fn to_bitvec(&self, articlevec: Vec<usize>) -> Vec<usize> {
    //     let mut bitvec = vec![];
        
        
        

    //     let title_bit: usize = 1 << ((n_titles - 1) % arch_bits);
    //     v[(n_titles - 1) / arch_bits] = v[(n_titles - 1) / arch_bits] | title_bit;
    // }
}
