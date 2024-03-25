use bollard::errors::Error;
use bollard::image::ListImagesOptions;
use bollard::Docker;
use bollard::API_DEFAULT_VERSION;
use serde_json::json;
use std::default::Default;
use std::env;
use std::path::Path;


#[tokio::main]
async fn main() {
    match get_docker() {
        Err(err) => {
            println!("{}", err)
        }
        Ok(ref docker) => {
            println!("{}", String::from("OK"));
            match docker.list_images(Some(ListImagesOptions::<String> {
                    all: true,
                    ..Default::default()
                })).await {
                Ok(list) => {
                    println!("{}", list.len())
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    }
}

fn get_docker() -> Result<Docker, String> {
    /* 
    if let Ok(docker) = Docker::connect_with_local_defaults() {
        println!("connect_with_local_defaults");
        return Ok(docker);
    }*/

    match get_user_docker_sock() {
        Ok(dir) => {
            let user_path = Path::new(&dir).join(&".docker/run/docker.sock");
            println!("{}", user_path.to_str().unwrap());
            if let Ok(docker) =
                Docker::connect_with_socket(user_path.to_str().unwrap(), 120, API_DEFAULT_VERSION)
            {
               
                return Ok(docker);
            }
            return Err(String::from("Sim"));
        }
        Err(err) => Err(err),
    }
}

fn get_user_docker_sock() -> Result<String, String> {
    match env::home_dir() {
        Some(path) => Ok(path.to_str().unwrap().to_string()),
        None => Err(String::from("no home directory")),
    }
}
