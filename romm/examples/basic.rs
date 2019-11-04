#[derive(Clone, Debug)]
struct Event
{
    uuid: uuid::Uuid,
    name: String,
    visitor_id: u32,
    properties: serde_json::Value,
    browser: serde_json::Value,
}

impl romm::Entity for Event
{
    fn from(data: &std::collections::HashMap<&'static str, (postgres::types::Type, Vec<u8>)>) -> Self
    {
        use postgres::types::FromSql;

        Self {
            uuid: {
                let (t, content) = data.get("uuid")
                    .expect("Unable to find 'uuid' field");
                FromSql::from_sql(t, content)
                    .expect("Unable to convert 'uuid' field of type 'Uuid' from SQL")
            },
            name: {
                let (t, content) = data.get("name")
                    .expect("Unable to find 'name' field");
                FromSql::from_sql(t, content)
                    .expect("Unable to convert 'name' field of type 'String' from SQL")
            },
            visitor_id: {
                let (t, content) = data.get("visitor_id")
                    .expect("Unable to find 'visitor_id' field");
                FromSql::from_sql(t, content)
                    .expect("Unable to convert 'visitor_id' field of type 'u32' from SQL")
            },
            properties: {
                let (t, content) = data.get("properties")
                    .expect("Unable to find 'properties' field");
                FromSql::from_sql(t, content)
                    .expect("Unable to convert 'properties' field of type 'json' from SQL")
            },
            browser: {
                let (t, content) = data.get("browser")
                    .expect("Unable to find 'browser' field");
                FromSql::from_sql(t, content)
                    .expect("Unable to convert 'browser' field of type 'json' from SQL")
            },
        }
    }
}

struct EventModel;

impl romm::Model for EventModel
{
    type Entity = Event;
    type RowStructure = EventStructure;
}

#[derive(Clone, Debug)]
struct EventExtra
{
    uuid: uuid::Uuid,
    name: String,
    visitor_id: u32,
    properties: serde_json::Value,
    browser: serde_json::Value,
    os: Option<String>,
}

impl romm::Entity for EventExtra
{
    fn from(data: &std::collections::HashMap<&'static str, (postgres::types::Type, Vec<u8>)>) -> Self
    {
        let event = <Event as romm::Entity>::from(data);

        Self {
            uuid: event.uuid,
            name: event.name,
            visitor_id: event.visitor_id,
            properties: event.properties,
            browser: event.browser,
            os: match data.get("os") {
                Some((t, content)) => Some(
                    postgres::types::FromSql::from_sql(t, content)
                        .expect("Unable to convert 'os' field of type 'String' from SQL")
                ),
                None => None,
            },
        }
    }
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
    let romm = romm::Romm::new("postgres://sanpi@localhost/romm")
        .unwrap();
    let connection = romm.get();

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
