use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};

fn main() {
    read_and_exec();
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

pub fn read_and_exec() -> Result<(), Error> {
    let file = File::open("./sample.json");
    let mut file = match file {
        Err(err) => {
            return Err(err);
        }
        Ok(f) => f,
    };

    let mut content = String::from("");
    file.read_to_string(&mut content);

    let data: Value = serde_json::from_str(&content).unwrap();
    let input: Value = json!({});
    if let Some(result) = cat_cat(data, input) {
        print!("{}", result.to_string())
    }
    Ok(())
}

pub fn read() -> Result<(), Error> {
    let file = File::open("./sample.json");
    let mut file = match file {
        Err(err) => {
            return Err(err);
        }
        Ok(f) => f,
    };

    let mut content = String::from("");
    file.read_to_string(&mut content);

    let data: Value = serde_json::from_str(&content).unwrap();
    println!("{}{}", data, data.is_object());

    if let Some(obj) = data.as_object() {
        for (key, val) in obj.into_iter() {
            println!("{}{}", key, val);
            if val.is_string() {}
        }
    }

    Ok(())
}

fn get_real_value(input: Value, schema: String) -> Option<Value> {
    let parts = schema.split(",");
    for val in parts.into_iter() {
        println!("{}get_real_value", val)
    }
    Some(json!(1))
}

fn cat_cat(value: Value, input: Value) -> Option<Value> {
    if value.is_string() {
        return get_real_value(input, value.to_string());
    }

    if !value.is_object() {
        return Some(value);
    }

    let mut ret: Value = json!({});

    if let Some(obj) = value.as_object() {
        for (key, val) in obj.into_iter() {
            if let Some(tmp) = cat_cat(val.clone(), input.clone()) {
                ret.as_object_mut()
                    .unwrap()
                    .insert(String::from(key), tmp.clone());
            }
        }
    }

    Some(ret)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Method {
    pub host: String,
    pub timeout: u32,
    pub comment: String,
}

impl Method {
    fn new(host: String, timeout: u32, comment: String) -> Self {
        Method {
            host: host,
            timeout: timeout,
            comment,
        }
    }
}

fn default_0() -> String {
    "0.0".to_string()
}
