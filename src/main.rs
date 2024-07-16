use std::error::Error;
use axum::{Router, routing::get, response::Html};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
		.route("/", get(root));

	let listener = tokio::net::TcpListener::bind("127.0.0.1:3030").await?;
	println!("listening on http://{}", listener.local_addr().unwrap());
	axum::serve(listener, app).await?;

	Ok(())
}

async fn root() -> Html<&'static str> {
	Html("<h1>Hello, World!</h1>")
}