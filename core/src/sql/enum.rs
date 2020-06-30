pub trait Enum: std::fmt::Debug {
    fn name() -> &'static str;
    fn from_text(value: &str) -> crate::Result<Box<Self>>;
}

impl<E: Enum> crate::Composite for E {
    fn name() -> &'static str {
        E::name()
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        use crate::ToSql;

        format!("{:?}", self).to_sql()
    }

    fn to_vec(&self) -> Vec<&dyn crate::ToSql> {
        unreachable!()
    }

    fn from_text_values(
        _ty: &crate::pq::Type,
        values: &[Option<&str>],
    ) -> crate::Result<Box<Self>> {
        Self::from_text(crate::not_null(values[0])?)
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Box<Self>> {
        use crate::FromSql;

        Self::from_text(&String::from_binary(ty, raw)?)
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
    #[derive(crate::Enum, Debug, PartialEq)]
    #[r#enum(internal)]
    enum Mood {
        Sad,
        Ok,
        Happy,
    }

    crate::sql_test!(mood, super::Mood, [
        ("'Sad'", super::Mood::Sad),
        ("'Ok'", super::Mood::Ok),
        ("'Happy'", super::Mood::Happy),
    ]);
}
