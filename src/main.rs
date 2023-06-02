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

 
    match boohashing::run(&config){
        Ok(_) => {
            //println!("{:?} hash computed in {} milliseconds.", config.args_opts.get("-i").unwrap(), elapsed_time);
        },
        Err(e) => {
            eprintln!("Error returned: {e}");
            process::exit(1);
        }
    }
}
