#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self { x, y, r }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.r)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::ToSql for Circle {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::CIRCLE
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L4557
     */
    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L4603
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        use byteorder::WriteBytesExt;

        let mut buf = Vec::new();

        buf.write_f64::<byteorder::BigEndian>(self.x)?;
        buf.write_f64::<byteorder::BigEndian>(self.y)?;
        buf.write_f64::<byteorder::BigEndian>(self.r)?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "geo")))]
impl crate::FromSql for Circle {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L4495
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        lazy_static::lazy_static! {
            static ref REGEX: regex::Regex = regex::Regex::new(r"([\d\.]+)").unwrap();
        }

        let mut matches = REGEX.find_iter(crate::not_null(raw)?);
        let x = crate::Coordinates::coordinate(&matches.next())
            .ok_or_else(|| Self::error(ty, "elephantry::Circle", raw))?;
        let y = crate::Coordinates::coordinate(&matches.next())
            .ok_or_else(|| Self::error(ty, "elephantry::Circle", raw))?;
        let r = crate::Coordinates::coordinate(&matches.next())
            .ok_or_else(|| Self::error(ty, "elephantry::Circle", raw))?;

        Ok(Self::new(x, y, r))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/geo_ops.c#L4579
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        let mut buf = crate::from_sql::not_null(raw)?;
        let x = buf.read_f64::<byteorder::BigEndian>()?;
        let y = buf.read_f64::<byteorder::BigEndian>()?;
        let r = buf.read_f64::<byteorder::BigEndian>()?;

        Ok(Self::new(x, y, r))
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        circle,
        crate::Circle,
        [("'0, 0, 5'", crate::Circle::new(0., 0., 5.))]
    );
}
