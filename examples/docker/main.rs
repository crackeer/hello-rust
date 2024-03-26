use bollard::errors::Error;
use bollard::image::ListImagesOptions;
use bollard::Docker;
use bollard::API_DEFAULT_VERSION;
use futures::stream::StreamExt;
use serde_json::json;
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    list_docker_images().await;
    save_docker_images().await;
}

async fn list_docker_images() {
    match get_docker() {
        Err(err) => {
            println!("{}", err.to_string())
        }
        Ok(docker) => {
            println!("{}", String::from("OK"));

            match docker
                .list_images(Some(ListImagesOptions::<String> {
                    all: true,
                    filters: HashMap::new(),
                    digests: true,
                }))
                .await
            {
                Ok(list) => {
                    for item in list.iter() {
                        println!("{}", serde_json::json!(item));
                    }
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    }
}

async fn save_docker_images() {
    match get_docker() {
        Err(err) => {
            println!("{}", err.to_string())
        }
        Ok(docker) => {
            println!("{}", String::from("OK"));
            let mut tmp_file = File::create(&"/tmp/realsee-shepherd-images.tar").unwrap();
            let mut stream = docker.export_image(&"realsee-shepherd-svc:local");
            while let Some(item) = stream.next().await {
                if let Ok(data) = item {
                    _ = tmp_file.write_all(&data);
                }
            }
        }
    }
}

fn get_docker() -> Result<Docker, Error> {
    if let Some(sock_path) = get_user_docker_sock_path() {
        return Docker::connect_with_socket(sock_path.to_str().unwrap(), 120, API_DEFAULT_VERSION);
    }
    Docker::connect_with_local_defaults()
}

fn get_user_docker_sock_path() -> Option<PathBuf> {
    match env::home_dir() {
        Some(path) => {
            let tmp_path = path.join(&".docker/run/docker.sock");
            if fs::metadata(tmp_path.as_path()).is_ok() {
                return Some(tmp_path);
            }
            None
        }
        None => None,
    }
}
