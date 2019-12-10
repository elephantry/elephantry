#[derive(Clone, Debug, romm_derive::Entity)]
struct Event
{
    uuid: uuid::Uuid,
    name: String,
    visitor_id: u32,
    properties: serde_json::Value,
    browser: serde_json::Value,
}

struct EventModel;

impl romm::Model for EventModel
{
    type Entity = Event;
    type RowStructure = EventStructure;
}

#[derive(Clone, Debug, romm_derive::Entity)]
struct EventExtra
{
    uuid: uuid::Uuid,
    name: String,
    visitor_id: u32,
    properties: serde_json::Value,
    browser: serde_json::Value,
    os: Option<String>,
}

struct EventExtraModel;

impl romm::Model for EventExtraModel
{
    type Entity = EventExtra;
    type RowStructure = EventStructure;

    fn create_projection() -> romm::Projection
    {
        Self::default_projection()
            .set_field("os", romm::Row {
                content: "%:browser:% ->> 'os'",
                ty: postgres::types::VARCHAR,
            })
    }
}

struct EventStructure;

impl romm::RowStructure for EventStructure
{
    fn relation() -> &'static str
    {
        "public.event"
    }

    fn primary_key() -> &'static [&'static str]
    {
        &["uuid"]
    }

    fn definition() -> std::collections::HashMap<&'static str, romm::Row>
    {
        maplit::hashmap! {
            "uuid" => romm::Row {
                content: "%:uuid:%",
                ty: postgres::types::UUID,
            },
            "name" => romm::Row {
                content: "%:name:%",
                ty: postgres::types::VARCHAR,
            },
            "visitor_id" => romm::Row {
                content: "%:visitor_id:%",
                ty: postgres::types::INT4,
            },
            "properties" => romm::Row {
                content: "%:properties:%",
                ty: postgres::types::JSON,
            },
            "browser" => romm::Row {
                content: "%:browser:%",
                ty: postgres::types::JSON,
            },
        }
    }
}

fn main()
{
    env_logger::init();

    let romm = romm::Romm::new()
        .add_default("romm", "postgres://sanpi@localhost/romm")
        .unwrap();
    let connection = romm.default()
        .unwrap();

    find_all::<EventModel>(connection);
    find_all::<EventExtraModel>(connection);
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
