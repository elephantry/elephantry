#[derive(Clone, Debug, loxo::Entity)]
struct Event {
    #[cfg(feature = "uuid")]
    uuid: Option<uuid::Uuid>,
    #[cfg(not(feature = "uuid"))]
    uuid: Option<String>,
    name: String,
    visitor_id: i32,
    #[cfg(feature = "json")]
    properties: serde_json::Value,
    #[cfg(not(feature = "json"))]
    properties: String,
    #[cfg(feature = "json")]
    browser: serde_json::Value,
    #[cfg(not(feature = "json"))]
    browser: String,
}

struct EventModel<'a> {
    connection: &'a loxo::Connection,
}

impl<'a> loxo::Model<'a> for EventModel<'a> {
    type Entity = Event;
    type Structure = EventStructure;

    fn new(connection: &'a loxo::Connection) -> Self {
        Self { connection }
    }
}

impl<'a> EventModel<'a> {
    fn count_uniq_visitor(&self) -> loxo::Result<u32> {
        self.connection
            .execute("select count(distinct visitor_id) as count from event", &[])
            .map(|x| x.get(0).unwrap().get("count"))
    }
}

#[derive(Clone, Debug, loxo::Entity)]
struct EventExtra {
    #[cfg(feature = "uuid")]
    uuid: Option<uuid::Uuid>,
    #[cfg(not(feature = "uuid"))]
    uuid: Option<String>,
    name: String,
    visitor_id: i32,
    #[cfg(feature = "json")]
    properties: serde_json::Value,
    #[cfg(not(feature = "json"))]
    properties: String,
    #[cfg(feature = "json")]
    browser: serde_json::Value,
    #[cfg(not(feature = "json"))]
    browser: String,
    os: Option<String>,
}

struct EventExtraModel;

impl<'a> loxo::Model<'a> for EventExtraModel {
    type Entity = EventExtra;
    type Structure = EventStructure;

    fn new(_: &'a loxo::Connection) -> Self {
        Self {}
    }

    fn create_projection() -> loxo::Projection {
        Self::default_projection().set_field(
            "os",
            loxo::Row {
                content: "%:browser:% ->> 'os'",
                ty: loxo::pq::ty::VARCHAR,
            },
        )
    }
}

struct EventStructure;

impl loxo::row::Structure for EventStructure {
    fn relation() -> &'static str {
        "public.event"
    }

    fn primary_key() -> &'static [&'static str] {
        &["uuid"]
    }

    fn definition() -> std::collections::HashMap<&'static str, loxo::Row> {
        maplit::hashmap! {
            "uuid" => loxo::Row {
                content: "%:uuid:%",
                ty: loxo::pq::ty::UUID,
            },
            "name" => loxo::Row {
                content: "%:name:%",
                ty: loxo::pq::ty::VARCHAR,
            },
            "visitor_id" => loxo::Row {
                content: "%:visitor_id:%",
                ty: loxo::pq::ty::INT4,
            },
            "properties" => loxo::Row {
                content: "%:properties:%",
                ty: loxo::pq::ty::JSON,
            },
            "browser" => loxo::Row {
                content: "%:browser:%",
                ty: loxo::pq::ty::JSON,
            },
        }
    }
}
