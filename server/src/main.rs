use axum::{response::{IntoResponse}, routing::{ get, post }, Router};
use tower_http::services::{ServeDir};

#[tokio::main]
async fn main(){
    let addr = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server started on {addr:?}");
    axum::serve(listener,router()).await.unwrap();
}

fn router() -> Router {
    let static_files = ServeDir::new("./static");
    Router::new()
        .route("/login", post(login))
        .fallback_service(static_files)
}

async fn login()
-> impl IntoResponse {
    // match fs::read_to_string("../static/index.html") {
    //     Ok(content) => Html(content),
    //     Err(e) => Html(format!("Error : {}", e)),
    // }
    "Nice try ;)"
}

