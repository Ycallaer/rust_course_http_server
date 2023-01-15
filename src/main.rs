#![allow(dead_code)] //disables compiler telling us certain code is not used

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;
use std::env;

mod server; //looks in the server.rs file
mod http;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path); //read environment variable
    println!("Public path {}", public_path);
    let server = Server::new("localhost:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}
