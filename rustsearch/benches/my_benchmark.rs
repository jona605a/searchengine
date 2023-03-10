// https://bheisler.github.io/criterion.rs/book/getting_started.html
use criterion::{criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use rand::rngs::StdRng;

use rustsearch::helpers::{Config, read_file_to_string};
use rustsearch::index::boolean_tests::boolean_ast_gen;
use rustsearch::index::{self, Index};
use rustsearch::parsing::AstNode;


// Timing reading files
pub fn opening_and_reading_file(c: &mut Criterion) {
    let file100kb = "data/WestburyLab.wikicorp.201004_100KB.txt";
    let file5mb = "data/WestburyLab.wikicorp.201004_5MB.txt";
    c.bench_function("Opening and reading file 100 KB", |b| b.iter(|| {
        read_file_to_string(&file100kb.to_string()).unwrap();
    }));
    c.bench_function("Opening and reading file 5 MB", |b| b.iter(|| {
        read_file_to_string(&file5mb.to_string()).unwrap();
    }));
}

// Timing the indexing on different files
pub fn indexing_7_file_100_kb(c: &mut Criterion) {
    c.bench_function("index 8 indexing 100 KB", |b| b.iter(|| {
        index::Index::index7(&rustsearch::helpers::Config {file_path : "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(), indexno : "7_0".to_string()})
    }));
}

pub fn indexing_7_file_5_mb(c: &mut Criterion) {
    c.bench_function("index 8 indexing 5 MB", |b| b.iter(|| {
        index::Index::index7(&rustsearch::helpers::Config {file_path : "data/WestburyLab.wikicorp.201004_5MB.txt".to_string(), indexno : "7_0".to_string()})
    }));
}

pub fn indexing_8_file_100_kb(c: &mut Criterion) {
    c.bench_function("index 8 indexing 100 KB", |b| b.iter(|| {
        index::Index::index8(&rustsearch::helpers::Config {file_path : "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(), indexno : "8_0".to_string()})
    }));
}

pub fn indexing_8_file_5_mb(c: &mut Criterion) {
    c.bench_function("index 8 indexing 5 MB", |b| b.iter(|| {
        index::Index::index8(&rustsearch::helpers::Config {file_path : "data/WestburyLab.wikicorp.201004_5MB.txt".to_string(), indexno : "8_0".to_string()})
    }));
}


// Timing search times

fn gen_a_lot_of_runs(file_path: String, number : usize) -> Vec<Vec<Box<AstNode>>> {
    let mut rng = StdRng::seed_from_u64(8008135);

    let config: Config = Config::build(&["".to_string(),file_path.clone(),"7".to_string()]).unwrap();
    
    let index8 = Index::index8(&config).unwrap();

    let mut database_words = index8.database.keys().collect::<Vec<&String>>();
    database_words.sort();
    
    
    let boolean_queries = (1..=7).map(|depth| {
        (1..=number).map(|_| boolean_ast_gen(&database_words, depth, &mut rng)).collect::<Vec<Box<AstNode>>>()
    }).collect::<Vec<Vec<Box<AstNode>>>>();
    
    boolean_queries
}


pub fn searching_index_7_0_5MB(c: &mut Criterion) {
    let file = "data/WestburyLab.wikicorp.201004_5MB.txt";
    let ast_vec = gen_a_lot_of_runs(file.to_string(), 1000);
    let index = index::Index::index7(&rustsearch::helpers::Config {file_path : file.to_string(), indexno : "7".to_string()}).unwrap();
    let mut depth = 0;
    for depth_vec in ast_vec {
        depth += 1;
        c.bench_function(&format!("searching index 7_0 in file 5 MB, depth {}", depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.bitvec_to_articleset(index.evaluate_syntax_tree(*ast.clone()));
            }
        }));
    }
}

pub fn searching_index_8_0_5MB(c: &mut Criterion) {
    let file = "data/WestburyLab.wikicorp.201004_5MB.txt";
    let ast_vec = gen_a_lot_of_runs(file.to_string(), 1000);
    let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.to_string(), indexno : "8".to_string()}).unwrap();
    let mut depth = 0;
    for depth_vec in ast_vec {
        depth += 1;
        c.bench_function(&format!("searching index 8_0 in file 5 MB, depth {}", depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articleset(index.evaluate_syntex_tree_naive(*ast.clone()));
            }
        }));
    }
}

pub fn searching_index_8_1_5MB(c: &mut Criterion) {
    let file = "data/WestburyLab.wikicorp.201004_5MB.txt";
    let ast_vec = gen_a_lot_of_runs(file.to_string(), 1000);
    let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.to_string(), indexno : "8".to_string()}).unwrap();
    let mut depth = 0;
    for depth_vec in ast_vec {
        depth += 1;
        c.bench_function(&format!("searching index 8_1 in file 5 MB, depth {}", depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articleset(index.evaluate_syntex_tree_demorgan(*ast.clone()));
            }
        }));
    }
}

pub fn searching_index_8_2_5MB(c: &mut Criterion) {
    let file = "data/WestburyLab.wikicorp.201004_5MB.txt";
    let ast_vec = gen_a_lot_of_runs(file.to_string(), 1000);
    let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.to_string(), indexno : "8".to_string()}).unwrap();
    let mut depth = 0;
    for depth_vec in ast_vec {
        depth += 1;
        c.bench_function(&format!("searching index 8_2 in file 5 MB, depth {}", depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articleset(index.evaluate_syntex_tree_binary_search(*ast.clone()));
            }
        }));
    }
}

pub fn searching_index_8_3_5MB(c: &mut Criterion) {
    let file = "data/WestburyLab.wikicorp.201004_5MB.txt";
    let ast_vec = gen_a_lot_of_runs(file.to_string(), 1000);
    let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.to_string(), indexno : "8".to_string()}).unwrap();
    let mut depth = 0;
    for depth_vec in ast_vec {
        depth += 1;
        c.bench_function(&format!("searching index 8_3 in file 5 MB, depth {}", depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articleset(index.evaluate_syntex_tree_hybrid(*ast.clone()));
            }
        }));
    }
}

criterion_group!(benches,indexing_7_file_5_mb, indexing_8_file_5_mb);
criterion_main!(benches);

