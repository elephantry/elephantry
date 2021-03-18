fn main() -> elephantry::Result {
    let mut config = config::Config::new();
    config.merge(config::Environment::with_prefix("DATABASE"))?;

    let elephantry = elephantry::Pool::from_config(&config.try_into()?)?;

    elephantry.ping()
}
