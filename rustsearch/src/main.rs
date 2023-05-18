use std::{env, fs, io, process};

use rustsearch::helpers::*;
use rustsearch::index::{Index, Query, Search, SearchType::*};

#[allow(unused_variables)]

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    user_dialog(config.to_index().unwrap())

    //separate_file_to_seperate_articles(&config);

    // profile_memory_old(&config);
}

#[allow(dead_code)]
fn user_dialog(index: Box<dyn Search>) {
    let search_types = [
        SingleWordSearch,
        BooleanSearch(String::new()),
        PrefixSearch,
        ExactSearch(String::new()),
        FuzzySearch,
    ];
    loop {
        println!("Please input your query. (exit to stop)");

        let mut query_string = String::new();
        let mut query_type = String::new();

        io::stdin()
            .read_line(&mut query_string)
            .expect("Failed to read line");
        if query_string.trim() == "exit" {
            break;
        }

        println!("Select the search type for your index");

        for (i, st) in search_types.iter().enumerate() {
            println!("{}) {}", i, st)
        }

        io::stdin()
            .read_line(&mut query_type)
            .expect("Failed to read line");

        let search_type = search_types[query_type.trim().parse::<usize>().unwrap()].clone();

        println!("Searching for \"{}\" using {search_type}", query_string.trim());
        let query = Query {
            search_string: query_string.trim().to_string(),
            search_type,
        };
        println!("Found in articles: {:?}\n", index.search(&query));
    }
}

#[allow(dead_code)]
fn profile_memory_old(config: &Config) {
    if config.indexno == "7" {
        let index = Index::index7(&config).expect("Config should have valid filename");
        println!("#### Rust indexing done! ####");
        index.boolean_search(&"(boot or shoe) and not sandal".to_string());
        println!("#### Rust searching done! ####");
    } else if config.indexno == "8" {
        let index = Index::index8(&config).expect("Config should have valid filename");
        println!("#### Rust indexing done! ####");
        index.boolean_search_articles_to_bitvecs(&"(boot or shoe) and not sandal".to_string());
        println!("#### Rust searching done! ####");
    } else {
        panic!("Invalid index number given. Accepts the following: 7, 8.");
    }
}

#[allow(dead_code)]
fn separate_file_to_seperate_articles(config: &Config) {
    
    let articles_iter = read_and_clean_file_to_iter(config).unwrap();

    let mut count = 0;

    for (title, contents) in articles_iter {
        if title != "" {
            let x = contents.join(" ");
            fs::write(format!("data/individual_articles/{:05}.txt", count), x).unwrap();
            count += 1;
        }
    }
}
