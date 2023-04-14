// https://bheisler.github.io/criterion.rs/book/getting_started.html
#![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, Criterion};
use rand::{SeedableRng, Rng};
use rand::rngs::StdRng;
use std::{fs, string};

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
pub fn indexing_7(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        c.bench_function(&format!("indexing index 7 {}",filesize),|b| b.iter(|| {
            index::Index::index7(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "7_0".to_string()})
        }) );
}
}

pub fn indexing_8(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        c.bench_function(&format!("indexing index 8 {}",filesize),|b| b.iter(|| {
            index::Index::index8(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "8_0".to_string()})
        }) );
}
}

pub fn indexing_9(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        c.bench_function(&format!("indexing index 9 {}",filesize),|b| b.iter(|| {
            index::Index::index9(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "9_0".to_string()})
        }) );
}
}

// Timing search times

fn gen_a_lot_of_runs_bool(file_path: String, number : usize) -> Vec<Vec<Box<AstNode>>> {
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

fn gen_a_lot_of_runs_tries(file_path: String, number : usize) -> Vec<String> {
    let mut rng = StdRng::seed_from_u64(8008135);

    let config: Config = Config::build(&["".to_string(),file_path.clone(),"7".to_string()]).unwrap();
    
    let index8 = Index::index8(&config).unwrap();

    let mut database_words = index8.database.keys().collect::<Vec<&String>>();
    database_words.sort();

    let boolean_queries = (1..=number).map(|_|{
        return match rng.gen_range(1..=10) {
            1 => "icantbefound".to_string(),
            _ => database_words[rng.gen_range(1..database_words.len())].to_string()
        };

    }).collect::<Vec<String>>();
    
    boolean_queries
}


pub fn searching_index_7_0(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 1000);
        let index = index::Index::index7(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "7".to_string()}).unwrap();
        let mut depth = 0;

        for depth_vec in ast_vec {
            depth += 1;
            c.bench_function(&format!("searching index 7_0 in file {}, depth {}", filesize, depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.bitvec_to_articlelist(index.evaluate_syntax_tree(*ast.clone()));
            }
        }));
    }
    } 
}

pub fn searching_index_8_0(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 1000);
        let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "8".to_string()}).unwrap();
        let mut depth = 0;

        for depth_vec in ast_vec {
            depth += 1;
            c.bench_function(&format!("searching index 8_0 in file {}, depth {}", filesize, depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articlelist(index.evaluate_syntex_tree_naive(*ast.clone()));
            }
        }));
    }
    } 
}

pub fn searching_index_8_1(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 1000);
        let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "8".to_string()}).unwrap();
        let mut depth = 0;

        for depth_vec in ast_vec {
            depth += 1;
            c.bench_function(&format!("searching index 8_1 in file {}, depth {}", filesize, depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articlelist(index.evaluate_syntex_tree_demorgan(*ast.clone()));
            }
        }));
    }
    } 
}

pub fn searching_index_8_2(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 1000);
        let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "8".to_string()}).unwrap();
        let mut depth = 0;

        for depth_vec in ast_vec {
            depth += 1;
            c.bench_function(&format!("searching index 8_2 in file {}, depth {}", filesize, depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articlelist(index.evaluate_syntex_tree_binary_search(*ast.clone()));
            }
        }));
    }
    } 
}

pub fn searching_index_8_3(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];
        print!("{}", filesize);


        let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 1000);
        let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "8".to_string()}).unwrap();
        let mut depth = 0;

        for depth_vec in ast_vec {
            depth += 1;
            c.bench_function(&format!("searching index 8_3 in file {}, depth {}", filesize, depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articlelist(index.evaluate_syntex_tree_hybrid(*ast.clone()));
            }
        }));
    }
    } 
}

pub fn searching_index_8_4(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 1000);
        let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "8".to_string()}).unwrap();
        let mut depth = 0;

        for depth_vec in ast_vec {
            depth += 1;
            c.bench_function(&format!("searching index 8_4 in file {}, depth {}", filesize, depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.bitvec_to_articlelist(index.evaluate_syntax_tree_convert_to_bitvecs(*ast.clone()));
            }
        }));
    }
    } 
}

pub fn searching_index_9_1(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let word_vec = gen_a_lot_of_runs_tries(file.clone(), 1000);
        let index = index::Index::index9(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "9".to_string()}).unwrap();

        c.bench_function(&format!("searching index 9_1 in file {}, depth {}", filesize, 0), |b| b.iter(|| {
            for word in &word_vec {
                index.trie_search_1(word);
            }
        }));
    } 
}

criterion_group!(benches,indexing_9);
criterion_main!(benches);

