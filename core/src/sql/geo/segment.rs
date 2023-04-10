/**
 * Rust type for
 * [segment](https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-LSEG).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
#[derive(Clone, Debug, PartialEq)]
pub struct Segment(geo_types::Line<f64>);

impl Segment {
    #[must_use]
    pub fn new(start: &crate::Coordinate, end: &crate::Coordinate) -> Self {
        use std::ops::Deref;

        Self(geo_types::Line::new(*start.deref(), *end.deref()))
    }
}

impl std::ops::Deref for Segment {
    type Target = geo_types::Line<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(({}, {}), ({}, {}))",
            self.0.start.x, self.0.start.y, self.0.end.x, self.0.end.y
        )
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Segment {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let segment = Self::new(
            &crate::Coordinate::arbitrary(u)?,
            &crate::Coordinate::arbitrary(u)?,
        );

        Ok(segment)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::ToSql for Segment {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::LSEG
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L2034
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L2064
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        crate::to_sql::write_f64(&mut buf, self.0.start.x)?;
        crate::to_sql::write_f64(&mut buf, self.0.start.y)?;
        crate::to_sql::write_f64(&mut buf, self.0.end.x)?;
        crate::to_sql::write_f64(&mut buf, self.0.end.y)?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::FromSql for Segment {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L2022
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let coordinates = crate::not_null(raw)?
            .parse::<crate::Coordinates>()
            .map_err(|_| Self::error(ty, raw))?;

        if coordinates.len() != 2 {
            return Err(Self::error(ty, raw));
        }

        Ok(Self::new(&coordinates[0], &coordinates[1]))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L2045
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;

        let start = crate::Coordinate::new(
            crate::from_sql::read_f64(&mut buf)?,
            crate::from_sql::read_f64(&mut buf)?,
        );
        let end = crate::Coordinate::new(
            crate::from_sql::read_f64(&mut buf)?,
            crate::from_sql::read_f64(&mut buf)?,
        );

        Ok(Self::new(&start, &end))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::entity::Simple for Segment {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        lseg,
        crate::Segment,
        [
            (
                "'[(1, 2), (3, 4)]'",
                crate::Segment::new(
                    &crate::Coordinate::new(1., 2.),
                    &crate::Coordinate::new(3., 4.)
                )
            ),
            (
                "'((10.3, 20.0), (0.5, 0.003))'",
                crate::Segment::new(
                    &crate::Coordinate::new(10.3, 20.),
                    &crate::Coordinate::new(0.5, 0.003)
                )
            ),
        ]
    );
}
