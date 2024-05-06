//! The entry point for the TCP server.
use futures::{sink::SinkExt, StreamExt};
use nanoservices_utils::networking::codec::BincodeCodec;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;

use kernel::TestContractHandler;
mod routing;
use routing::handle_contract_routes;


#[tokio::main]
async fn main() {
    let bind_server = "0.0.0.0:8001";
    let listener = TcpListener::bind(bind_server).await.unwrap();
    println!("Server listening on port 8001");

    while let Ok((socket, _)) = listener.accept().await {
        let mut framed = Framed::new(socket, BincodeCodec::<TestContractHandler>::new());

        while let Some(result) = framed.next().await {
            match result {
                Ok(data) => {
                    println!("Received: {:?}", data);
                    let response = handle_contract_routes(data).await.unwrap();
                    framed.send(response).await.unwrap();
                    break;
                },
                Err(e) => {
                    eprintln!("Error processing data: {}", e);
                    break;
                }
            }
        }
    }
}
