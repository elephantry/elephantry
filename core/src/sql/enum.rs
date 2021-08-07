/**
 * Trait to convert rust enum to [postgresql
 * enum](https://www.postgresql.org/docs/current/datatype-enum.html).
 */
pub trait Enum: std::fmt::Debug {
    /** Enum name */
    fn name() -> &'static str;
    /** Convert str to enum value */
    fn from_text(value: &str) -> crate::Result<Box<Self>>;
}

/*
 * @FIXME impl FromSql/ToSql instead
 * https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md
 */
impl<E: Enum> crate::Composite for E {
    fn name() -> &'static str {
        E::name()
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        use crate::ToSql;

        format!("{:?}", self).to_text()
    }

    fn from_text(_: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Box<Self>> {
        Self::from_text(crate::not_null(raw)?)
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Box<Self>> {
        use crate::FromSql;

        Self::from_text(&String::from_binary(ty, raw)?)
    }

    fn to_vec(&self) -> Vec<&dyn crate::ToSql> {
        unreachable!()
    }

    fn from_text_values(_: &crate::pq::Type, _: &[Option<&str>]) -> crate::Result<Box<Self>> {
        unreachable!();
    }

    fn from_binary_values(
        _ty: &crate::pq::Type,
        _values: &[Option<&[u8]>],
    ) -> crate::Result<Box<Self>> {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    #[derive(elephantry_derive::Enum, Debug, PartialEq)]
    #[elephantry(internal)]
    enum Mood {
        Sad,
        Ok,
        Happy,
    }

    crate::sql_test!(
        mood,
        super::Mood,
        [
            ("'Sad'", super::Mood::Sad),
            ("'Ok'", super::Mood::Ok),
            ("'Happy'", super::Mood::Happy),
        ]
    );
}
