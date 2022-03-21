use std::env;

use std::process;

use minigrep::Config;

fn main() {
    // get the params from input.
    let args: Vec<String> = env::args().collect();

    // split the params.
    let mut config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

