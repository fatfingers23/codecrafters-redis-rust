mod server;
mod resp;
mod commands;
mod cache;
use server::Server;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    let mut server = Server::new().unwrap();
    server.run();
} //mod server;
