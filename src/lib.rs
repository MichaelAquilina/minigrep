use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


pub struct Config {
    pub query: String,
    pub filename: String,
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>  {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}