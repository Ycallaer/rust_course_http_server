#![allow(dead_code)] //disables compiler telling us certain code is not used

use server::Server;
use http::Request;
use http::Method;
use website_handler::WebsiteHandler;

mod server; //looks in the server.rs file
mod http;
mod website_handler;

fn main() {

    let server = Server::new("localhost:8080".to_string());
    server.run(WebsiteHandler);
}
