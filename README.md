# Introduction
This is a Bachelor thesis project deleloping various search engines to search through snapshots of Wikipedia articles from 2010. The whole Bachelor thesis can be read in the LaTeX folder. A general project description and more data files can be found at http://www2.compute.dtu.dk/searchengineproject/ .

The project has been developed using Java (openjdk 20.0.1) and Rust (rustc 1.67.1). 

# How to run the Basic part in Java
The basic part can be run with or without make.

To run the basic part with make, install make from https://www.gnu.org/software/make/#download

When using make, run the command below from the SearchEngine directory

``` sh
$ make run IDX=5 SIZE="100KB"
```
To run the basic part without make run the commands below from the SearchEngine directory:

``` sh
$ javac src/*.java -d bin
$ java --enable-preview -cp bin Index5 Data/WestburyLab.wikicorp.201004_100KB.txt
```
To search with another Index or in another file, simply change the number or filename in the respective places. 

# How to run the Advanced part in Rust
An important prerequisite for your system is to intall rust with the rustup tool: https://www.rust-lang.org/tools/install

The main function reads the data files from the folder rustsearch/data/ . There are already the files, 100KB and 5MB, but other documents may be added. 

In order to run the Main TUI and use the various search engines implemented, go to the rustsearch directory and run the command below:
``` sh
$ cargo run --release
```

In order to run all our tests, the following command may be used
``` sh
$ cargo test --release -- --test-threads=1
```



