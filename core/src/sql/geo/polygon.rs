/**
 * Rust type for
 * [polygon](https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-POLYGON).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
#[derive(Clone, Debug, PartialEq)]
pub struct Polygon(geo_types::Polygon<f64>);

impl Polygon {
    pub fn new(path: &crate::Path) -> Self {
        use std::ops::Deref;

        Self(geo_types::Polygon::new(path.deref().clone(), Vec::new()))
    }
}

impl std::ops::Deref for Polygon {
    type Target = geo_types::Polygon<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for coordinate in self.0.exterior().points_iter() {
            s.push_str(&format!("({}, {}),", coordinate.x(), coordinate.y()));
        }

        write!(f, "{}", s.trim_end_matches(','))
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Polygon {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let polygon = Self::new(&crate::Path::arbitrary(u)?);

        Ok(polygon)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::ToSql for Polygon {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::POLYGON
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L3389
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L3440
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let points = &self.0.exterior().0;

        let mut buf = Vec::new();

        crate::to_sql::write_i32(&mut buf, points.len() as i32)?;

        for point in points {
            crate::to_sql::write_f64(&mut buf, point.x)?;
            crate::to_sql::write_f64(&mut buf, point.y)?;
        }

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::FromSql for Polygon {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L3348
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let path = crate::Path::from_text(ty, raw)?;

        Ok(Self::new(&path))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L3405
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;
        let npts = crate::from_sql::read_i32(&mut buf)?;
        let mut coordinates = Vec::new();

        for _ in 0..npts {
            let x = crate::from_sql::read_f64(&mut buf)?;
            let y = crate::from_sql::read_f64(&mut buf)?;

            let coordinate = crate::Coordinate::new(x, y);
            coordinates.push(coordinate);
        }

        Ok(Self::new(&crate::Path::new(&coordinates.into())))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::entity::Simple for Polygon {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        polygon,
        crate::Polygon,
        [(
            "'((0, 0), (10, 10), (10, 0), (0, 0))'",
            crate::Polygon::new(&crate::Path::new(
                &vec![
                    crate::Coordinate::new(0., 0.),
                    crate::Coordinate::new(10., 10.),
                    crate::Coordinate::new(10., 0.),
                    crate::Coordinate::new(0., 0.),
                ]
                .into()
            ))
        )]
    );
}
