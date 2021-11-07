use futures::StreamExt;
use chrono::DateTime;
use chrono::Utc;
use mijia::MijiaSession;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (_, session) = MijiaSession::new().await?;
    session.bt_session.start_discovery().await?;
    let mut stream = session.event_stream().await?;
    // tokio::spawn(async move {
    //     while let Some(event) = stream.next().await {
    //         println!("{:?}", event);
    //     }
    // });
    loop {
        println!("Sensors:");
        let sensors= session.get_sensors().await?;
        for sensor in sensors {
            println!("Connecting to {} ({})", sensor.mac_address, sensor.id);
            if let Err(e) = session.bt_session.connect(&sensor.id).await {
                println!("Failed to connect to {}: {:?}", sensor.mac_address, e);
            } else {
                let sensor_time: DateTime<Utc> = session.get_time(&sensor.id).await?.into();
                let temperature_unit = session.get_temperature_unit(&sensor.id).await?;
                let comfort_level = session.get_comfort_level(&sensor.id).await?;
                let history_range = session.get_history_range(&sensor.id).await?;
                let last_record = session.get_last_history_record(&sensor.id).await?;
                println!(
                    "Time: {}, Unit: {}, Comfort level: {}, Range: {:?} Last value: {}",
                    sensor_time, temperature_unit, comfort_level, history_range, last_record
                );
                let history = session.get_all_history(&sensor.id).await?;
                println!("History: {:?}", history);
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}