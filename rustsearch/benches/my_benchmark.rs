// https://bheisler.github.io/criterion.rs/book/getting_started.html
#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rustsearch::index;


// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

pub fn indexing_8_file_100_kb(c: &mut Criterion) {
    c.bench_function("index 8 indexing 100 KB", |b| b.iter(|| {
        index::Index::index8_0(&rustsearch::helpers::Config {file_path : "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(), indexno : "8_0".to_string()})
    }));
}

pub fn indexing_8_file_5_mb(c: &mut Criterion) {
    c.bench_function("index 8 indexing 5 MB", |b| b.iter(|| {
        index::Index::index8_0(&rustsearch::helpers::Config {file_path : "data/WestburyLab.wikicorp.201004_5MB.txt".to_string(), indexno : "8_0".to_string()})
    }));
}



criterion_group!(benches, indexing_8_file_100_kb, indexing_8_file_5_mb);
criterion_main!(benches);

