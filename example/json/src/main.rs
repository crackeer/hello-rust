use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

fn main() {
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

    /*
      let method = match(result){
        Ok(data) => data,
        Err(err) => println!("{}", err)
    };
     */

  
    

    // Parse the string of data into serde_json::Value.
    //serde_json::from_str(data);

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

