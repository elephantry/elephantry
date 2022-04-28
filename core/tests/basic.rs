#[cfg(feature = "derive")]
include!("entity_derive.rs");

#[cfg(not(feature = "derive"))]
include!("entity.rs");

fn main() -> elephantry::Result {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/elephantry".to_string());
    let elephantry = elephantry::Pool::default().add_default("elephantry", &database_url)?;

    let count = elephantry.count_where::<EventModel>("name = $1", &[&"pageview"])?;
    println!("Count events: {count}");
    assert_eq!(count, 7);
    println!();

    println!("Find one event:\n");
    find_by_pk::<EventModel>(&elephantry, "f186b680-237d-449d-ad66-ad91c4e53d3d")?;
    println!();

    println!("Find all events:\n");
    find_all::<EventModel>(&elephantry)?;
    println!();

    println!("Find all extra events:\n");
    find_all::<EventExtraModel>(&elephantry)?;
    println!();

    println!("Insert one row:\n");
    let new_event = Event::<String> {
        uuid: None,
        name: "purchase".to_string(),
        visitor_id: Some(15),
        #[cfg(feature = "json")]
        properties: serde_json::json!({ "amount": 200 }),
        #[cfg(not(feature = "json"))]
        properties: "{\"amount\": 200}".to_string(),
        #[cfg(feature = "json")]
        browser: serde_json::json!({ "name": "Firefox", "resolution": { "x": 1280, "y": 800 } }),
        #[cfg(not(feature = "json"))]
        browser: "{ \"name\": \"Firefox\", \"resolution\": { \"x\": 1280, \"y\": 800 } }"
            .to_string(),
        generic: None,
    };
    let mut entity = insert_one::<EventModel>(&elephantry, &new_event)?;
    println!();

    println!("Update one row:\n");
    entity.name = "pageview".to_string();
    let entity =
        update_one::<EventModel>(&elephantry, &elephantry::pk!(uuid => entity.uuid), &entity)?;
    assert_eq!(&entity.name, "pageview");
    println!();

    println!("Delete one row\n");
    elephantry.delete_one::<EventModel>(&entity)?;
    let uuid = entity.uuid.unwrap();
    assert!(elephantry
        .find_by_pk::<EventModel>(&elephantry::pk! {uuid => uuid})?
        .is_none());
    assert_eq!(
        elephantry.exist_where::<EventModel>("uuid = $1", &[&uuid])?,
        false
    );

    let count = elephantry.model::<EventModel>().count_uniq_visitor()?;
    assert_eq!(count, 4);
    println!("Count uniq visitor: {count}");

    Ok(())
}

fn find_by_pk<M>(connection: &elephantry::Connection, uuid: &str) -> elephantry::Result
where
    M: elephantry::Model,
    M::Entity: std::fmt::Debug,
{
    #[cfg(feature = "uuid")]
    let uuid = uuid::Uuid::parse_str(uuid).unwrap();
    let event = connection.find_by_pk::<EventModel>(&elephantry::pk!(uuid))?;

    match event {
        Some(event) => println!("{event:?}"),
        None => println!("Event '{uuid}' not found"),
    };

    Ok(())
}

fn find_all<M>(connection: &elephantry::Connection) -> elephantry::Result
where
    M: elephantry::Model,
    M::Entity: std::fmt::Debug,
{
    let events = connection.find_all::<M>(None)?;

    if events.is_empty() {
        println!("No events in database.");
    } else {
        for event in events {
            println!("{event:?}");
        }
    }

    Ok(())
}

fn insert_one<M>(
    connection: &elephantry::Connection,
    entity: &M::Entity,
) -> elephantry::Result<M::Entity>
where
    M: elephantry::Model,
    M::Entity: std::fmt::Debug,
{
    let new_entity = connection.insert_one::<M>(&entity)?;

    println!("{new_entity:?}");

    Ok(new_entity)
}

fn update_one<M>(
    connection: &elephantry::Connection,
    pk: &std::collections::HashMap<&str, &dyn elephantry::ToSql>,
    entity: &M::Entity,
) -> elephantry::Result<M::Entity>
where
    M: elephantry::Model,
    M::Entity: std::fmt::Debug,
{
    let new_entity = connection.update_one::<M>(pk, entity)?;

    println!("{new_entity:?}");

    Ok(new_entity.unwrap())
}
