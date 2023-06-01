// program that checks the sha256 of a file and then compares the output against the original sha256
// produces a response to the user in the console.

use std::env;
use std::process;
use boosha256sum::Config;
use std::time::{Instant};

fn main() {
    let cli_input: Vec<String> = env::args().collect();

    let config = Config::build(&cli_input).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(0);
    });

    let now = Instant::now();
    match boosha256sum::run(&config){
        Ok(digest) => {
            let elapsed_time = now.elapsed().as_millis();
            println!("{:?} hash computed in {} milliseconds.", config.args_opts.get("-i").unwrap(), elapsed_time);

            match config.args_opts.get("-f") {
                Some(output_file) => { 
                    match boosha256sum::write_to_file(&output_file.as_str(), digest.as_str()){
                        Ok(_) => {
                            println!("Saved to file.");
                        }
                        Err(e) => {
                            eprintln!("Error returned: {e}");
                            process::exit(1);
                        }
                    }
                }
                None => { println!("{}: {:?}", config.command.to_uppercase(), digest);}
             }
        },
        Err(e) => {
            eprintln!("Error returned: {e}");
            process::exit(1);
        }
    }

    
    
}
