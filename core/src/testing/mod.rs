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
        let result = connection.execute(&format!("select {value}::{sql_type} as actual"))?;
        assert_eq!(result.get(0).get::<T>("actual"), *expected, "from_text");
    }

    Ok(())
}

#[doc(hidden)]
pub fn from_binary<T>(
    connection: &crate::Connection,
    sql_type: &str,
    tests: &[(&str, T)],
) -> crate::Result
where
    T: crate::FromSql + crate::ToSql + PartialEq + std::fmt::Debug,
{
    use std::collections::HashMap;

    for (value, expected) in tests {
        let result = connection
            .query::<HashMap<String, T>>(&format!("select {value}::{sql_type} as actual"), &[])?;
        assert_eq!(
            result.get(0).get("actual").unwrap(),
            expected,
            "from_binary"
        );
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
    for (_, value) in tests {
        let result = connection.query::<T>(&format!("select $1::{sql_type}"), &[value]);
        assert!(dbg!(&result).is_ok());
        assert_eq!(&result.unwrap().get(0), value, "to_text");
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
    for (_, value) in tests {
        let result: crate::pq::Result = connection
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
