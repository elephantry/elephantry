#[cfg(feature = "romm-derive")]
include!("entity_derive.rs");

#[cfg(not(feature = "romm-derive"))]
include!("entity.rs");

fn main()
{
    let romm = romm::Romm::new()
        .add_default("romm", "postgres://sanpi@localhost/romm")
        .unwrap();
    let connection = romm.default()
        .unwrap();

    let count = connection.count_where::<EventModel>("name = $1", &[&"pageview"]).unwrap();
    println!("Count events: {}", count);
    assert_eq!(count, 7);
    println!();

    println!("Find one event:\n");
    find_by_pk::<EventModel>(connection, "f186b680-237d-449d-ad66-ad91c4e53d3d");
    println!();

    println!("Find all events:\n");
    find_all::<EventModel>(connection);
    println!();

    println!("Find all extra events:\n");
    find_all::<EventExtraModel>(connection);
    println!();

    println!("Insert one row:\n");
    let new_event = Event {
        uuid: None,
        name: "purchase".to_string(),
        visitor_id: 15,
        properties: serde_json::json!({ "amount": 200 }),
        browser: serde_json::json!({ "name": "Firefox", "resolution": { "x": 1280, "y": 800 } }),
    };
    let entity = insert_one::<EventModel>(connection, &new_event);
    println!();

    println!("Update one row:\n");
    let entity = update_one::<EventModel>(connection, &entity, &maplit::hashmap! {
        "name" => &"pageview" as &dyn romm::pq::ToSql,
    });
    assert_eq!(&entity.name, "pageview");
    println!();

    println!("Delete one row\n");
    connection.delete_one::<EventModel>(&entity).unwrap();
    let uuid = entity.uuid.unwrap();
    assert!(connection.find_by_pk::<EventModel>(&romm::pk!{uuid => uuid}).unwrap().is_none());
    assert_eq!(connection.exist_where::<EventModel>("uuid = $1", &[&entity.uuid.unwrap()]).unwrap(), false);
}

fn find_by_pk<M>(connection: &romm::Connection, uuid: &str) where M: romm::Model, M::Entity: std::fmt::Debug
{
    let uuid = uuid::Uuid::parse_str(uuid)
        .unwrap();
    let event = connection.find_by_pk::<EventModel>(&romm::pk!(uuid))
        .unwrap();

    match event {
        Some(event) => println!("{:?}", event),
        None => println!("Event '{}' not found", uuid),
    };
}

fn find_all<M>(connection: &romm::Connection) where M: romm::Model, M::Entity: std::fmt::Debug
{
    let events = connection.find_all::<M>()
        .unwrap();

    if events.is_empty() {
        println!("No events in database.");
    }
    else {
        for event in events {
            println!("{:?}", event);
        }
    }
}

fn insert_one<M>(connection: &romm::Connection, entity: &M::Entity) -> M::Entity where M: romm::Model, M::Entity: std::fmt::Debug
{
    let new_entity = connection.insert_one::<M>(&entity)
        .unwrap();

    println!("{:?}", new_entity);

    new_entity
}

fn update_one<M>(connection: &romm::Connection, entity: &M::Entity, data: &std::collections::HashMap<&str, &dyn romm::pq::ToSql>) -> M::Entity where M: romm::Model, M::Entity: std::fmt::Debug
{
    let new_entity = connection.update_one::<M>(&entity, &data)
        .unwrap();

    println!("{:?}", new_entity);

    new_entity
}
