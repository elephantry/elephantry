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
pub mod to_sql;
pub mod transaction;

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
mod r#where;

pub use crate::config::Config;
pub use r#async::*;
pub use connection::Connection;
pub use elephantry_derive::*;
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
    use std::collections::HashMap;

    #[macro_export]
    macro_rules! sql_test_from {
        ($sql_type:ident, $rust_type:ty, $tests:expr) => {
            #[test]
            fn from_text() -> $crate::Result {
                $crate::test::from_text::<$rust_type>(stringify!($sql_type), &$tests)
            }

            #[test]
            fn from_binary() -> $crate::Result {
                $crate::test::from_binary::<$rust_type>(stringify!($sql_type), &$tests)
            }
        };
    }

    pub(crate) fn from_text<T>(sql_type: &str, tests: &[(&str, T)]) -> crate::Result
    where
        T: crate::FromSql + crate::ToSql + PartialEq + std::fmt::Debug,
    {
        let conn = crate::test::new_conn()?;

        for (value, expected) in tests {
            let result = conn.execute(&format!("select {value}::{sql_type} as actual"))?;
            assert_eq!(result.get(0).get::<T>("actual"), *expected, "from_text");
        }

        Ok(())
    }

    pub(crate) fn from_binary<T>(sql_type: &str, tests: &[(&str, T)]) -> crate::Result
    where
        T: crate::FromSql + crate::ToSql + PartialEq + std::fmt::Debug,
    {
        let conn = crate::test::new_conn()?;

        for (value, expected) in tests {
            let result = conn.query::<HashMap<String, T>>(
                &format!("select {value}::{sql_type} as actual"),
                &[],
            )?;
            assert_eq!(
                result.get(0).get("actual").unwrap(),
                expected,
                "from_binary"
            );
        }

        Ok(())
    }

    #[macro_export]
    macro_rules! sql_test_to {
        ($sql_type:ident, $rust_type:ty, $tests:expr) => {
            #[test]
            fn to_text() -> $crate::Result {
                $crate::test::to_text::<$rust_type>(stringify!($sql_type), &$tests)
            }

            #[test]
            fn to_binary() -> $crate::Result {
                $crate::test::to_binary::<$rust_type>(stringify!($sql_type), &$tests)
            }
        };
    }

    pub(crate) fn to_text<T>(sql_type: &str, tests: &[(&str, T)]) -> crate::Result
    where
        T: crate::Entity + crate::ToSql + PartialEq + std::fmt::Debug,
    {
        let conn = crate::test::new_conn()?;

        for (_, value) in tests {
            let result = conn.query::<T>(&format!("select $1::{sql_type}"), &[value]);
            assert!(dbg!(&result).is_ok());
            assert_eq!(&result.unwrap().get(0), value, "to_text");
        }

        Ok(())
    }

    pub(crate) fn to_binary<T>(sql_type: &str, tests: &[(&str, T)]) -> crate::Result
    where
        T: crate::Entity + crate::ToSql + PartialEq + std::fmt::Debug,
    {
        let conn = crate::test::new_conn()?;

        for (_, value) in tests {
            let result: crate::pq::Result = conn
                .connection
                .lock()
                .map_err(|e| crate::Error::Mutex(e.to_string()))?
                .exec_params(
                    &format!("select $1::{sql_type}"),
                    &[value.ty().oid],
                    &[value.to_binary()?.as_deref()],
                    &[crate::pq::Format::Binary],
                    crate::pq::Format::Binary,
                )
                .try_into()?;
            let rows: crate::Rows<T> = result.into();

            assert_eq!(&rows.get(0), value, "to_binary");
        }

        Ok(())
    }

    #[macro_export]
    macro_rules! sql_test {
        ($sql_type:ident, $rust_type:ty, $tests:expr) => {
            mod $sql_type {
                $crate::sql_test_from!($sql_type, $rust_type, $tests);
                $crate::sql_test_to!($sql_type, $rust_type, $tests);
            }
        };
    }

    pub fn dsn() -> String {
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "host=localhost".to_string())
    }

    pub fn new_conn() -> crate::Result<&'static crate::Connection> {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            env_logger::init();
        });

        // @TODO #[feature(once_cell_try)]
        static POOL: std::sync::LazyLock<crate::Result<crate::Pool>> =
            std::sync::LazyLock::new(|| {
                let pool = crate::Pool::new(&dsn())?;
                pool.execute("create extension if not exists hstore")?;
                pool.execute("create extension if not exists ltree")?;
                pool.execute("set lc_monetary to 'en_US.UTF-8';")?;
                pool.execute(
                    "
do $$
begin
    if not exists (select 1 from pg_type where typname = 'compfoo')
    then
        create type compfoo as (f1 int, f2 text);
    end if;

    if not exists (select 1 from pg_type where typname = 'mood')
    then
        create type mood as enum ('Sad', 'Ok', 'Happy');
    end if;

    if not exists (select 1 from pg_type where typname = 'us_postal_code')
    then
        create domain us_postal_code as text
        check(
            value ~ '^\\d{5}$'
            or value ~ '^\\d{5}-\\d{4}$'
        );
    end if;
end$$;
        ",
                )?;

                Ok(pool)
            });

        Ok(POOL.as_ref().unwrap())
    }

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
