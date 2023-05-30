// https://bheisler.github.io/criterion.rs/book/getting_started.html
// #![allow(non_snake_case)]
use criterion::{criterion_group, criterion_main, Criterion};
use rustsearch::index::gen_query::gen_a_lot_of_runs_full_text;
use rustsearch::index::{Query, SearchType};

use std::fs;

use rustsearch::helpers::{read_file_to_string, Config};
use rustsearch::index::{
    
    gen_query::{gen_a_lot_of_runs_bool, gen_a_lot_of_runs_tries},
};

// Timing reading files
pub fn opening_and_reading_file(c: &mut Criterion) {
    let file100kb = "data/WestburyLab.wikicorp.201004_100KB.txt";
    let file5mb = "data/WestburyLab.wikicorp.201004_5MB.txt";
    c.bench_function("Opening and reading file 100 KB", |b| {
        b.iter(|| {
            read_file_to_string(&file100kb.to_string()).unwrap();
        })
    });
    c.bench_function("Opening and reading file 5 MB", |b| {
        b.iter(|| {
            read_file_to_string(&file5mb.to_string()).unwrap();
        })
    });
}

// Timing the indexing on different files
pub fn index_template(c: &mut Criterion, i_string: &str) {
    let files = fs::read_dir("data/");

    for dir in files.unwrap() {
        if dir.as_ref().unwrap().path().is_dir() {
            continue;
        }

        let file_path = dir.unwrap().path().into_os_string().into_string().unwrap();

        if &file_path[0..9] != "data/West" {
            continue;
        }

        let filesize = match file_path.rsplit_once('_') {
            Some((_, suffix)) => {
                suffix
                    .split_once('.')
                    .expect("What kind of file doesn't end in a file extension?")
                    .0
            }
            None => continue,
        };

        let config = Config {
            file_path: file_path.to_owned(),
            indexno: i_string.to_string(),
        };

        c.bench_function(&format!("indexing index {} {}", i_string, filesize), |b| {
            b.iter(|| config.to_index())
        });
    }
}

pub fn indexing_7_0(c: &mut Criterion) {
    index_template(c, "7_0");
}

pub fn indexing_8_0(c: &mut Criterion) {
    index_template(c, "8_0");
}

pub fn indexing_9_0(c: &mut Criterion) {
    index_template(c, "9_0");
}

pub fn indexing_9_1(c: &mut Criterion) {
    index_template(c, "9_1");
}

pub fn indexing_10_0(c: &mut Criterion) {
    index_template(c, "10_0");
}

pub fn indexing_10_1(c: &mut Criterion) {
    index_template(c, "10_1");
}

pub fn indexing_11_0(c: &mut Criterion) {
    index_template(c, "11_0");
}

pub fn indexing_11_1(c: &mut Criterion) {
    index_template(c, "11_1");
}

// Timing search times
pub fn bool_searching_template(c: &mut Criterion, i_string: &str) {
    let files = fs::read_dir("../../data.nosync/");

    let boolean_searchtype = match i_string {
        "7_0" => " ",
        "8_0" => "Naive",
        "8_1" => "DeMorgan",
        "8_2" => "BinarySearch",
        "8_3" => "Hybrid",
        "8_4" => "Bitvecs",
        _ => panic!(),
    };

    for dir in files.unwrap() {
        let file_path = dir.unwrap().path().into_os_string().into_string().unwrap();
        let filesize = &file_path[46..file_path.len() - 4];
  
        let ast_vec: Vec<Vec<String>> = gen_a_lot_of_runs_bool(file_path.clone(), 1000);

        let config = Config {
            file_path: file_path.to_owned(),
            indexno: i_string.to_string(),
        };
        let index = config.to_index().unwrap();

        let mut depth = 0;

        for depth_vec in ast_vec {
            depth += 1;

            c.bench_function(
                &format!(
                    "searching index {} in file {}, depth {}",
                    i_string, filesize, depth
                ),
                |b| {
                    b.iter(|| {
                        for word in &depth_vec {
                            let query = Query {
                                search_string: word.clone(),
                                search_type: SearchType::BooleanSearch(
                                    boolean_searchtype.to_string(),
                                ),
                            };

                            index.search(&query);
                        }
                    })
                },
            );
        }
    }
}

pub fn searching_index_7_0(c: &mut Criterion) {
    bool_searching_template(c, "7_0");
}

pub fn searching_index_8_0(c: &mut Criterion) {
    bool_searching_template(c, "8_0");
}

pub fn searching_index_8_1(c: &mut Criterion) {
    bool_searching_template(c, "8_1");
}

pub fn searching_index_8_2(c: &mut Criterion) {
    bool_searching_template(c, "8_2");
}

pub fn searching_index_8_3(c: &mut Criterion) {
    bool_searching_template(c, "8_3");
}

pub fn searching_index_8_4(c: &mut Criterion) {
    bool_searching_template(c, "8_4");
}

pub fn prefix_search_template(c: &mut Criterion, i_string: &str, prefix_bool: bool) {
    let files = fs::read_dir("data/");
    let searchtype_string = match prefix_bool {
        false => "Find word",
        true => "prefix search 9_0 in file",
    };

    let search_type = match i_string {
        "8_0" => SearchType::BooleanSearch("Naive".to_string()),
        _ => SearchType::PrefixSearch
    };

    for dir in files.unwrap() {

        if dir.as_ref().unwrap().path().is_dir() {
            continue;
        }

        let file_path = dir.unwrap().path().into_os_string().into_string().unwrap();

        if &file_path[0..9] != "data/West" {
            continue;
        }
        
        let filesize = file_path
            .rsplit_once('_')
            .unwrap()
            .1
            .split_once('.')
            .unwrap()
            .0;

        let word_vec = gen_a_lot_of_runs_tries(file_path.clone(), 1000, prefix_bool);
    
        let config = Config {
            file_path: file_path.to_owned(),
            indexno: i_string.to_string(),
        };
        let index = config.to_index().unwrap();

        c.bench_function(
            &format!("{} {} {}", searchtype_string, i_string, filesize),
            |b| {
                b.iter(|| {
                    for word in &word_vec {
                        let query = Query {
                            search_string: word.to_owned(),
                            search_type: search_type.to_owned(),
                        };
                        index.search(&query);
                    }
                })
            },
        );
    }
}

pub fn find_word_8_0(c: &mut Criterion) {
    prefix_search_template(c, "8_0", false)
}

pub fn find_word_9_0(c: &mut Criterion) {
    prefix_search_template(c, "9_0", false)
}

pub fn find_word_9_1(c: &mut Criterion) {
    prefix_search_template(c, "9_1", false)
}

pub fn prefix_search_index_9_0(c: &mut Criterion) {
    prefix_search_template(c, "9_0", true)
}

pub fn prefix_search_index_9_1(c: &mut Criterion) {
    prefix_search_template(c, "9_1", true)
}

pub fn full_text_searching_template(c: &mut Criterion, i_string: &str) {
    let files = fs::read_dir("data/");

    let full_text_searchtype = match i_string {
        "10_0" => SearchType::ExactSearch("KMP".to_string()),
        "10_1" => SearchType::ExactSearch("BoyerMoore".to_string()),
        "10_2" => SearchType::ExactSearch("ApostolicoGiancarlo".to_string()),
        "11_0" => SearchType::FuzzySearch,
        "11_1" => SearchType::ExactSearch("TripleBoyerMoore".to_string()),
        _ => panic!(),
    };

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

        let full_text_queries: Vec<String> = gen_a_lot_of_runs_full_text(file_path.clone(), 1000);
        let config = Config {
            file_path: file_path.to_owned(),
            indexno: i_string.to_string(),
        };
        let index = config.to_index().unwrap();
        c.bench_function(&format!("Long Query Full text {} {}", i_string, filesize), |b| {
            b.iter(|| {
                for sentence in &full_text_queries {
                    let query = Query {
                        search_string: sentence.to_owned(),
                        search_type: full_text_searchtype.to_owned(),
                    };

                    index.search(&query);
                }
            })
        });
    }
}

pub fn full_text_search_10_0(c: &mut Criterion) {
    full_text_searching_template(c, "10_0")
}

pub fn full_text_search_10_1(c: &mut Criterion) {
    full_text_searching_template(c, "10_1")
}

pub fn full_text_search_10_2(c: &mut Criterion) {
    full_text_searching_template(c, "10_2")
}

pub fn full_text_search_11_0(c: &mut Criterion) {
    full_text_searching_template(c, "11_0")
}

pub fn full_text_search_11_1(c: &mut Criterion) {
    full_text_searching_template(c, "11_1")
}

pub fn kmp_vs_boyer_moore(c: &mut Criterion) {
    let file_size = "5MB";

    let file_path = format!("data/WestburyLab.wikicorp.201004_{}.txt", file_size);

    let full_text_queries: Vec<String> = gen_a_lot_of_runs_full_text(file_path.clone(), 1000);
    let config = Config {
        file_path: file_path.to_owned(),
        indexno: "10_0".to_string(),
    };

    let index = config.to_index().unwrap();

    // c.bench_function(&format!("Bench KMP {}", file_size), |b| {
    //     b.iter(|| {
    //         for sentence in &full_text_queries {
    //             let query = Query {
    //                 search_string: sentence.to_owned(),
    //                 search_type: SearchType::ExactSearch("KMP".to_string()),
    //             };
    //             index.search(&query);
    //         }
    //     })
    // });

    c.bench_function(&format!("Bench BoyerMoore {}", file_size), |b| {
        b.iter(|| {
            for sentence in &full_text_queries {
                let query = Query {
                    search_string: sentence.to_owned(),
                    search_type: SearchType::ExactSearch("BoyerMoore".to_string()),
                };
                index.search(&query);
            }
        })
    });

    c.bench_function(&format!("Bench ApostolicoGiancarlo {}", file_size), |b| {
        b.iter(|| {
            for sentence in &full_text_queries {
                let query = Query {
                    search_string: sentence.to_owned(),
                    search_type: SearchType::ExactSearch("ApostolicoGiancarlo".to_string()),
                };
                index.search(&query);
            }
        })
    });

    // c.bench_function(&format!("Bench Dumide 100KB"), |b| {
    //     b.iter(|| {
    //         for sentence in &full_text_queries {
    //             let query = Query {
    //                 search_string: sentence.to_owned(),
    //                 search_type: SearchType::ExactSearch("dumide".to_string()),
    //             };

    //             index.search(&query);
    //         }
    //     })
    // });
}

//criterion_group!(benches,indexing_7,indexing_8_0,indexing_9_1,indexing_9_0,searching_index_7_0,searching_index_8_0,searching_index_8_1,searching_index_8_2,searching_index_8_3,searching_index_8_4,find_word_9_0,find_word_9_1,prefix_search_index_9_0,prefix_search_index_9_1);
criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = indexing_9_1,indexing_9_0
);

criterion_main!(benches);
