#![warn(warnings)]
#![doc(html_logo_url = "https://elephantry.github.io/logo.png")]
#![cfg_attr(docsrs, feature(doc_cfg))]

/*!
 * Foreign types (ie defined in another crate), have [type alias](#types).
 *
 * | SQL type                    | Rust type                                                                                                       | Feature                              |
 * |-----------------------------|-----------------------------------------------------------------------------------------------------------------|--------------------------------------|
 * | `bigint`/`int8`             | `i64`                                                                                                           |                                      |
 * | `bit`                       | `u8`                                                                                                            | bit                                  |
 * | `bit(n)`                    | `[u8; n]`                                                                                                       | bit                                  |
 * | `bit varying`               | `bit_vec::BitVec`<br>`elephantry::Bits`                                                                         | bit                                  |
 * | `boolean`                   | `bool`                                                                                                          |                                      |
 * | `box`                       | `elephantry::Box`                                                                                               | geo                                  |
 * | `bytea`                     | `elephantry::Bytea`                                                                                             |                                      |
 * | `char`                      | `char`                                                                                                          |                                      |
 * | `varchar`                   | `String`                                                                                                        |                                      |
 * | `cidr`                      | `ipnetwork::IpNetwork`<br>`elephantry::Cidr`                                                                    | net                                  |
 * | `circle`                    | `elephantry::Circle`                                                                                            | geo                                  |
 * | `date`                      | `chrono::NaiveDate`<br>`elephantry::Date`<br>`jiff::civil::Date`                                                | date<br><br>jiff                     |
 * | `double precision`/`float8` | `f64`                                                                                                           |                                      |
 * | `hstore`                    | `elephantry::Hstore`                                                                                            |                                      |
 * | `inet`                      | `std::net::IpAddr`                                                                                              | net                                  |
 * | `integer`/`int4`            | `i32`                                                                                                           |                                      |
 * | `interval`                  | `elephantry::Interval`                                                                                          | date                                 |
 * | `json`                      | `serde_json::Value`<br>`elephantry::Json`                                                                       | json                                 |
 * | `jsonb`                     | `elephantry::Jsonb`                                                                                             | json                                 |
 * | `line`                      | `elephantry::Line`                                                                                              | geo                                  |
 * | `lquery`                    | `elephantry::Lquery`                                                                                            | ltree                                |
 * | `lseg`                      | `elephantry::Segment`                                                                                           | geo                                  |
 * | `ltree`                     | `elephantry::Ltree`                                                                                             | ltree                                |
 * | `ltxtquery`                 | `elephantry::Ltxtquery`                                                                                         | ltree                                |
 * | `null`                      | `()`                                                                                                            |                                      |
 * | `macaddr`                   | `macaddr::MacAddr6`<br>`elephantry::MacAddr`                                                                    | net                                  |
 * | `macaddr8`                  | `macaddr::MacAddr8`<br>`elephantry::MacAddr8`                                                                   | net                                  |
 * | `money`                     | `postgres_money::Money`<br>`elephantry::Money`                                                                  | money                                |
 * | `multirange`                | `elephantry::Multirange`                                                                                        | multirange                           |
 * | `numeric`                   | `bigdecimal::BigDecimal`<br>`elephantry::Numeric`                                                               | numeric                              |
 * | `path`                      | `elephantry::Path`                                                                                              | geo                                  |
 * | `point`                     | `elephantry::Point`                                                                                             | geo                                  |
 * | `polygon`                   | `elephantry::Polygon`                                                                                           | geo                                  |
 * | `real`/`float4`             | `f32`                                                                                                           |                                      |
 * | `record`                    | `tuple`                                                                                                         |                                      |
 * | `smallint`/`int2`           | `i16`                                                                                                           |                                      |
 * | `text`                      | `String`                                                                                                        |                                      |
 * | `time`                      | `elephantry::Time`<br>`time::Time`<br>`chrono::NaiveTime`<br>`jiff::civil::Time`                                | time<br><br>chrono<br>jiff           |
 * | `timetz`                    | `elephantry::TimeTz`<br>`(chrono::NaiveTime, chrono::FixedOffset)`<br>`(jiff::civil::Time, jiff::tz::TimeZone)` | time<br>chrono<br>jiff               |
 * | `timestamp`                 | `chrono::NaiveDateTime`<br>`elephantry::Timestamp`<br>`jiff::civil::DateTime`                                   | date<br><br>jiff                     |
 * | `timestamptz`               | `chrono::DateTime`<br>`elephantry::Timestamp`<br>`jiff::Zoned`                                                  | date<br><br>jiff                     |
 * | `uuid`                      | `uuid::Uuid`<br>`elephantry::Uid`                                                                               | uuid                                 |
 * | `xml`                       | `xmltree::Element`<br>`elephantry::Xml`                                                                         | xml                                  |
 * | `[x, y)`                    | `std::ops::Range`                                                                                               |                                      |
 * | `[x,)`                      | `std::ops::RangeFrom`                                                                                           |                                      |
 * | `[,y)`                      | `std::ops::RangeTo`                                                                                             |                                      |
 * | `(,)`                       | `std::ops::RangeFull`                                                                                           |                                      |
 */

pub mod config;
pub mod connection;
pub mod entity;
pub mod from_sql;
#[cfg(feature = "inspect")]
/** database inspection module. */
pub mod inspect;
/** libpq abstraction layer. */
pub mod pq;
#[cfg(feature = "r2d2")]
pub mod r2d2;
#[cfg(feature = "rocket")]
#[doc(hidden)]
pub mod rocket;
#[cfg(any(test, feature = "testing"))]
pub mod testing;
pub mod to_sql;
pub mod transaction;
pub mod r#where;

mod r#async;
mod errors;
mod from_text;
mod model;
mod notify;
mod pager;
mod pool;
mod projectable;
mod projection;
mod rows;
mod sql;
mod structure;
mod to_text;
mod tuple;

pub use crate::config::Config;
pub use r#async::*;
pub use connection::Connection;
#[cfg(any(test, feature = "testing"))]
pub use elephantry_derive::test;
pub use elephantry_derive::{Composite, Entity, Enum};
pub use entity::Entity;
pub use errors::*;
pub use from_sql::FromSql;
pub use from_text::*;
pub use model::*;
pub use notify::Notify;
pub use pager::*;
pub use pool::*;
pub use projectable::*;
pub use projection::*;
pub use rows::*;
pub use sql::*;
pub use structure::*;
pub use to_sql::ToSql;
pub use to_text::*;
pub use transaction::Transaction;
pub use tuple::*;
pub use r#where::Where;

macro_rules! regex {
    ($regex:literal) => {{
        static REGEX: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new($regex).unwrap());
        &REGEX
    }};
}

pub(crate) use regex;

/**
 * Easily create pk argument for where clause, including [`find_by_pk`]
 * function.
 *
 * ```
 * # #[macro_use] extern crate elephantry;
 * # fn main() {
 * # let uuid = "";
 * # let name = "";
 * pk!(uuid);
 * pk![uuid, name];
 * pk!{uuid => "uuid", name => "name"};
 * # }
 * ```
 *
 * [`find_by_pk`]: crate::Connection::find_by_pk
 */
#[macro_export]
macro_rules! pk {
    ($($pk:ident),+ $(,)?) => {
        $crate::pk!($(
            $pk => $pk,
        )*)
    };

    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut hash = std::collections::HashMap::new();

        $(
            hash.insert(stringify!($key), &$value as &dyn $crate::ToSql);
        )*

        hash
    }}
}

/**
 * Likes [`pk`] macro but for value argument, including [`update_by_pk`]
 * function.
 *
 * [`pk`]: crate::pk
 * [`update_by_pk`]: crate::Connection::update_by_pk
 */
#[macro_export]
macro_rules! values {
    ($($pk:ident),+ $(,)?) => {
        $crate::values!($(
            $pk => $pk,
        )*)
    };

    ($($key:expr => $value:expr),+ $(,)?) => {{
        let mut hash = std::collections::HashMap::new();

        $(
            hash.insert(stringify!($key).to_string(), &$value as &dyn $crate::ToSql);
        )*

        hash
    }}
}

#[cfg(test)]
mod test {
    #[test]
    fn test_pk_one() {
        let uuid = "1234";
        let pk = crate::pk!(uuid);

        assert_eq!(pk.len(), 1);
        assert!(pk.contains_key("uuid"));
    }

    #[test]
    fn test_pk_multi() {
        let uuid = "1234";
        let name = "name";
        let pk = crate::pk![uuid, name,];

        assert_eq!(pk.len(), 2);
        assert!(pk.contains_key("uuid"));
        assert!(pk.contains_key("name"));
    }

    #[test]
    fn test_pk_hash() {
        let pk = crate::pk! {
            uuid => "1234",
            name => "name",
        };

        assert_eq!(pk.len(), 2);
        assert!(pk.contains_key("uuid"));
        assert!(pk.contains_key("name"));
    }

    #[allow(dead_code)]
    #[derive(elephantry_derive::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "entity")]
    pub struct Entity {
        #[elephantry(pk, column = "employee_id")]
        pub id: i32,
        pub first_name: String,
        #[elephantry(default)]
        pub last_name: String,
    }
}
