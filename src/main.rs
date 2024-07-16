use axum::{
    extract::{Path, WebSocketUpgrade}, http::HeaderMap, response::Html, routing::get, Extension, Router
};
use std::{collections::HashMap, error::Error, sync::Arc};
use tokio::sync::RwLock;

mod media;
use media::nowplaying;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(nowplaying::NowPlayingState::new());

    let app = Router::new()
        .route("/", get(root))
        .route("/media/nowplaying/ws", get(nowplaying::nowplaying_socket))
        .route(
            "/media/nowplaying/:username",
            get(nowplaying::nowplaying_get),
        )
		.layer(Extension(state));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await?;
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
