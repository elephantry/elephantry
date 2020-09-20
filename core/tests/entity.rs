#[derive(Debug)]
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

impl elephantry::Entity for Event {
    fn from(tuple: &elephantry::Tuple) -> Self {
        Self {
            uuid: tuple.get("uuid"),
            name: tuple.get("name"),
            visitor_id: tuple.get("visitor_id"),
            properties: tuple.get("properties"),
            browser: tuple.get("browser"),
        }
    }

    fn get(&self, field: &str) -> Option<&dyn elephantry::ToSql> {
        match field {
            "uuid" => {
                match self.uuid {
                    Some(ref uuid) => Some(uuid),
                    None => None,
                }
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
    #[allow(dead_code)]
    connection: &'a elephantry::Connection,
}

impl<'a> elephantry::Model<'a> for EventModel<'a> {
    type Entity = Event;
    type Structure = EventStructure;

    fn new(connection: &'a elephantry::Connection) -> Self {
        Self {
            connection,
        }
    }
}

impl<'a> EventModel<'a> {
    #[allow(dead_code)]
    fn count_uniq_visitor(&self) -> elephantry::Result<u32> {
        self.connection
            .execute("select count(distinct visitor_id) as count from event")
            .map(|x| x.get(0).get("count"))
    }
}

#[derive(Debug)]
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

impl elephantry::Entity for EventExtra {
    fn from(tuple: &elephantry::Tuple) -> Self {
        let event = <Event as elephantry::Entity>::from(tuple);

        Self {
            uuid: event.uuid,
            name: event.name,
            visitor_id: event.visitor_id,
            properties: event.properties,
            browser: event.browser,
            os: tuple.get("os"),
        }
    }

    fn get(&self, field: &str) -> Option<&dyn elephantry::ToSql> {
        match field {
            "uuid" => {
                match self.uuid {
                    Some(ref uuid) => Some(uuid),
                    None => None,
                }
            },
            "name" => Some(&self.name),
            "visitor_id" => Some(&self.visitor_id),
            "properties" => Some(&self.properties),
            "browser" => Some(&self.browser),
            "os" => {
                match self.os {
                    Some(ref os) => Some(os),
                    None => None,
                }
            },
            _ => None,
        }
    }
}

struct EventExtraModel;

impl<'a> elephantry::Model<'a> for EventExtraModel {
    type Entity = EventExtra;
    type Structure = EventStructure;

    fn new(_: &'a elephantry::Connection) -> Self {
        Self {}
    }

    fn create_projection() -> elephantry::Projection {
        Self::default_projection().add_field("os", "%:browser:% ->> 'os'")
    }
}

struct EventStructure;

impl elephantry::Structure for EventStructure {
    fn relation() -> &'static str {
        "public.event"
    }

    fn primary_key() -> &'static [&'static str] {
        &["uuid"]
    }

    fn definition() -> &'static [&'static str] {
        &["uuid", "name", "visitor_id", "properties", "browser"]
    }
}
