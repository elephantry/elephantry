/**
 * Trait to convert rust struct to [composite
 * type](https://www.postgresql.org/docs/current/rowtypes.html).
 */
pub trait Composite {
    /**
     * Composite type name.
     */
    fn name() -> &'static str;

    /**
     * Convert struct to a vector of SQL value.
     */
    fn to_vec(&self) -> Vec<&dyn crate::ToSql>;

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        crate::sql::record::vec_to_text(&self.to_vec())
    }

    /**
     * Create a new struct from SQL result in text format.
     */
    fn from_text_values(ty: &crate::pq::Type, values: &[Option<&str>]) -> crate::Result<Box<Self>>;

    /**
     * Create a new struct from SQL result in binary format.
     */
    fn from_binary_values(
        ty: &crate::pq::Type,
        values: &[Option<&[u8]>],
    ) -> crate::Result<Box<Self>>;

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Box<Self>> {
        let values = crate::sql::record::binary_to_vec(raw)?;

        Self::from_binary_values(ty, &values)
    }

    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Box<Self>> {
        let values = crate::sql::record::text_to_vec(raw)?;

        Self::from_text_values(ty, &values)
    }
}

impl<C: Composite> crate::ToSql for C {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::Type {
            oid: 0,
            descr: Self::name(),
            name: Self::name(),
            kind: libpq::types::Kind::Composite,
        }
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_text()
    }
}

impl<C: Composite> crate::FromSql for C {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        Self::from_text(ty, raw).map(|x| *x)
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        Self::from_binary(ty, raw).map(|x| *x)
    }
}

#[cfg(test)]
mod test {
    #[derive(elephantry_derive::Composite, Debug, PartialEq)]
    #[elephantry(internal)]
    struct CompFoo {
        f1: i32,
        f2: String,
    }

    crate::sql_test!(
        compfoo,
        super::CompFoo,
        [(
            "'(1,foo)'",
            super::CompFoo {
                f1: 1,
                f2: "foo".to_string()
            }
        )]
    );
}
