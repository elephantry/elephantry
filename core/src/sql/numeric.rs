pub use bigdecimal::BigDecimal as Numeric;

impl crate::ToSql for Numeric {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::NUMERIC
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        todo!()
    }
}

impl crate::FromSql for Numeric {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        use std::str::FromStr;

        Self::from_str(&crate::not_null!(raw))
            .map_err(|_| Self::error(ty, "numeric", raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/numeric.c#L872
     */
    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        const NBASE: i64 = 10_000;
        const DEC_DIGITS: u32 = 4;

        let mut buf = crate::not_null!(raw);
        let ndigits = buf.read_u16::<byteorder::BigEndian>()? as u32;
        let weight = buf.read_u16::<byteorder::BigEndian>()? as u32;
        let sign = buf.read_u16::<byteorder::BigEndian>()?;
        let dscale = buf.read_u16::<byteorder::BigEndian>()?;

        let mut result = Numeric::default();

        if ndigits == 0 {
            return Ok(result);
        }

        result = match sign {
            0 => result,
            0x4000 => -result,
            0xC000 => return Err(Self::error(ty, "numeric", raw)),
            _ => return Err(Self::error(ty, "numeric", raw)),
        };

        let first_digit = buf.read_i16::<byteorder::BigEndian>()?;
        result += Numeric::from(
            first_digit as i64 * NBASE.pow(weight),
        );

        for _ in 1..weight {
            let digit = buf.read_i16::<byteorder::BigEndian>()?;

            result *= Numeric::from(NBASE);
            result += Numeric::from(digit);
        }

        if dscale > 0 {
            for x in weight + 1..ndigits {
                let digit = buf.read_i16::<byteorder::BigEndian>()?;
                result += Numeric::from(
                    digit as f32 / (10_u32.pow(DEC_DIGITS) * x) as f32,
                );
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(numeric, crate::Numeric, [
        ("20000", crate::Numeric::from(20_000.)),
        ("3900", crate::Numeric::from(3_900.)),
        ("3900.98", crate::Numeric::from(3_900.98)),
    ]);
}
