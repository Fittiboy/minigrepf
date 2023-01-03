use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{}, got \"{}\"", err, args[1..].join(" "));
        process::exit(1);
    });

    let result = run(&config).unwrap_or_else(|err| {
        println!("Error reading file \"{}\": {}", config.file_paths[0], err);
        process::exit(1);
    });

    println!("{}", result)
}

fn run(config: &Config) -> Result<String, Box<dyn Error>> {
    println!(
        "Searching for {} in {}...",
        config.query,
        config.file_paths.join(", ")
    );

    let contents = fs::read_to_string(&config.file_paths[0])?;
    return Ok(format!("With text:\n{}", contents));
}

struct Config<'a> {
    query: &'a String,
    file_paths: &'a [String],
}

impl<'a> Config<'a> {
    fn build(args: &[String]) -> Result<Config, &'static str> {
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
