/**
 * Rust type for
 * [box](https://www.postgresql.org/docs/current/datatype-geometric.html#id-1.5.7.16.8).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
#[derive(Clone, Debug, PartialEq)]
pub struct Box(geo_types::Rect<f64>);

impl Box {
    pub fn new(start: crate::Point, end: crate::Point) -> Self {
        use std::ops::Deref;

        Self(geo_types::Rect::new(*start.deref(), *end.deref()))
    }
}

impl std::ops::Deref for Box {
    type Target = geo_types::Rect<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(({}, {}), ({}, {}))",
            self.0.min().x,
            self.0.min().y,
            self.0.max().x,
            self.0.max().y
        )
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Box {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let b = Self::new(crate::Point::arbitrary(u)?, crate::Point::arbitrary(u)?);

        Ok(b)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::ToSql for Box {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::BOX
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L443
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L489
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        crate::to_sql::write_f64(&mut buf, self.0.max().x)?;
        crate::to_sql::write_f64(&mut buf, self.0.max().y)?;
        crate::to_sql::write_f64(&mut buf, self.0.min().x)?;
        crate::to_sql::write_f64(&mut buf, self.0.min().y)?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::FromSql for Box {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L413
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let segment = crate::Segment::from_text(ty, raw)?;

        Ok(Self::new(segment.end.into(), segment.start.into()))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L454
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let segment = crate::Segment::from_binary(ty, raw)?;

        Ok(Self::new(segment.end.into(), segment.start.into()))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::entity::Simple for Box {}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]
    crate::sql_test!(
        Box,
        crate::Box,
        [
            (
                "'((1, 2), (3, 4))'",
                crate::Box::new(crate::Point::new(1., 2.), crate::Point::new(3., 4.))
            ),
            (
                "'((0.5, 0.003), (10.3, 20.0))'",
                crate::Box::new(crate::Point::new(0.5, 0.003), crate::Point::new(10.3, 20.))
            ),
        ]
    );
}
