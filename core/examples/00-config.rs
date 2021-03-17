fn main() -> elephantry::Result<()> {
    let mut config = config::Config::new();
    config.merge(config::Environment::with_prefix("DATABASE"))?;

    let elephantry = elephantry::Pool::from_config(&config.try_into()?)?;

    let msg = match elephantry.ping() {
        elephantry::PingStatus::Ok => "Ok".to_string(),
        status => format!("Connection error: {:?}", status),
    };

    println!("{}", msg);

    Ok(())
}
