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
            .map(|x| x.get(0).get("count"))
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
        Self::default_projection().set_field("os", "%:browser:% ->> 'os'")
    }
}

struct EventStructure;

impl loxo::Structure for EventStructure {
    fn relation() -> &'static str {
        "public.event"
    }

    fn primary_key() -> &'static [&'static str] {
        &["uuid"]
    }

    fn definition() -> std::collections::HashMap<&'static str, &'static str> {
        maplit::hashmap! {
            "uuid" => "%:uuid:%",
            "name" => "%:name:%",
            "visitor_id" => "%:visitor_id:%",
            "properties" => "%:properties:%",
            "browser" => "%:browser:%",
        }
    }
}
