use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{}, got \"{}\"", err, args[1..].join(" "));
        process::exit(1);
    });

    let result = minigrep::run(config).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("{}", result)
}
