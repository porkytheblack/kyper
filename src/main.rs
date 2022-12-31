#![warn(dead_code)]

pub mod server;
pub mod http;
pub mod kyper_handler;

pub use server::Server;
pub use kyper_handler::KyperHandler;

/*
    @todo - Add functionality to handle address in use
*/

fn main() {
    let server_listening_on = String::from("127.0.0.1:8080");
    let port = &server_listening_on[10..];

    println!("PORT:: {}", port);

    let kyper_server = Server::new(server_listening_on);

    kyper_server.run(&mut KyperHandler{});


}
