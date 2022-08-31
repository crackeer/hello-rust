use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    extract::{Path, Query},
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/proxy", get(proxy))
        .route("/files", get(md_list));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn proxy(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let path = params.get("path");
    if path.is_some() {
        let data = api::read_config(path.unwrap().to_string());
        if data.is_some() {
            //println!("Some Data:{}", data);
            (StatusCode::CREATED, Json(data))
        } else {
            (StatusCode::CREATED, Json(data))
        }
    } else {
        (StatusCode::CREATED, Json(None))
    }
    
}

async fn md_list(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let path = params.get("dir");
    if path.is_some() {
        let data = api::get_md_list(path.unwrap().to_string());
            //println!("Some Data:{}", data);
        (StatusCode::CREATED, Json(data))
    } else {
        (StatusCode::CREATED, Json(vec![]))
    }
    
}


// the input to our `create_user` handler
// the input to our `create_user` handler
#[derive(Deserialize, Debug)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}