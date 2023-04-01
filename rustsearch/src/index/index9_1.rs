use super::index9_0::{Trie, TrieNode};

impl Trie {
    fn articlevec_to_bitvec(&self, articlevec: &Vec<usize>) -> Vec<usize> {
        let arch_bits = usize::BITS as usize;
        let mut bitvec: Vec<usize> = vec![0; (self.n_titles - 1) / arch_bits + 1];

        for n in articlevec {
            let title_bit: usize = 1 << (n % arch_bits);
            bitvec[n / arch_bits] = bitvec[n / arch_bits] | title_bit;
        }
        bitvec
    }

    fn or_bitvec(&self, articlevec1: Vec<usize>, articlevec2: Vec<usize>) -> Vec<usize> {
        articlevec1
            .iter()
            .zip(articlevec2.iter())
            .map(|(l, r)| l | r)
            .collect()
    }

    fn get_subtree_match(&self, node: &TrieNode) -> Vec<usize> {
        match &node.article_vec {
            Some(articles) => node.children_map.values().fold(
                self.articlevec_to_bitvec(articles),
                |acc: Vec<usize>, child: &TrieNode| {
                    (self.or_bitvec(acc, self.get_subtree_match(child)))
                },
            ),
            None => {
                let mut children = node.children_map.values();
                let first_child = children.next().unwrap();
                children.fold(self.get_subtree_match(first_child), |acc, child| {
                    self.or_bitvec(acc, self.get_subtree_match(child))
                })
            }
        }
    }
}
