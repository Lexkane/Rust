extern crate tokio;

use tokio::prelude::*;
use tokio::net::TcpListener;
use tokio::io::copy;

pub fn main() -> Result<(), Box<std::error::Error>> {
    let addr = "127.0.0.1:3000".parse()?;
    let listen_socket = TcpListener::bind(&addr)?;
    let server = listen_socket
        .incoming()
        .map_err(|e| eprintln!("Error accepting socket: {}", e))
        .for_each(|socket| {
            let (reader, writer) = socket.split();
            let handle_conn =
                copy(reader, writer)
                .map(|copy_info| println!("Finished, bytes copied: {:?}", copy_info))
                .map_err(|e| {
                    eprintln!("Error echoing: {}", e);
                })
                ;
            tokio::spawn(handle_conn)
        })
        ;
    tokio::run(server);
    Ok(())
}