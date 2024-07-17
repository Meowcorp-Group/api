use axum::{response::Html, routing::get, Router};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;
use util::state::AppState;

mod media;
use media::nowplaying;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app_state = Arc::new(Mutex::new(AppState::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/media/nowplaying/:username/ws", get(nowplaying::nowplaying_handler))
        .route(
            "/media/nowplaying/:username",
            get(nowplaying::nowplaying_get),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await?;
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
