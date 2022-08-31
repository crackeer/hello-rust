
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

use std::{
    fs::metadata,
    fs::{read_dir, DirEntry},
};

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
        if  file.read_to_string(&mut content).is_err() {
           return None
        }

        let decoded: APIConfig = toml::from_str(&content).unwrap();
        println!("connent is {}", content);
        Some(Box::new(decoded))
    } else {
        println!("{}", result.err().unwrap());
        None
    }
}

pub fn get_md_list(dir: String) -> Vec<String> {
    let mut dir_vec: Vec<String> = Vec::new();
    let mut list: Vec<String> = Vec::new();
    println!("{}", dir);
    dir_vec.push(dir);
   
    let mut cur_index: usize = 0;
    while cur_index < dir_vec.len() {
        let entry = read_dir(dir_vec.get(cur_index).unwrap().to_string());
        if let Ok(data) = entry {
            for item in data.into_iter() {
                if let Ok(dataEntry) = item {
                    if let Ok(abc) = dataEntry.metadata() {
                        if abc.is_dir() {
                            dir_vec.push(dataEntry.path().to_str().unwrap().clone().to_string());
                        } else {
                            let file = dataEntry.path().to_str().unwrap().to_string();
                            if file.ends_with(".md") {
                                list.push(file);
                            }
                        }
                    }
                }
            }
        }
        cur_index = cur_index + 1;
    }
    println!("{}", list.join(","));
    list
}

