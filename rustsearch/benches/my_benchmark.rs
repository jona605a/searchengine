// https://bheisler.github.io/criterion.rs/book/getting_started.html
// #![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, Criterion};
use rustsearch::index::{Query, Search};
use std::fs;

use rustsearch::helpers::{read_file_to_string, Config};
use rustsearch::index::{self, gen_query::{gen_a_lot_of_runs_bool,gen_a_lot_of_runs_tries}};


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
pub fn index_template(c: &mut Criterion, i_string: &str) {
    let files = fs::read_dir("../../data.nosync/");
    

    for dir in files.unwrap() {
        let file_path = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file_path[46..file_path.len()-4];

        let config = Config {
            file_path: file_path.to_owned(),
            indexno: i_string.to_string(),
        };

        c.bench_function(&format!("indexing index {} {}",i_string,filesize),|b| b.iter(|| {
            config.to_index()
        }) );
    }
}

pub fn indexing_7(c: &mut Criterion) {
    index_template(c, "7_0");
}



pub fn indexing_8_0(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        c.bench_function(&format!("indexing index 8_0 {}",filesize),|b| b.iter(|| {
            index::Index::index8(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "8_0".to_string()})
        }) );
}
}

pub fn indexing_9_1(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        c.bench_function(&format!("indexing index 9_1 {}",filesize),|b| b.iter(|| {
            index::Index::index9_1(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "9_0".to_string()})
        }) );
}
}

pub fn indexing_9_0(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        c.bench_function(&format!("indexing index 9_0 {}",filesize),|b| b.iter(|| {
            index::Index::index9_0(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "9_0".to_string()})
        }) );
}
}
// Timing search times

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

        if filesize != "100MB"{
            continue;
        }

        let ast_vec = gen_a_lot_of_runs_bool(file.clone(), 1000);
        let index = index::Index::index8(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "8".to_string()}).unwrap();
        let mut depth = -1;

        for depth_vec in ast_vec {
            depth += 1;
            c.bench_function(&format!("searching index 8_0 in file {}, depth {}", filesize, depth), |b| b.iter(|| {
            for ast in &depth_vec {
                index.vec_to_articlelist(index.evaluate_syntax_tree_naive(*ast.clone()));
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
                index.vec_to_articlelist(index.evaluate_syntax_tree_demorgan(*ast.clone()));
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
                index.vec_to_articlelist(index.evaluate_syntax_tree_binary_search(*ast.clone()));
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
                index.vec_to_articlelist(index.evaluate_syntax_tree_hybrid(*ast.clone()));
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

pub fn find_word_9_0(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let word_vec = gen_a_lot_of_runs_tries(file.clone(), 1000,false);

        let index = index::Index::index9_0(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "9".to_string()}).unwrap();

        c.bench_function(&format!("Find word 9_0 {}", filesize), |b| b.iter(|| {
            for word in &word_vec {
                let query = Query {
                    search_string: word.to_owned(),
                    search_type: index::SearchType::PrefixSearch
                };
                index.search(&query);
            }
        }));
    } 
}

pub fn find_word_9_1(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let word_vec = gen_a_lot_of_runs_tries(file.clone(), 1000,false);
        let index = index::Index::index9_1(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "9".to_string()}).unwrap();

        c.bench_function(&format!("Find word 9_1 {}", filesize), |b| b.iter(|| {
            for word in &word_vec {
                let query = Query {
                    search_string: word.to_owned(),
                    search_type: index::SearchType::PrefixSearch
                };
                index.search(&query);
            }
        }));
    } 
}

pub fn prefix_search_index_9_0(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let word_vec:Vec<String> = gen_a_lot_of_runs_tries(file.clone(), 1000,true);
        let index = index::Index::index9_0(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "9".to_string()}).unwrap();
        
        c.bench_function(&format!("prefix search 9_0 in file {}", filesize), |b| b.iter(|| {
            for word in &word_vec {
                let query = Query {
                    search_string: word.to_owned(),
                    search_type: index::SearchType::PrefixSearch
                };
                index.search(&query);
            }
        }));
    } 
}

pub fn prefix_search_index_9_1(c: &mut Criterion) {
    let files = fs::read_dir("../../data.nosync/");

    for dir in files.unwrap() {
        let file = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file[46..file.len()-4];

        let word_vec:Vec<String> = gen_a_lot_of_runs_tries(file.clone(), 1000,true);
        let index = index::Index::index9_1(&rustsearch::helpers::Config {file_path : file.clone(), indexno : "9".to_string()}).unwrap();
        
        c.bench_function(&format!("prefix search 9_1 in file {}", filesize), |b| b.iter(|| {
            for word in &word_vec {
                let query = Query {
                    search_string: word.to_owned(),
                    search_type: index::SearchType::PrefixSearch
                };
                index.search(&query);
            }
        }));
    } 
}

//criterion_group!(benches,indexing_7,indexing_8_0,indexing_9_1,indexing_9_0,searching_index_7_0,searching_index_8_0,searching_index_8_1,searching_index_8_2,searching_index_8_3,searching_index_8_4,find_word_9_0,find_word_9_1,prefix_search_index_9_0,prefix_search_index_9_1);
criterion_group!(benches,find_word_9_0,prefix_search_index_9_0);

criterion_main!(benches);