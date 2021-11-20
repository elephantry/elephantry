#![warn(warnings)]
#![doc(html_logo_url = "https://elephantry.github.io/logo.png")]
#![cfg_attr(docsrs, feature(doc_cfg))]

/*!
 *
 * | SQL type                    | Rust type                | Feature    |
 * |-----------------------------|--------------------------|------------|
 * | `bigint`/`int8`             | `i64`                    |            |
 * | `bit`                       | `u8`                     | bit        |
 * | `bit varying`               | `bit_vec::BitVec`        | bit        |
 * | `boolean`                   | `bool`                   |            |
 * | `box`                       | `elephantry::Box`        | geo        |
 * | `bytea`                     | `elephantry::Bytea`      |            |
 * | `char`                      | `char`                   |            |
 * | `varchar`                   | `String`                 |            |
 * | `cidr`                      | `ipnetwork::IpNetwork`   | net        |
 * | `circle`                    | `elephantry::Circle`     | geo        |
 * | `date`                      | `chrono::NaiveDate`      | date       |
 * | `double precision`/`float8` | `f64`                    |            |
 * | `hstore`                    | `elephantry::Hstore`     |            |
 * | `inet`                      | `std::net::IpAddr`       | net        |
 * | `integer`/`int4`            | `i32`                    |            |
 * | `interval`                  | `elephantry::Interval`   | date       |
 * | `json`                      | `serde_json::Value`      | json       |
 * | `jsonb`                     | `elephantry::Jsonb`      | json       |
 * | `line`                      | `elephantry::Line`       | geo        |
 * | `lseg`                      | `elephantry::Segment`    | geo        |
 * | `null`                      | `()`                     |            |
 * | `macaddr`                   | `macaddr::MacAddr6`      | net        |
 * | `macaddr8`                  | `macaddr::MacAddr8`      | net        |
 * | `money`                     | `f32`                    |            |
 * | `multirange`                | `elephantry::Multirange` | multirange |
 * | `numeric`                   | `bigdecimal::BigDecimal` | numeric    |
 * | `path`                      | `elephantry::Path`       | geo        |
 * | `point`                     | `elephantry::Point`      | geo        |
 * | `polygon`                   | `elephantry::Polygon`    | geo        |
 * | `real`/`float4`             | `f32`                    |            |
 * | `record`                    | `tuple`                  |            |
 * | `smallint`/`int2`           | `i16`                    |            |
 * | `text`                      | `String`                 |            |
 * | `time`                      | `elephantry::Time`       | time       |
 * | `timetz`                    | `elephantry::TimeTz`     | time       |
 * | `timestamp`                 | `chrono::NaiveDateTime`  | date       |
 * | `timestamptz`               | `chrono::DateTime`       | date       |
 * | `uuid`                      | `uuid::Uuid`             | uuid       |
 * | `xml`                       | `xmltree::Element`       | xml        |
 * | `[x, y)`                    | `std::ops::Range`        |            |
 * | `[x,)`                      | `std::ops::RangeFrom`    |            |
 * | `[,y)`                      | `std::ops::RangeTo`      |            |
 * | `(,)`                       | `std::ops::RangeFull`    |            |
 */

pub mod config;
pub mod connection;
pub mod entity;
/** database inspection module. */
pub mod inspect;
/** libpq abstraction layer. */
pub mod pq;
#[cfg(feature = "r2d2")]
#[cfg_attr(docsrs, doc(cfg(feature = "r2d2")))]
pub mod r2d2;
#[cfg(feature = "rocket")]
#[doc(hidden)]
pub mod rocket;
pub mod transaction;

mod r#async;
mod errors;
mod from_sql;
mod model;
mod pager;
mod pool;
mod projection;
mod rows;
mod sql;
mod structure;
mod to_sql;
mod tuple;
mod r#where;

pub use crate::config::Config;
pub use connection::Connection;
pub use elephantry_derive::*;
pub use entity::Entity;
pub use errors::*;
pub use from_sql::*;
pub use model::*;
pub use pager::*;
pub use pool::*;
pub use projection::*;
pub use r#async::*;
pub use r#where::Where;
pub use rows::*;
pub use sql::*;
pub use structure::*;
pub use to_sql::*;
pub use transaction::Transaction;
pub use tuple::*;

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
    static INIT: std::sync::Once = std::sync::Once::new();

    #[macro_export]
    macro_rules! sql_test_from {
        ($sql_type:ident, $rust_type:ty, $tests:expr) => {
            use std::collections::HashMap;

            #[test]
            fn from_text() -> crate::Result {
                let conn = crate::test::new_conn()?;

                for (value, expected) in &$tests {
                    let result = conn.execute(&format!(
                        "select {}::{} as actual",
                        value,
                        stringify!($sql_type)
                    ))?;
                    assert_eq!(result.get(0).get::<$rust_type>("actual"), *expected);
                }

                Ok(())
            }

            #[test]
            fn from_binary() -> crate::Result {
                let conn = crate::test::new_conn()?;

                for (value, expected) in &$tests {
                    let result = conn.query::<HashMap<String, $rust_type>>(
                        &format!("select {}::{} as actual", value, stringify!($sql_type)),
                        &[],
                    )?;
                    assert_eq!(result.get(0).get("actual").unwrap(), expected);
                }

                Ok(())
            }
        };
    }

    #[macro_export]
    macro_rules! sql_test_to {
        ($sql_type:ident, $rust_type:ty, $tests:expr) => {
            #[test]
            fn to_text() -> crate::Result {
                let conn = crate::test::new_conn()?;

                for (_, value) in &$tests {
                    let result = conn.query::<$rust_type>(
                        &format!("select $1::{}", stringify!($sql_type)),
                        &[value],
                    );
                    assert!(dbg!(&result).is_ok());
                    assert_eq!(&result.unwrap().get(0), value);
                }

                Ok(())
            }

            #[test]
            fn to_binary() -> crate::Result {
                let conn = crate::test::new_conn()?;

                for (_, value) in &$tests {
                    use crate::ToSql;

                    let result: crate::pq::Result = conn
                        .connection
                        .lock()
                        .map_err(|e| crate::Error::Mutex(e.to_string()))?
                        .exec_params(
                            &format!("select $1::{}", stringify!($sql_type)),
                            &[value.ty().oid],
                            &[value.to_binary()?],
                            &[crate::pq::Format::Binary],
                            crate::pq::Format::Binary,
                        )
                        .try_into()?;
                    let rows: crate::Rows<$rust_type> = result.into();

                    assert_eq!(&rows.get(0), value);
                }

                Ok(())
            }
        };
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

    pub fn new_conn() -> crate::Result<crate::Pool> {
        INIT.call_once(|| {
            env_logger::init();
        });

        let conn = crate::Pool::new(&dsn())?;
        conn.execute("create extension if not exists hstore")?;
        conn.execute("set lc_monetary to 'en_US.UTF-8';")?;
        conn.execute(
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

        Ok(conn)
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

    #[derive(elephantry_derive::Entity)]
    #[elephantry(
        internal,
        model = "Model",
        structure = "Structure",
        relation = "entity"
    )]
    pub struct Entity {
        #[elephantry(pk, column = "employee_id")]
        pub id: i32,
        pub first_name: String,
        #[elephantry(default)]
        pub last_name: String,
    }
}
