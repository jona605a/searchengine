#[cfg(test)]
mod tests {
    use csv::Writer;
    use std::{collections::HashMap, fs};

    use crate::{
        helpers::Config,
        index::{index9_0::TrieNodeLin, Index},
    };
    #[test]
    #[ignore]
    fn create_index9_and_count() {
        let files = fs::read_dir("data/");

        let mut stats: HashMap<String, Vec<f64>> = HashMap::new();

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
                indexno: "9_0".to_string(),
            };
            let index = Index::index9_0(&config).unwrap();
            let trie = index.get_trie_lin();

            // 0, 1,    2,  3,          4,          5
            // N, avg, max, avglayer0, avglayer1, avglayer2
            let mut file_stat = vec![0.0; 6];
            let mut depth_map: HashMap<usize, Vec<&TrieNodeLin>> = HashMap::new();
            depth_map.insert(0, vec![&trie.root]);
            let mut depth = 0;

            let mut leaves = 0.0;
            let mut max_children = 0;

            while let Some(node_vec) = depth_map.get(&depth) {
                let mut next_layer_vec = vec![];
                for &node in node_vec {
                    for (_, child) in &node.children_vec {
                        next_layer_vec.push(child);
                    }

                    file_stat[0] += 1.0; // Total number of nodes
                    if node.children_vec.len() == 0 {
                        leaves += 1.0
                    }
                    if node.children_vec.len() > max_children {
                        max_children = node.children_vec.len();
                    }
                }
                if next_layer_vec.len() > 0 {
                    depth_map.insert(depth + 1, next_layer_vec);
                } else {
                    break;
                }
                if depth < 3 {
                    file_stat[3 + depth] = depth_map[&(depth + 1)].len() as f64 / depth_map[&depth].len() as f64;
                };
                depth += 1;
            }
            file_stat[1] = (file_stat[0] - 1.0) / (file_stat[0] - leaves); // Avg children = #children / #parents
            file_stat[2] = max_children as f64;
            stats.insert(filesize.to_string(), file_stat);
        }
        let mut wtr = Writer::from_path("trie_counting.csv").unwrap();
        for (k, v) in stats {
            let mut s: Vec<String> = v
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            s.push(k);
            wtr.write_record(s).unwrap();
        }
    }
}
