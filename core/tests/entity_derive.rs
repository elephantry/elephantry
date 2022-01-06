#[derive(Debug, elephantry::Entity)]
#[elephantry(structure = "EventStructure")]
struct Event<T: elephantry::FromSql + elephantry::ToSql> {
    #[cfg(feature = "uuid")]
    #[elephantry(pk)]
    uuid: Option<uuid::Uuid>,
    #[cfg(not(feature = "uuid"))]
    #[elephantry(pk)]
    uuid: Option<String>,
    name: String,
    #[elephantry(default)]
    visitor_id: i32,
    #[cfg(feature = "json")]
    properties: serde_json::Value,
    #[cfg(not(feature = "json"))]
    properties: String,
    #[cfg(feature = "json")]
    browser: serde_json::Value,
    #[cfg(not(feature = "json"))]
    browser: String,
    #[elephantry(default)]
    generic: Option<T>,
}

struct EventModel {
    #[allow(dead_code)]
    connection: elephantry::Connection,
}

impl elephantry::Model for EventModel {
    type Entity = Event<String>;
    type Structure = EventStructure;

    fn new(connection: &elephantry::Connection) -> Self {
        Self { connection: connection.clone() }
    }
}

impl EventModel {
    #[allow(dead_code)]
    fn count_uniq_visitor(&self) -> elephantry::Result<u32> {
        self.connection
            .execute("select count(distinct visitor_id) as count from event")
            .map(|x| x.get(0).get("count"))
    }
}

#[derive(Debug, elephantry::Entity)]
struct EventExtra {
    #[cfg(feature = "uuid")]
    uuid: Option<uuid::Uuid>,
    #[cfg(not(feature = "uuid"))]
    uuid: Option<String>,
    name: String,
    #[elephantry(default)]
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

impl elephantry::Model for EventExtraModel {
    type Entity = EventExtra;
    type Structure = EventStructure;

    fn new(_: &elephantry::Connection) -> Self {
        Self
    }

    fn create_projection() -> elephantry::Projection {
        Self::default_projection().add_field("os", "%:browser:% ->> 'os'")
    }
}
