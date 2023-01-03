use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query;
    let file_paths;

    if args.len() > 2 {
        query = &args[1];
        file_paths = &args[2..];
        println!("Searching for {} in {}...", query, file_paths.join(", "))
    } else {
        panic!("Expected a regex pattern followed by one or more filenames!")
    };

    let contents = fs::read_to_string(&file_paths[0])
        .expect(&format!("Could not read file {}", &file_paths[0]));

    println!("With text:\n{}", contents);
}
