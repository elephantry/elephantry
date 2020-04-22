#[derive(Clone, Debug, romm::Entity)]
struct Event
{
    uuid: Option<uuid::Uuid>,
    name: String,
    visitor_id: i32,
    properties: serde_json::Value,
    browser: serde_json::Value,
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
