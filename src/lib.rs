use std::env;
use std::error::Error;
use std::fs;

pub struct Config<'a> {
    query: &'a String,
    file_paths: &'a [String],
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            let config = Config {
                query: &args[1],
                file_paths: &args[2..],
                ignore_case: env::var("IGNORE_CASE").is_ok(),
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
    let search_fn = if config.ignore_case {
        search_case_insensitive
    } else {
        search
    };

    for line in search_fn(&config.query, &contents) {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
