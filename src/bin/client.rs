use std::time::Duration;

use futures::prelude::*;
use test_simple::Message;
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
pub async fn main() {
    let socket = TcpStream::connect("127.0.0.1:8000").await.unwrap();
    let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());
    let mut stream =
        tokio_serde::SymmetricallyFramed::new(length_delimited, SymmetricalJson::default());
    stream
        .send(Message {
            msg: "some message".to_string(),
            server_count: 0,
            client_count: 0,
        })
        .await
        .unwrap();
    loop {
        tokio::time::sleep(Duration::from_micros(1000)).await;
        let mut message: Message = stream.next().await.unwrap().unwrap();
        message.client_increment();
        stream.send(message).await.unwrap();
    }
}
