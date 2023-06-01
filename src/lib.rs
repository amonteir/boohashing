use std::fs;
use std::error::Error;
use sha2::{Sha256, Sha512, Digest};
use std::io::{BufReader, Read, Write};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub program: &'static str,
    pub command: &'static str,
    pub args_opts: HashMap<String, String>,
}

impl Config {

    pub fn build(cli_input: &[String]) -> Result<Config, &'static str> {
        if cli_input.len() <= 1{
            return Err("not enough arguments.\nConsider using option --help.");
        }
        
        let cli_program_name: &str = "boohash";

        let cli_command = match cli_input[1].to_lowercase().as_str() {
            "sha256" => "sha256",
            "sha512" => "sha512",
            _ => return Err("hashing algorithm not implemented.")
        };

        let mut args_opts: HashMap<String, String> = HashMap::new();
        let mut index: usize = 2;
        //let arg_file_input = "-i";
        //let arg_file_output = "-f";
        let mut input_file_provided: bool = false;

        while index < cli_input.len() {
            match cli_input[index].as_str() {
                "-i" => {
                    args_opts.insert(String::from("-i"), cli_input[index+1].clone());
                    index += 1;
                    input_file_provided = true;
                }
                "-f" => {
                    args_opts.insert(String::from("-f"), cli_input[index+1].clone());
                    index += 1;
                }
                _ => ()
            }
            index += 1;
        }

        if !input_file_provided {
            return Err("no input file provided. \nTry again with argument -i <file_name> or type --help.")
        }
 
        Ok(Config {
            program: cli_program_name,
            command: cli_command,
            args_opts: args_opts,
        })
    }
}

// fn generate_hash<T>(hasher: &T, file: &fs::File) -> Result<&T, Box<dyn Error>> {

//     //let file = fs::File::open(file_name)?;
//     let mut reader = BufReader::new(file);
//     //let mut hasher = Sha512::new();
//     let mut buffer = [0; 1024];

//     loop {
//         let count = reader.read(&mut buffer)?;
//         if count == 0 {
//             break;
//         }
//         hasher.update(&buffer[..count]);
//     }
//     Ok(hasher)

// }


pub fn write_to_file(file_name: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = fs::File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn run(config: &Config) -> Result<String, Box<dyn Error>> {

    let file_name = config.args_opts.get("-i").unwrap();
    let file = fs::File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];

    match config.command {
        "sha512" =>
        {
            let mut hasher = Sha512::new();
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }

            let result = hasher.finalize();
            Ok(format!("{:x}", result))
        }
        "sha256" | _ => 
        {
            let mut hasher = Sha256::new();
            loop {
                let count = reader.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }

            let result = hasher.finalize();
            Ok(format!("{:x}", result))
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build() {
        let mut config_args_opts: HashMap<String, String> = HashMap::new();
        config_args_opts.insert(String::from("-i"), String::from("sample1.txt"));
        let test_config: Config = Config{
            program: "boohashing",
            command: "sha256",
            args_opts: config_args_opts,
        };

        let test_args: Vec<String> = vec![  String::from("program_name"), 
                                            String::from("sha256"), 
                                            String::from("-i"), 
                                            String::from("sample1.txt")];

        assert_eq!( test_config.args_opts.get("-i"), 
                    Config::build(&test_args).unwrap().args_opts.get("-i")
        );
    }
    #[test]
    fn sha256_run(){
        let mut config_args_opts: HashMap<String, String> = HashMap::new();
        config_args_opts.insert(String::from("-i"), String::from("sample1.txt"));
        let test_config1: Config = Config{
            program: "boohashing",
            command: "sha256",
            args_opts: config_args_opts,
        };
        assert_eq!("bc658c641ef71739fb9995bded59b21150bbff4367f6e4e4c7934b489b9d2c00", run(&test_config1).unwrap());

        let mut config_args_opts2: HashMap<String, String> = HashMap::new();
        config_args_opts2.insert(String::from("-i"), String::from("sample2.pdf"));
        let test_config2: Config = Config{
            program: "boohashing",
            command: "sha256",
            args_opts: config_args_opts2,
        };        
        assert_eq!("040ee8ec7061270d226637daa21e3ad7087457448f7dcf32b14dff9747b138ac", run(&test_config2).unwrap());
    }
}