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

    println!("Find one event:\n");
    find_by_pk::<EventModel>(connection, "f186b680-237d-449d-ad66-ad91c4e53d3d");
    find_by_pk::<EventModel>(connection, "f186b680-237d-449d-ad66-ad91c4e53d4e");
    println!();

    println!("Find all events:\n");
    find_all::<EventModel>(connection);
    println!();

    println!("Find all extra events:\n");
    find_all::<EventExtraModel>(connection);
    println!();
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
