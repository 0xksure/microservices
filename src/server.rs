use crate::handshake::greeter_server::{Greeter, GreeterServer};
use crate::handshake::{HandshakeReply, HandshakeRequest};
use tonic::transport::Server;
use tonic::{Request, Response};

pub mod handshake {
    tonic::include_proto!("index");
}

#[derive(Debug, Default)]
pub struct HandshakeService {}

#[tonic::async_trait]
impl Greeter for HandshakeService {
    async fn handshake(
        &self,
        request: Request<HandshakeRequest>,
    ) -> Result<Response<HandshakeReply>, tonic::Status> {
        log::info!("Received request: {:?}", request);
        let reply = handshake::HandshakeReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5051".parse()?;
    let handshake = HandshakeService::default();

    Server::builder()
        .add_service(GreeterServer::new(handshake))
        .serve(addr)
        .await?;

    Ok(())
}
