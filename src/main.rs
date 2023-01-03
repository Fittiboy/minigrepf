use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let (query, file_paths) = parse_config(&args)?;
    println!("Searching for {} in {}...", query, file_paths.join(", "));

    let contents = fs::read_to_string(&file_paths[0])?;

    println!("With text:\n{}", contents);

    return Ok(());
}

fn parse_config(args: &[String]) -> Result<(&String, &[String]), Box<dyn Error>> {
    let query;
    let file_paths;

    if args.len() > 2 {
        query = &args[1];
        file_paths = &args[2..];
    } else {
        return Err("Expected a regex pattern followed by one or more file names".into());
    };

    return Ok((query, file_paths));
}
