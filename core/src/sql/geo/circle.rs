/**
 * Rust type for
 * [circle](https://www.postgresql.org/docs/current/datatype-geometric.html#DATATYPE-CIRCLE).
 */
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl Circle {
    #[must_use]
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self { x, y, r }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.r)
    }
}

impl crate::ToSql for Circle {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::CIRCLE
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L4557
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L4603
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        crate::to_sql::write_f64(&mut buf, self.x)?;
        crate::to_sql::write_f64(&mut buf, self.y)?;
        crate::to_sql::write_f64(&mut buf, self.r)?;

        Ok(Some(buf))
    }
}

impl crate::FromSql for Circle {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L4495
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let regex = crate::regex!(r"([\d\.]+)");

        let mut matches = regex.find_iter(crate::from_sql::not_null(raw)?);
        let x =
            crate::Coordinates::coordinate(&matches.next()).ok_or_else(|| Self::error(ty, raw))?;
        let y =
            crate::Coordinates::coordinate(&matches.next()).ok_or_else(|| Self::error(ty, raw))?;
        let r =
            crate::Coordinates::coordinate(&matches.next()).ok_or_else(|| Self::error(ty, raw))?;

        Ok(Self::new(x, y, r))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L4579
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;

        let x = crate::from_sql::read_f64(&mut buf)?;
        let y = crate::from_sql::read_f64(&mut buf)?;
        let r = crate::from_sql::read_f64(&mut buf)?;

        Ok(Self::new(x, y, r))
    }
}

impl crate::entity::Simple for Circle {}

#[cfg(test)]
mod test {
    crate::sql_test!(
        circle,
        crate::Circle,
        [("'0, 0, 5'", crate::Circle::new(0., 0., 5.))]
    );
}
