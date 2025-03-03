/**
 * Rust type for
 * [line](https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-LINE).
 */
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Line {
    #[must_use]
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.a, self.b, self.c)
    }
}

impl crate::ToSql for Line {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::LINE
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L996
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1034
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let circle = crate::Circle::new(self.a, self.b, self.c);

        circle.to_binary()
    }
}

impl crate::FromSql for Line {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L958
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let circle = crate::Circle::from_text(ty, raw)?;

        Ok(Self::new(circle.x, circle.y, circle.r))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L1011
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let circle = crate::Circle::from_binary(ty, raw)?;

        Ok(Self::new(circle.x, circle.y, circle.r))
    }
}

impl crate::entity::Simple for Line {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        line,
        crate::Line,
        [("'{1, 2, 3}'", crate::Line::new(1., 2., 3.))]
    );
}
