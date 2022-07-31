
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::error::Error;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Service {
    host: String,
    timeout: u32,
    id: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Api {
    path: String,
    method: String,
    id: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct APIConfig {
    service : Service,
    api_list : Vec<Api>,
}

pub fn read_config(path : String) -> Option<Box<APIConfig>> {
    println!("path is:{}", path);
    let result = File::open(path);
    let mut content = String::from("");
    if result.is_ok() {
        let mut file = result.unwrap();
        file.read_to_string(&mut content);

        let decoded: APIConfig = toml::from_str(&content).unwrap();
        println!("connent is {}", content);
        Some(Box::new(decoded))
    } else {
        println!("{}", result.err().unwrap());
        None
    }
}

