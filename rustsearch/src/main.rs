use std::collections::{HashMap, HashSet};
use std::io;

use rustsearch::helpers::*;
use rustsearch::index::Index;

#[allow(unused_variables)]

fn main() {
    word_freq()

    // let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    // println!("In file {}", config.file_path);

    // let index = Index::index7(&config)
    //     .expect("Config should have valid filename");
    //user_dialog(&index);

    // if let Err(e) = rustsearch::run(config) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }
}

#[allow(dead_code)]
fn user_dialog(index: &Index<HashMap<String, HashSet<String>>, Option<u128>>) {
    loop {
        println!("Please input your query. (exit to stop)");

        let mut query = String::new();

        io::stdin()
            .read_line(&mut query)
            .expect("Failed to read line");
        if query == "exit\n" {
            break;
        }
        print!("Searching for {query}");
        println!(
            "Found in articles: {:?}\n",
            index.search(&query.strip_suffix('\n').unwrap().to_string())
        );
    }
}
