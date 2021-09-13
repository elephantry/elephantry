#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
#[derive(Clone, Debug, PartialEq)]
pub struct Path(geo_types::LineString<f64>);

impl Path {
    pub fn new(coordinates: &crate::Coordinates) -> Self {
        Self(geo_types::LineString(
            coordinates.iter().map(|x| *x.clone()).collect(),
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
        let mut s = String::new();

        for coordinate in self.0.points_iter() {
            s.push_str(&format!("({}, {}),", coordinate.x(), coordinate.y()));
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

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::FromSql for Path {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let raw = crate::not_null(raw)?;

        let coordinates = raw
            .parse()
            .map_err(|_| Self::error(ty, "elephantry::Path", raw))?;

        let mut path = Self::new(&coordinates);

        if raw.chars().next() == Some('(') {
            path.0.close();
        }

        Ok(path)
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1485
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        let mut buf = crate::not_null(raw)?;
        let closed = buf.read_u8()? == 1;
        let npts = buf.read_i32::<byteorder::BigEndian>()?;
        let mut coordinates = Vec::new();

        for _ in 0..npts {
            let x = buf.read_f64::<byteorder::BigEndian>()?;
            let y = buf.read_f64::<byteorder::BigEndian>()?;

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
