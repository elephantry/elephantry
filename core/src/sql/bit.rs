impl crate::ToSql for u8 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::BIT
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L146
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        let bytes = Bits::from_bytes(&[self.reverse_bits()]);

        bytes.to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L330
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let bytes = Bits::from_bytes(&[self.reverse_bits()]);

        bytes.to_binary()
    }
}

impl crate::FromSql for u8 {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L279
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let bytes = Bits::from_text(ty, raw)?;

        bytes
            .get(0)
            .map(u8::from)
            .ok_or_else(|| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L375
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let bytes = Bits::from_binary(ty, raw)?;

        bytes
            .get(0)
            .map(u8::from)
            .ok_or_else(|| Self::error(ty, raw))
    }
}

impl crate::entity::Simple for u8 {}

impl<const N: usize> crate::ToSql for [u8; N] {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::BIT
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L146
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        let bytes = Bits::from_bytes(self);

        bytes.to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L330
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let bytes = Bits::from_bytes(self);

        bytes.to_binary()
    }
}

impl<const N: usize> crate::FromSql for [u8; N] {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L279
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let bytes = Bits::from_text(ty, raw)?;

        bytes
            .to_bytes()
            .as_slice()
            .try_into()
            .map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L375
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let bytes = Bits::from_binary(ty, raw)?;

        bytes
            .to_bytes()
            .as_slice()
            .try_into()
            .map_err(|_| Self::error(ty, raw))
    }
}

impl<const N: usize> crate::entity::Simple for [u8; N] {}

/**
 * Rust type for [varbit](https://www.postgresql.org/docs/current/datatype.html).
 */
pub type Bits = bit_vec::BitVec;

impl crate::ToSql for Bits {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::VARBIT
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L451
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        format!("b{self}").to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L635
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        crate::to_sql::write_i32(&mut buf, self.len() as i32)?;

        for byte in self.to_bytes() {
            buf.push(byte);
        }

        Ok(Some(buf))
    }
}

impl crate::FromSql for Bits {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L586
     */
    fn from_text(_: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        let s = crate::from_sql::not_null(raw)?;
        let mut bits = Bits::from_elem(s.len(), false);

        for (x, bit) in s.chars().enumerate() {
            if bit == '1' {
                bits.set(x, true);
            }
        }

        Ok(bits)
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/varbit.c#L680
     */
    fn from_binary(_: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;

        let _size = crate::from_sql::read_i32(&mut buf)?;

        Ok(Bits::from_bytes(buf))
    }
}

impl crate::entity::Simple for Bits {}

#[cfg(test)]
mod test {
    crate::sql_test!(bit, u8, [("'0'", 0), ("'1'", 1), ("0", 0), ("1", 1)]);

    #[test]
    fn byte() -> crate::Result {
        let tests = [("'00000000'", [0]), ("'11111111'", [255])];

        crate::test::from_text("bit(8)", &tests)?;
        crate::test::from_binary("bit(8)", &tests)?;
        crate::test::to_text("bit(8)", &tests)?;
        crate::test::to_binary("bit(8)", &tests)?;

        Ok(())
    }

    #[test]
    fn bytes() -> crate::Result {
        let tests = [(
            "'1111111110000000010000000010000000010000'",
            [255, 128, 64, 32, 16],
        )];

        crate::test::from_text("bit(40)", &tests)?;
        crate::test::from_binary("bit(40)", &tests)?;
        crate::test::to_text("bit(40)", &tests)?;
        crate::test::to_binary("bit(40)", &tests)?;

        Ok(())
    }

    crate::sql_test!(
        varbit,
        crate::Bits,
        [
            ("'00000000'", crate::Bits::from_bytes(&[0b0000_0000])),
            ("'11110000'", crate::Bits::from_bytes(&[0b1111_0000])),
            ("'10101010'", crate::Bits::from_bytes(&[0b1010_1010])),
            ("'11111111'", crate::Bits::from_bytes(&[0b1111_1111])),
        ]
    );
}
