mod r#fn;
mod macros;

pub use r#fn::Fn;
pub use macros::*;

#[doc(hidden)]
pub fn from_text<T>(
    connection: &crate::Connection,
    sql_type: &str,
    tests: &[(&str, T)],
) -> crate::Result
where
    T: crate::FromSql + crate::ToSql + PartialEq + std::fmt::Debug,
{
    for (value, expected) in tests {
        let result = connection.execute(&format!("select {value}::{sql_type}"));

        match result {
            Ok(actual) => assert_eq!(actual.get(0).nth::<T>(0), *expected, "from_text"),
            Err(err) => {
                panic!("{err}");
            }
        }
    }

    Ok(())
}

macro_rules! assert_result {
    ($result:ident, $expected:ident, $test:literal) => {
        match $result {
            Ok(actual) => assert_eq!(actual.get(0), *$expected, $test),
            Err(err) => {
                panic!("{err}");
            }
        }
    };
}

#[doc(hidden)]
pub fn from_binary<T>(
    connection: &crate::Connection,
    sql_type: &str,
    tests: &[(&str, T)],
) -> crate::Result
where
    T: crate::Entity + crate::ToSql + PartialEq + std::fmt::Debug,
{
    for (value, expected) in tests {
        let result = connection.query::<T>(&format!("select {value}::{sql_type}"), &[]);
        assert_result!(result, expected, "from_binary");
    }

    Ok(())
}

#[doc(hidden)]
pub fn to_text<T>(
    connection: &crate::Connection,
    sql_type: &str,
    tests: &[(&str, T)],
) -> crate::Result
where
    T: crate::Entity + crate::ToSql + PartialEq + std::fmt::Debug,
{
    let mut connection = connection.clone();
    connection.mode = crate::pq::Format::Text;

    for (_, value) in tests {
        let result = connection.query::<T>(&format!("select $1::{sql_type}"), &[value]);
        assert_result!(result, value, "to_text");
    }

    Ok(())
}

#[doc(hidden)]
pub fn to_binary<T>(
    connection: &crate::Connection,
    sql_type: &str,
    tests: &[(&str, T)],
) -> crate::Result
where
    T: crate::Entity + crate::ToSql + PartialEq + std::fmt::Debug,
{
    let mut connection = connection.clone();
    connection.mode = crate::pq::Format::Binary;

    for (_, value) in tests {
        let result = connection.query::<T>(&format!("select $1::{sql_type}"), &[value]);
        assert_result!(result, value, "to_binary");
    }

    Ok(())
}
