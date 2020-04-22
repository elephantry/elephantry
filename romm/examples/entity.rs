#[derive(Clone, Debug)]
struct Event
{
    uuid: Option<uuid::Uuid>,
    name: String,
    visitor_id: i32,
    properties: serde_json::Value,
    browser: serde_json::Value,
}

impl romm::Entity for Event
{
    fn from(tuple: &romm::pq::Tuple) -> Self
    {
        Self {
            uuid: tuple.get("uuid"),
            name: tuple.get("name"),
            visitor_id: tuple.get("visitor_id"),
            properties: tuple.get("properties"),
            browser: tuple.get("browser"),
        }
    }

    fn get(&self, field: &str) -> Option<&dyn romm::pq::ToSql> {
        match field {
            "uuid" => match self.uuid {
                Some(ref uuid) => Some(uuid),
                None => None,
            },
            "name" => Some(&self.name),
            "visitor_id" => Some(&self.visitor_id),
            "properties" => Some(&self.properties),
            "browser" => Some(&self.browser),
            _ => None,
        }
    }
}

struct EventModel<'a> {
    connection: &'a romm::Connection,
}

impl<'a> romm::Model<'a> for EventModel<'a>
{
    type Entity = Event;
    type RowStructure = EventStructure;

    fn new(connection: &'a romm::Connection) -> Self {
        Self {
            connection,
        }
    }
}

impl<'a> EventModel<'a> {
    fn count_uniq_visitor(&self) -> romm::Result<u32> {
        self.connection.execute("select count(distinct visitor_id) as count from event", &[])
            .map(|x| x.get(0).unwrap().get("count"))
    }
}

#[derive(Clone, Debug)]
struct EventExtra
{
    uuid: Option<uuid::Uuid>,
    name: String,
    visitor_id: i32,
    properties: serde_json::Value,
    browser: serde_json::Value,
    os: Option<String>,
}

impl romm::Entity for EventExtra
{
    fn from(tuple: &romm::pq::Tuple) -> Self
    {
        let event = <Event as romm::Entity>::from(tuple);

        Self {
            uuid: event.uuid,
            name: event.name,
            visitor_id: event.visitor_id,
            properties: event.properties,
            browser: event.browser,
            os: tuple.get("os"),
        }
    }

    fn get(&self, field: &str) -> Option<&dyn romm::pq::ToSql> {
        match field {
            "uuid" => match self.uuid {
                Some(ref uuid) => Some(uuid),
                None => None,
            },
            "name" => Some(&self.name),
            "visitor_id" => Some(&self.visitor_id),
            "properties" => Some(&self.properties),
            "browser" => Some(&self.browser),
            "os" => match self.os {
                Some(ref os) => Some(os),
                None => None,
            },
            _ => None,
        }
    }
}

struct EventExtraModel;

impl<'a> romm::Model<'a> for EventExtraModel
{
    type Entity = EventExtra;
    type RowStructure = EventStructure;

    fn new(_: &'a romm::Connection) -> Self {
        Self {
        }
    }

    fn create_projection() -> romm::Projection
    {
        Self::default_projection()
            .set_field("os", romm::Row {
                content: "%:browser:% ->> 'os'",
                ty: romm::pq::ty::VARCHAR,
            })
    }
}

struct EventStructure;

impl romm::row::Structure for EventStructure
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
                ty: romm::pq::ty::UUID,
            },
            "name" => romm::Row {
                content: "%:name:%",
                ty: romm::pq::ty::VARCHAR,
            },
            "visitor_id" => romm::Row {
                content: "%:visitor_id:%",
                ty: romm::pq::ty::INT4,
            },
            "properties" => romm::Row {
                content: "%:properties:%",
                ty: romm::pq::ty::JSON,
            },
            "browser" => romm::Row {
                content: "%:browser:%",
                ty: romm::pq::ty::JSON,
            },
        }
    }
}
