use std::fs;
use std::collections::HashMap;

const DEFAULT_FLAG: &str = "flag{none}";
const DEFAULT_FAIL_MSG: &str = "none";
const DEFAULT_CMD: &str = "exit 1";
const DEFAULT_PORT: u16 = 8080;

pub struct Config{
    pub flag: String,
    pub fail_msg: String,
    pub cmd: String,
    pub port: u16,
}

fn parse_hash_map(input: &str) -> HashMap<String, String>{
    input
    .lines()
    .filter_map(|line|{
        let line = line.trim();

        // Skip empty lines and comments denoted by a #.
        if line.is_empty() || line.starts_with('#'){
            return None;
        }

        let (key, value) = line.split_once(':')?;
        Some((key.trim().to_string(), value.trim().to_string()))
    })
    .collect()
}

impl Config{
    pub fn new() -> Config{
        Config{ flag: String::from(DEFAULT_FLAG), fail_msg: String::from(DEFAULT_FAIL_MSG), cmd: String::from(DEFAULT_CMD), port: DEFAULT_PORT }
    }

    pub fn load(path: &str) -> Config{
        let content = fs::read_to_string(path).expect("ERROR: Failed to read from configuration file.");

        let map = parse_hash_map(&content);

        let flag = match map.get("flag"){
            Some(v) => v,
            None => { println!("WARNING: Flag not defined. Using default."); DEFAULT_FLAG },
        };

        let fail_msg = match map.get("fail_msg"){
            Some(v) => v,
            None => { println!("WARNING: Error message not defined. Using default."); DEFAULT_FAIL_MSG },
        };

        let cmd = match map.get("cmd"){
            Some(v) => v,
            None => { println!("WARNING: CMD not defined. Using default."); DEFAULT_CMD },
        };

        let port = match map.get("port"){
            Some(v) => match v.parse::<u16>(){
                Ok(v) => v,
                Err(_) => { println!("WARNING: Invalid port number \"{}\". Using default.", v); DEFAULT_PORT },
            },
            None => { println!("WARNING: Port not defined. Using default."); DEFAULT_PORT },
        };

        Config{ flag: flag.to_string(), fail_msg: fail_msg.to_string(), cmd: cmd.to_string(), port: port }
    }

    pub fn print(&self){
        println!("Configuration:\n\tflag: {}\n\tfail_msg: {}\n\tcmd: {}\n\tport: {}", self.flag, self.fail_msg, self.cmd, self.port);
    }
}