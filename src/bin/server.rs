use std::time::Duration;

use futures::prelude::*;
use test_simple::Message;
use tokio::net::TcpListener;
use tokio_serde::formats::*;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let length_delimited = Framed::new(socket, LengthDelimitedCodec::new());
        let mut deserialized = tokio_serde::SymmetricallyFramed::new(
            length_delimited,
            SymmetricalJson::<Message>::default(),
        );
        tokio::spawn(async move {
            while let Some(mut msg) = deserialized.try_next().await.unwrap() {
                tokio::time::sleep(Duration::from_micros(1000)).await;
                println!("GOT: {:?}", msg);
                msg.server_increment();
                deserialized.send(msg).await.unwrap_or(());
            }
        });
    }
}
