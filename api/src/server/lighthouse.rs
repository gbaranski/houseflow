use std::net::SocketAddr;
use h2::client;
use http::{HeaderMap, Request};
use tokio::net::TcpStream;
use std::net::ToSocketAddrs;

pub async fn connect(address: impl ToSocketAddrs) -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = address.to_socket_addrs().unwrap().next().unwrap();
    let tcp = TcpStream::connect(address).await?;
    let (mut client, h2) = client::handshake(tcp).await?;

    println!("sending request");

    let request = Request::builder()
        .uri(format!("https://{}/", address))
        .body(())
        .unwrap();

    let mut trailers = HeaderMap::new();
    trailers.insert("zomg", "hello".parse().unwrap());

    let (response, mut stream) = client.send_request(request, false).unwrap();

    // send trailers
    stream.send_trailers(trailers).unwrap();

    // Spawn a task to run the conn...
    tokio::spawn(async move {
        if let Err(e) = h2.await {
            println!("GOT ERR={:?}", e);
        }
    });

    let response = response.await?;
    println!("GOT RESPONSE: {:?}", response);

    // Get the body
    let mut body = response.into_body();

    while let Some(chunk) = body.data().await {
        println!("GOT CHUNK = {:?}", chunk?);
    }

    if let Some(trailers) = body.trailers().await? {
        println!("GOT TRAILERS: {:?}", trailers);
    }

    todo!()

}