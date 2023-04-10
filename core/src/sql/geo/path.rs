/**
 * Rust type for
 * [path](https://www.postgresql.org/docs/current/datatype-geometric.html#id-1.5.7.16.9).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
#[derive(Clone, Debug, PartialEq)]
pub struct Path(geo_types::LineString<f64>);

impl Path {
    #[must_use]
    pub fn new(coordinates: &crate::Coordinates) -> Self {
        use std::ops::Deref;

        Self(geo_types::LineString(
            coordinates.iter().map(|x| *x.deref()).collect(),
        ))
    }
}

impl std::ops::Deref for Path {
    type Target = geo_types::LineString<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write as _;

        let mut s = String::new();

        for coordinate in self.0.points() {
            write!(s, "({}, {}),", coordinate.x(), coordinate.y())?;
        }

        if !self.is_closed() {
            write!(f, "[")?;
        }

        write!(f, "{}", s.trim_end_matches(','))?;

        if !self.is_closed() {
            write!(f, "]")?;
        }

        Ok(())
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Path {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let path = Self::new(&crate::Coordinates::arbitrary(u)?);

        Ok(path)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::ToSql for Path {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::PATH
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1433
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1485
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = vec![self.0.is_closed() as u8];

        let points = self.0.clone().into_points();
        crate::to_sql::write_i32(&mut buf, points.len() as i32)?;

        for point in points {
            crate::to_sql::write_f64(&mut buf, point.x())?;
            crate::to_sql::write_f64(&mut buf, point.y())?;
        }

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::FromSql for Path {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1364
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let raw = crate::not_null(raw)?;

        let coordinates = raw.parse().map_err(|_| Self::error(ty, raw))?;

        let mut path = Self::new(&coordinates);

        if raw.starts_with('(') {
            path.0.close();
        }

        Ok(path)
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1447
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::not_null(raw)?;
        let closed = crate::from_sql::read_u8(&mut buf)? == 1;
        let npts = crate::from_sql::read_i32(&mut buf)?;
        let mut coordinates = Vec::new();

        for _ in 0..npts {
            let x = crate::from_sql::read_f64(&mut buf)?;
            let y = crate::from_sql::read_f64(&mut buf)?;

            let coordinate = crate::Coordinate::new(x, y);
            coordinates.push(coordinate);
        }

        let mut path = Self::new(&coordinates.into());

        if closed {
            path.0.close();
        }

        Ok(path)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::entity::Simple for Path {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        path,
        crate::Path,
        [
            (
                "'[(0, 0), (10, 10), (10, 0), (0, 0)]'",
                crate::Path::new(
                    &vec![
                        crate::Coordinate::new(0., 0.),
                        crate::Coordinate::new(10., 10.),
                        crate::Coordinate::new(10., 0.),
                        crate::Coordinate::new(0., 0.),
                    ]
                    .into()
                )
            ),
            (
                "'[(0, 0), (10, 10), (10, 0)]'",
                crate::Path::new(
                    &vec![
                        crate::Coordinate::new(0., 0.),
                        crate::Coordinate::new(10., 10.),
                        crate::Coordinate::new(10., 0.),
                    ]
                    .into()
                )
            ),
            (
                "'((0, 0), (10, 10), (10, 0))'",
                crate::Path::new(
                    &vec![
                        crate::Coordinate::new(0., 0.),
                        crate::Coordinate::new(10., 10.),
                        crate::Coordinate::new(10., 0.),
                        crate::Coordinate::new(0., 0.),
                    ]
                    .into()
                )
            ),
        ]
    );
}
