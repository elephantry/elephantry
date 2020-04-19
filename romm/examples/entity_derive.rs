#[derive(Clone, Debug, romm::Entity)]
struct Event
{
    uuid: Option<uuid::Uuid>,
    name: String,
    visitor_id: i32,
    properties: serde_json::Value,
    browser: serde_json::Value,
}

struct EventModel;

impl romm::Model for EventModel
{
    type Entity = Event;
    type RowStructure = EventStructure;
}

#[derive(Clone, Debug, romm::Entity)]
struct EventExtra
{
    uuid: Option<uuid::Uuid>,
    name: String,
    visitor_id: i32,
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
