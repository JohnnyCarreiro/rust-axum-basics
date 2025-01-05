#![allow(unused)]

pub use self::error::{Error, Result};
use std::net::SocketAddr;

use axum::{
	extract::{Path, Query},
	middleware,
	response::{Html, IntoResponse, Response},
	routing::{get, get_service},
	Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
	let app = Router::new()
		.merge(rotes_hello())
		.merge(web::routes_login::routes())
		.layer(middleware::map_response(main_response_mapper))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static());

	// region:   --- Start Server
	let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
	let listener = tokio::net::TcpListener::bind(&addr)
		.await
		.unwrap();
	println!("->> LISTENING on {:?}\n", listener.local_addr());
	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
	// endregion --- Start Server
}

async fn main_response_mapper(res: Response) -> Response {
	println!("->> {:<12} -  main_response_mapper ", "RES_MAPPER");
	println!();
	res
}

// region:   --- Routes
fn routes_static() -> Router {
	Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
// endregion:   --- Routes

// region:   --- Routes Hello
fn rotes_hello() -> Router {
	Router::new()
		.route("/hello", get(hanlder_hello))
		.route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}
// e.g., `hello?name=johnny`
async fn hanlder_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
	println!("->> {:<12} -  handler_hello - {params:?}", "HANDLER");
	let name = params.name.as_deref().unwrap_or("world");
	Html(format!("<h1>Hello, {name} </h1>"))
}
// e.g., `hello2/johnny`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
	println!("->> {:<12} -  handler_hello2 - {name:?}", "HANDLER");
	let name = name.as_str();
	Html(format!("<h1>Hello, {name} </h1>"))
}
// endregion:   --- Routes Hello
