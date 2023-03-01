use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub indexno: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let indexno = args[2].clone();

        Ok(Config { file_path, indexno })
    }
}


pub fn read_file_to_string(file_path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}




