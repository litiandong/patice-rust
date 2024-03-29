use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Usage: cargo run [query] [filename]");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename })
    }

    pub fn show(&self) {
        println!("Searching for {}", self.query);
        println!("In file {}", self.filename);
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.filename)?;

    let results = search(&cfg.query, &contents);
    for line in results {
        println!("{line}");
    }
    Ok(())

}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
            );
    }
}

