// program that checks the sha256 of a file and then compares the output against the original sha256
// produces a response to the user in the console.

use std::env;
use std::process;
use boohashing::Config;

fn main() {
    let cli_input: Vec<String> = env::args().collect();

    let config = Config::build(&cli_input).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(0);
    });

    if let Err(e) = boohashing::run(&config){
            eprintln!("{e}");
            process::exit(1);
    }
    
}
