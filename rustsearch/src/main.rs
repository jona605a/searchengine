use std::collections::{HashMap, HashSet};
use std::{env, io, process};

use rustsearch::helpers::*;
use rustsearch::index::Index;

#[allow(unused_variables)]

fn main() {
    // word_freq()

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    

    if config.indexno == "7" {
        let index = Index::index7(&config).expect("Config should have valid filename");
    } else if config.indexno == "8" {
        let index = Index::index8(&config).expect("Config should have valid filename");
    } else {
        panic!("Invalid index number given. Accepts the following: 7, 8.");
    }
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
