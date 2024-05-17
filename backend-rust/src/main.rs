use std::time::Duration;

use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::{net::TcpListener, time::sleep};

use crate::app::ApplicationContext;

mod app;
mod auth;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/2", get(index2))
        .with_state(ApplicationContext {});
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(app): State<ApplicationContext>) -> Result<String, (StatusCode, String)> {
    // app.login().await;
    println!("start");
    sleep(Duration::from_secs(20)).await;
    println!("done");
    Ok("hello world".into())
}

async fn index2(State(app): State<ApplicationContext>) -> Result<String, (StatusCode, String)> {
    // app.login().await;
    println!("start 2");
    sleep(Duration::from_secs(5)).await;
    println!("done 2");
    Ok("hello world".into())
}
