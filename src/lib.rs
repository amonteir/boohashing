use std::fs;
use std::error::Error;
use sha2::{Sha256, Sha512, Digest};
use std::io::{BufReader, Read, Write};
use std::collections::HashMap;
use std::time::{Instant};


#[derive(Debug)]
pub struct Config {
    pub program: &'static str,
    pub command: &'static str,
    pub args_opts_map: HashMap<String, String>,
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

        let mut args_opts_map: HashMap<String, String> = HashMap::new();
        let mut index: usize = 2;
        
        while index < cli_input.len() {
            match cli_input[index].as_str() {
                "-i" => {
                    if args_opts_map.contains_key("-i"){
                        return Err("option '-i' is only allowed once.\nFor syntax help, type --help.");
                    }
                    else{
                        args_opts_map.insert(String::from("-i"), cli_input[index+1].clone());
                        index += 1;
                    }
                }
                "-f" => {
                    if args_opts_map.contains_key("-f"){
                        return Err("option '-f' is only allowed once.\nFor syntax help, type --help.");
                    }
                    else{
                        args_opts_map.insert(String::from("-f"), cli_input[index+1].clone());
                        index += 1;
                    }
                }
                _ => return Err("option not available. Use --help to check options available.")
            }
            index += 1;
        }

        if !args_opts_map.contains_key("-i") {
            return Err("no input file provided. \nTry again with option '-i' <file_name> or type --help.")
        }
 
        Ok(Config {
            program: cli_program_name,
            command: cli_command,
            args_opts_map: args_opts_map,
        })
    }
}

fn write_to_file(file_name: &str, content: &str) -> Result<(), Box<dyn Error>> {

    let mut file = fs::File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn compute_hash<T: Digest + Clone>(config: &Config, input_hasher: T) -> Result<String, Box<dyn Error>> {

    let mut hasher = input_hasher.clone();
    let file_name = config.args_opts_map.get("-i").ok_or("missing '-i' option")?;
    let file = fs::File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1024];

    let now = Instant::now();
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    let result: digest::generic_array::GenericArray<u8, <T as Digest>::OutputSize> = hasher.finalize();
    println!("{:?} hash computed in {} milliseconds.", config.args_opts_map.get("-i").unwrap(), now.elapsed().as_millis());
    let result_hex = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
    Ok(result_hex)
}

fn process_outputs(config: &Config, digest: &String) -> Result<(), Box<dyn Error>>{

    match config.args_opts_map.get("-f") {
        Some(output_file) => { 
            match write_to_file(&output_file.as_str(), digest.as_str()){
                Ok(_) => {
                    println!("Saved to file.");
                }
                Err(e) => {
                    eprintln!("error saving to file: {e}");
                }
            }
        }
        None => { println!("{}: {:?}", config.command.to_uppercase(), digest);}
    }
    Ok(())
}

fn compute_hash_and_process_output<T: Digest + Clone>(config: &Config, hasher: T) -> Result<String, Box<dyn Error>>  {
    
    let hash = compute_hash(config, hasher)?;
    process_outputs(config, &hash)?;     
    Ok(hash)
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    match config.command {
        "sha512" =>
        {
            let hasher = Sha512::new();
            if let Err(e) = compute_hash_and_process_output(&config, hasher){
                eprintln!("Error: {}", e);
            }  
        }
        "sha256" => 
        {
            let hasher = Sha256::new();
            if let Err(e) = compute_hash_and_process_output(&config, hasher){
                eprintln!("Error: {}", e);
            }            
        }
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Error: hashing function not implemented. Type --help to check available hashing functions."
            )));
        }        
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build() {
        let mut config_args_opts_map: HashMap<String, String> = HashMap::new();
        config_args_opts_map.insert(String::from("-i"), String::from("sample1.txt"));
        let test_config: Config = Config{
            program: "boohashing",
            command: "sha256",
            args_opts_map: config_args_opts_map,
        };

        let test_args: Vec<String> = vec![  String::from("program_name"), 
                                            String::from("sha256"), 
                                            String::from("-i"), 
                                            String::from("sample1.txt")];

        assert_eq!( test_config.args_opts_map.get("-i"), 
                    Config::build(&test_args).unwrap().args_opts_map.get("-i")
        );
    }
    #[test]
    fn sha256_run(){
        let mut config_args_opts_map: HashMap<String, String> = HashMap::new();
        config_args_opts_map.insert(String::from("-i"), String::from("sample1.txt"));
        let test_config1: Config = Config{
            program: "boohashing",
            command: "sha256",
            args_opts_map: config_args_opts_map,
        };
        let hasher = Sha256::new();
        assert_eq!(
            "bc658c641ef71739fb9995bded59b21150bbff4367f6e4e4c7934b489b9d2c00", 
            compute_hash_and_process_output(&test_config1, hasher).unwrap());

        let mut config_args_opts_map2: HashMap<String, String> = HashMap::new();
        config_args_opts_map2.insert(String::from("-i"), String::from("sample2.pdf"));
        let test_config2: Config = Config{
            program: "boohashing",
            command: "sha256",
            args_opts_map: config_args_opts_map2,
        };        
        let hasher = Sha256::new();
        assert_eq!(
            "040ee8ec7061270d226637daa21e3ad7087457448f7dcf32b14dff9747b138ac", 
            compute_hash_and_process_output(&test_config2, hasher).unwrap());
    }
}