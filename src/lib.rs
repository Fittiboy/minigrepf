use std::error::Error;
use std::fs;

pub struct Config<'a> {
    query: &'a String,
    file_paths: &'a [String],
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            let config = Config {
                query: &args[1],
                file_paths: &args[2..],
            };
            return Ok(config);
        } else {
            return Err("Expected a regex pattern followed by one or more file names");
        };
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_paths[0])
        .map_err(|err| format!("Error reading file \"{}\": {}", config.file_paths[0], err))?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
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
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
