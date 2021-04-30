#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Line {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.a, self.b, self.c)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::ToSql for Line {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::LINE
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::FromSql for Line {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let circle = crate::Circle::from_text(ty, raw)?;

        Ok(Self::new(circle.x, circle.y, circle.r))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1034
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let circle = crate::Circle::from_binary(ty, raw)?;

        Ok(Self::new(circle.x, circle.y, circle.r))
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        line,
        crate::Line,
        [("'{1, 2, 3}'", crate::Line::new(1., 2., 3.))]
    );
}
