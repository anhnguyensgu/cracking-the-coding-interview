use axum::{extract::State, http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

use crate::app::ApplicationContext;

mod app;
mod auth;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .with_state(ApplicationContext {});
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index(State(app): State<ApplicationContext>) -> Result<String, (StatusCode, String)> {
    app.login().await;
    Ok("hello world".into())
}
