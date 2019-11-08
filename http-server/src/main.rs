#![warn(rust_2018_idioms)]

use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use http::{header::HeaderValue, Request, Response, StatusCode};
use serde::Serialize;
use std::{env, error::Error, fmt, io};
use tokio::{
    codec::{Decoder, Encoder, Framed},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8989".to_string());

    let mut incoming = TcpListener::bind(&addr).await?.incoming();

    println!("Listening on: {}", addr);

    while let Some(Ok(stream)) = incoming.next().await {
        tokio::swawn(async move {
            if let Err(e) = process(stream).await {
                println!("failed to process connection: error = {}", e);
            }
        })
    }

    Ok(())
}

async fn process(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut transport = Framed::new(stream, Http);

    while let Some(request) = transport.next().await {
        match request {
            Ok(request) => {
                let response = respond(request).await?;
                transport.send(response).await?;
            }
            Err(e) => return Err(e.into()),
        }
    }
}
