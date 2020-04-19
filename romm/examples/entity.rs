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
    fn from(row: &romm::pq::Row) -> Self
    {
        Self {
            uuid: row.get("uuid").expect("Unable to find 'uuid' field"),
            name: row.get("name").expect("Unable to find 'name' field"),
            visitor_id: row.get("visitor_id").expect("Unable to find 'visitor_id' field"),
            properties: row.get("properties").expect("Unable to find 'properties' field"),
            browser: row.get("browser").expect("Unable to find 'browser' field"),
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

struct EventModel;

impl romm::Model for EventModel
{
    type Entity = Event;
    type RowStructure = EventStructure;
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
    fn from(row: &romm::pq::Row) -> Self
    {
        let event = <Event as romm::Entity>::from(row);

        Self {
            uuid: event.uuid,
            name: event.name,
            visitor_id: event.visitor_id,
            properties: event.properties,
            browser: event.browser,
            os: row.get("os").expect("Unable to find 'os' field"),
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

impl romm::Model for EventExtraModel
{
    type Entity = EventExtra;
    type RowStructure = EventStructure;

    fn create_projection() -> romm::Projection
    {
        Self::default_projection()
            .set_field("os", romm::Row {
                content: "%:browser:% ->> 'os'",
                ty: romm::pq::Type::VARCHAR,
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
                ty: romm::pq::Type::UUID,
            },
            "name" => romm::Row {
                content: "%:name:%",
                ty: romm::pq::Type::VARCHAR,
            },
            "visitor_id" => romm::Row {
                content: "%:visitor_id:%",
                ty: romm::pq::Type::INT4,
            },
            "properties" => romm::Row {
                content: "%:properties:%",
                ty: romm::pq::Type::JSON,
            },
            "browser" => romm::Row {
                content: "%:browser:%",
                ty: romm::pq::Type::JSON,
            },
        }
    }
}
