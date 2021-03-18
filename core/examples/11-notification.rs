const CHANNEL_NAME: &str = "channel_name";

fn main() -> elephantry::Result {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    elephantry.listen(CHANNEL_NAME)?;
    elephantry.notify(CHANNEL_NAME, Some("payload"))?;
    listen(&elephantry)?;

    elephantry.unlisten(CHANNEL_NAME)?;
    elephantry.notify(CHANNEL_NAME, Some("payload"))?;
    listen(&elephantry)?;

    Ok(())
}

fn listen(elephantry: &elephantry::Connection) -> elephantry::Result {
    while let Some(notify) = elephantry::v2::connection::notifies(elephantry)? {
        dbg!(notify);
    }

    Ok(())
}
