use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args)?;
    println!(
        "Searching for {} in {}...",
        config.query,
        config.file_paths.join(", ")
    );

    let contents = fs::read_to_string(&config.file_paths[0])?;

    println!("With text:\n{}", contents);

    return Ok(());
}

struct Config<'a> {
    query: &'a String,
    file_paths: &'a [String],
}

fn parse_config(args: &[String]) -> Result<Config, Box<dyn Error>> {
    let config;

    if args.len() > 2 {
        config = Config {
            query: &args[1],
            file_paths: &args[2..],
        };
    } else {
        return Err("Expected a regex pattern followed by one or more file names".into());
    };

    return Ok(config);
}
