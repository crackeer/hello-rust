use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::io::{Error, Read};
use std::fs::File;

fn main() {

    read();
    /* 
    let data = r#"
        {
            "host": "John Doe",
            "timeout": 43,
            "comment" : "Simple"
        }"#;
    println!("{}", data);

    let result : Result<Method>  = serde_json::from_str(data);
    if result.is_ok() {
        let method = result.unwrap();
        println!("{}", method.comment)
    } else {
        println!("{}", result.err().unwrap())
    }
    */

}

pub fn read() -> Result<(), Error>{

    let file = File::open("./sample.json");
    let mut file = match file {
        Err(err) => {
            return Err(err);
        }
        Ok(f) => f
    };

    let mut content = String::from("");
    file.read_to_string(&mut content);

    let data : Value = serde_json::from_str(&content).unwrap();
    println!("{}{}", data, data.is_object());
    
    if let Some(obj) = data.as_object() {
        for (key, val) in obj.into_iter(){
            println!("{}{}", key, val)
        }
    }
    
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Method {
    pub host : String,
    pub timeout : u32,
    pub comment: String,
}

impl Method {
    fn new(host : String, timeout : u32, comment : String) -> Self {
        Method  {
            host : host,
            timeout : timeout,
            comment
        }
    }
}

fn default_0() -> String {
    "0.0".to_string()
}

