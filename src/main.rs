use server::Server;
use http::Request;
use http::Method;
mod server; //looks in the server.rs file
mod http;


fn main() {

    let server = Server::new("localhost:8080".to_string());
    server.run();
}
