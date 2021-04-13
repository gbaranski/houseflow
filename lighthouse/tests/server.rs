use houseflow_lighthouse::server::Server;
use houseflow_lighthouse::session;
use std::sync::Arc;
use std::pin::Pin;
use std::cmp::min;
use std::task::{Poll, Context};
use std::io::{Cursor, Read, Write};
use tokio::io;
use io::{AsyncRead, ReadBuf};

struct MockStream {
    rx: Cursor<Vec<u8>>,
    tx: Cursor<Vec<u8>>,
}


#[tokio::test]
async fn test_connect() {
    let store = session::Store::new();
    let server = Server::new(store).await;
    // let stream = MockStream {


    // };
}
