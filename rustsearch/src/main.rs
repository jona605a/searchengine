use std::collections::HashMap;
use std::{env, fs, io};

use rustsearch::helpers::*;
use rustsearch::index::{
    Index, Query, Search,
    SearchType::{self, *},
};

#[allow(unused_variables)]

fn main() {
    let args: Vec<String> = env::args().collect();

    user_dialog();

    // profile_memory_old(&config);
}

fn user_dialog() {
    let mut loaded_indices: HashMap<String, Box<dyn Search>> = HashMap::new();
    let index_names = ["6", "7", "8", "9.0", "9.1", "10", "11"];

    let available_search_types: HashMap<String, Vec<SearchType>> = HashMap::from_iter([
        ("6".to_string(), vec![SingleWordSearch]),
        (
            "7".to_string(),
            vec![SingleWordSearch, BooleanSearch("".to_string())],
        ),
        (
            "8".to_string(),
            vec![
                SingleWordSearch,
                BooleanSearch("Naive".to_string()),
                BooleanSearch("DeMorgan".to_string()),
                BooleanSearch("BinarySearch".to_string()),
                BooleanSearch("Hybrid".to_string()),
                BooleanSearch("Bitvecs".to_string()),
            ],
        ),
        ("9.0".to_string(), vec![SingleWordSearch, PrefixSearch]),
        ("9.1".to_string(), vec![SingleWordSearch, PrefixSearch]),
        (
            "10".to_string(),
            vec![
                SingleWordSearch,
                ExactSearch("KMP".to_string()),
                ExactSearch("BoyerMoore".to_string()),
                ExactSearch("ApostolicoGiancarlo".to_string()),
            ],
        ),
        (
            "11".to_string(),
            vec![
                SingleWordSearch,
                FuzzySearch,
                ExactSearch("TripleBoyerMoore".to_string()),
            ],
        ),
    ]);

    // let config = Config::build(&[
    //     "".to_string(),
    //     "data/WestburyLab.wikicorp.201004_100KB.txt".to_string(),
    //     "9_0".to_string(),
    // ]);
    // loaded_indices.insert("90".to_string(), Box::new(Index::index9_0(&config).unwrap()));

    println!("Welcome to the Search Engine");

    let mut fileinput = String::new();
    let filenames: Vec<String> = fs::read_dir("data/")
        .unwrap()
        .filter(|dir| !dir.as_ref().unwrap().path().is_dir())
        .map(|dir| dir.unwrap().path().into_os_string().into_string().unwrap())
        .collect();
    'selectfile: loop {
        println!("Please select the file to search in, among the files in data/ . The following are available:");
        for filename in &filenames {
            println!("  {}", filename);
        }
        println!("  exit");
        println!("Simply input the filesize (e.g. \"100KB\") : ");
        // Read user input
        io::stdin()
            .read_line(&mut fileinput)
            .expect("Failed to read line");
        if fileinput.trim() == "exit" {
            return;
        } else {
            for filename in &filenames {
                if filename.contains(fileinput.trim()) {
                    fileinput = filename.to_owned();
                    break 'selectfile;
                }
            }
            fileinput = String::new();
            continue 'selectfile;
        }
    }

    'selectindex: loop {
        println!("Please select the Index to use. Already loaded indices are marked with a *.");
        for idx_name in index_names {
            let ready = loaded_indices.contains_key(idx_name);
            println!("{} {}", if ready { "*" } else { " " }, idx_name);
        }
        println!("  exit");
        let mut indexinput = String::new();
        io::stdin()
            .read_line(&mut indexinput)
            .expect("Failed to read line");
        indexinput = indexinput.trim().to_string();
        if indexinput == "exit" {
            break 'selectindex;
        } else if !index_names.contains(&&indexinput[..]) {
            continue 'selectindex;
        }
        // Index chosen

        if !loaded_indices.contains_key(&indexinput) {
            // Need to index a file
            println!("Indexing. Please wait...");

            let config = Config {
                file_path: fileinput.clone(),
                indexno: indexinput.clone(),
            };
            match config.to_index() {
                Ok(idx) => loaded_indices.insert(indexinput.clone(), idx),
                Err(_) => {
                    println!("Unexpected error. Try again");
                    continue;
                }
            };
        }
        let current_index = loaded_indices.get(&indexinput).unwrap();
        println!("Using index {} to search in file {}", indexinput, fileinput);

        let user_search_type;
        'selectsearchtype: loop {
            println!("Please enter query type. The following are supported:");
            let ast = match available_search_types.get(&indexinput) {
                None => {
                    println!("Unexpected. Index supports no search types. ");
                    continue 'selectindex;
                }
                Some(searchtypes) => searchtypes,
            };

            for (i, st) in ast.iter().enumerate() {
                println!("{}) {}", i, st);
            }

            println!("Simply enter the number: ");
            let mut searchtype = String::new();
            io::stdin()
                .read_line(&mut searchtype)
                .expect("Failed to read line");
            searchtype = searchtype.trim().to_string();
            let i = match searchtype.parse::<usize>() {
                Ok(x) => x,
                Err(_) => {
                    println!("Input couldn't be parsed as an integer");
                    continue 'selectsearchtype;
                }
            };
            user_search_type = ast[i].clone();
            break 'selectsearchtype;
        }

        // Ready to get the query!!
        loop {
            println!("\nSearch type {} selected. Please enter a query (\"back\" to go back):", &user_search_type);
            let mut user_search_string = String::new();
            io::stdin()
                .read_line(&mut user_search_string)
                .expect("Failed to read line");
            user_search_string = user_search_string.trim().to_string();
            if user_search_string == "back" {
                continue 'selectindex;
            }

            let query = Query {
                search_string: user_search_string.clone(),
                search_type: user_search_type.clone(),
            };

            let articletitles = current_index.search(&query);
            println!(
                "\nThe query \"{}\" was found in the following articles: \n{:?}",
                user_search_string, articletitles
            );
        }
    }
}

#[allow(dead_code)]
fn old_user_dialog() {
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

        println!(
            "Searching for \"{}\" using {search_type}",
            query_string.trim()
        );
        let _query = Query {
            search_string: query_string.trim().to_string(),
            search_type,
        };
        // println!("Found in articles: {:?}\n", index.search(&query));
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
