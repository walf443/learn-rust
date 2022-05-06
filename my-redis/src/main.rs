use bytes::Bytes;
use mini_redis::Command::Set;
use mini_redis::{Command, Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use dashmap::DashMap;
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<DashMap<String, Bytes>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6380").await.unwrap();

    let db = Arc::new(DashMap::new());

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let res = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            _ => Frame::Null,
        };

        connection.write_frame(&res).await.unwrap();
    }
}
