impl crate::ToSql for bigdecimal::BigDecimal {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::NUMERIC
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

impl crate::FromSql for bigdecimal::BigDecimal {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        crate::not_null(raw)?
            .parse()
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

        const NBASE: f64 = 10_000.;
        const DEC_DIGITS: i32 = 4;

        let mut buf = crate::not_null(raw)?;
        let ndigits = buf.read_i16::<byteorder::BigEndian>()? as i32;
        let weight = buf.read_i16::<byteorder::BigEndian>()? as i32;
        let sign = buf.read_i16::<byteorder::BigEndian>()? as i32;
        let dscale = buf.read_i16::<byteorder::BigEndian>()?;

        let mut result = bigdecimal::BigDecimal::default();

        if ndigits == 0 {
            return Ok(result);
        }

        let first_digit = buf.read_i16::<byteorder::BigEndian>()?;
        result += bigdecimal::BigDecimal::from(
            first_digit as f64 * NBASE.powi(weight),
        );

        for x in 1..ndigits {
            let digit = buf.read_i16::<byteorder::BigEndian>()?;

            if x < weight {
                result *= bigdecimal::BigDecimal::from(NBASE);
                result += bigdecimal::BigDecimal::from(digit);
            }
            else {
                assert_ne!(dscale, 0);

                result += bigdecimal::BigDecimal::from(
                    digit as f32 * 10_f32.powi(-DEC_DIGITS * (x - weight)),
                );
            }
        }

        result = match sign {
            0 => result,
            0x4000 => -result,
            0xC000 => return Err(Self::error(ty, "numeric", raw)),
            _ => return Err(Self::error(ty, "numeric", raw)),
        };

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(numeric, bigdecimal::BigDecimal, [
        ("20000", bigdecimal::BigDecimal::from(20_000.)),
        ("20000.000001", bigdecimal::BigDecimal::from(20_000.000001)),
        ("3900", bigdecimal::BigDecimal::from(3_900.)),
        ("3900.98", bigdecimal::BigDecimal::from(3_900.98)),
        ("-0.12345", bigdecimal::BigDecimal::from(-0.12345)),
    ]);
}
