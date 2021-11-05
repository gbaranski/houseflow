use mijia::MijiaSession;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (_, session) = MijiaSession::new().await?;
    session.bt_session.start_discovery().await?;
    let mut stream = session.event_stream().await?;
    while let Some(event) = stream.next().await {
        println!("{:?}", event);
    }

    Ok(())
}