impl crate::ToSql for u8 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::BIT
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl crate::FromSql for u8 {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        crate::not_null!(raw)
            .parse()
            .map_err(|_| Self::error(ty, "bit", raw))
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        let mut buf = crate::not_null!(raw);
        let _size = buf.read_i32::<byteorder::BigEndian>()?;
        let v = match buf.read_u8()? {
            0 => 0,
            128 => 1,
            _ => return Err(Self::error(ty, stringify!($type), raw)),
        };

        if !buf.is_empty() {
            return Err(Self::error(ty, stringify!($type), raw));
        }

        Ok(v)

    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(bit, u8, [
        ("'0'", 0),
        ("'1'", 1),
        ("0", 0),
        ("1", 1),
    ]);
}
