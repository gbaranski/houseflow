use mijia::MijiaSession;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (_, session) = MijiaSession::new().await?;
    let mut stream = session.event_stream().await?;
    while let Some(device) = stream.next().await {
        println!("{:?}", device);
    }

    Ok(())
}