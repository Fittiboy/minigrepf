//! # minigrepf
//!
//! `minigrepf` is a tiny command line tool built as an exercise from the book
//! "The Rust Programming Language," chapter 12.
//!
//! ## Usage
//!
//! ```shell
//! $ ls
//! poem.txt minigrepf
//!
//! $ cat poem.txt
//! I'm nobody! Who are you?
//! Are you nobody, too?
//! Then there's a pair of us - don't tell!
//! They'd banish us, you know.
//!
//! How dreary to be somebody!
//! How public, like a frog
//! To tell your name the livelong day
//! To an admiring bog!
//!
//! $ ./minigrepf to poem.txt
//! Are you nobody, too?
//! How dreary to be somebody!
//!
//! $ IGNORE_CASE=1 ./minigrepf to poem.txt
//! Are you nobody, too?
//! How dreary to be somebody!
//! To tell your name the livelong day
//! To an admiring bog!
//! ```

use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Runs the command line tool with the Config inferred from command line options
///
/// # Examples
///
/// ```
/// let config = minigrepf::Config::build(
///     vec![
///         String::from("./minigrepf"),
///         String::from("frog"),
///         String::from("poem.txt"),
///     ]
///     .into_iter(),
/// );
/// let result = minigrepf::run(config.unwrap());
/// assert_eq!(result.unwrap(), ());
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)
        .map_err(|err| format!("Error reading file \"{}\": {}", config.file_path, err))?;
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
