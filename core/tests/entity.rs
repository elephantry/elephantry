#[derive(Debug)]
struct Event<T: elephantry::FromSql + elephantry::ToSql> {
    #[cfg(feature = "uuid")]
    uuid: Option<uuid::Uuid>,
    #[cfg(not(feature = "uuid"))]
    uuid: Option<String>,
    name: String,
    visitor_id: Option<i32>,
    #[cfg(feature = "json")]
    properties: serde_json::Value,
    #[cfg(not(feature = "json"))]
    properties: String,
    #[cfg(feature = "json")]
    browser: serde_json::Value,
    #[cfg(not(feature = "json"))]
    browser: String,
    generic: Option<T>,
}

impl<T: elephantry::FromSql + elephantry::ToSql> elephantry::Entity for Event<T> {
    fn from(tuple: &elephantry::Tuple) -> Self {
        Self {
            uuid: tuple.get("uuid"),
            name: tuple.get("name"),
            visitor_id: tuple.get("visitor_id"),
            properties: tuple.get("properties"),
            browser: tuple.get("browser"),
            generic: tuple.get("generic"),
        }
    }

    fn get(&self, field: &str) -> Option<&dyn elephantry::ToSql> {
        match field {
            "uuid" => match self.uuid {
                Some(ref uuid) => Some(uuid),
                None => None,
            },
            "name" => Some(&self.name),
            "visitor_id" => match self.visitor_id {
                Some(ref visitor_id) => Some(visitor_id),
                None => Default::default(),
            },
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
    type Entity = Event<String>;
    type Structure = EventStructure;

    fn new(connection: &'a elephantry::Connection) -> Self {
        Self { connection }
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
struct EventExtra<T> {
    #[cfg(feature = "uuid")]
    uuid: Option<uuid::Uuid>,
    #[cfg(not(feature = "uuid"))]
    uuid: Option<String>,
    name: String,
    visitor_id: Option<i32>,
    #[cfg(feature = "json")]
    properties: serde_json::Value,
    #[cfg(not(feature = "json"))]
    properties: String,
    #[cfg(feature = "json")]
    browser: serde_json::Value,
    generic: Option<T>,
    #[cfg(not(feature = "json"))]
    browser: String,
    os: Option<String>,
}

impl<T: elephantry::Entity> elephantry::Entity for EventExtra<T> {
    fn from(tuple: &elephantry::Tuple) -> Self {
        let event = <Event<String> as elephantry::Entity>::from(tuple);

        Self {
            uuid: event.uuid,
            name: event.name,
            visitor_id: event.visitor_id,
            properties: event.properties,
            browser: event.browser,
            generic: None,
            os: tuple.get("os"),
        }
    }

    fn get(&self, field: &str) -> Option<&dyn elephantry::ToSql> {
        match field {
            "uuid" => match self.uuid {
                Some(ref uuid) => Some(uuid),
                None => None,
            },
            "name" => Some(&self.name),
            "visitor_id" => match self.visitor_id {
                Some(ref visitor_id) => Some(visitor_id),
                None => Default::default(),
            },
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

impl<'a> elephantry::Model<'a> for EventExtraModel {
    type Entity = EventExtra<String>;
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

    fn columns() -> &'static [&'static str] {
        &["uuid", "name", "visitor_id", "properties", "browser"]
    }
}
