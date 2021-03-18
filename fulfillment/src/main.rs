use houseflow_db::{Database, DatabaseOptions};


#[tokio::main]
async fn main() -> Result<(), String> {
    let options = DatabaseOptions::from_env().unwrap();
    let db = Database::connect(options).await
        .map_err(|err| err.to_string())?;
    println!("Hello, world!");

    Ok(())
}
