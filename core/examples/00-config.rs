fn main() -> elephantry::Result {
    let config = config::Config::builder()
        .add_source(config::Environment::with_prefix("DATABASE"))
        .build()?;

    let elephantry = elephantry::Pool::from_config(&config.try_deserialize()?)?;

    elephantry.ping()
}
