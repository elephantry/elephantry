pub use bit_vec::BitVec;

impl crate::ToSql for u8 {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::BIT
    }

    fn format(&self) -> crate::pq::Format {
        crate::pq::Format::Binary
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        let bytes = BitVec::from_bytes(&[*self]);

        bytes.to_sql()
    }
}

impl crate::FromSql for u8 {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let bytes = BitVec::from_text(ty, raw)?;

        Ok(bytes.get(0).unwrap() as u8)
    }

    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        let bytes = BitVec::from_binary(ty, raw)?;

        Ok(bytes.get(0).unwrap() as u8)
    }
}

impl crate::ToSql for BitVec {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::VARBIT
    }

    fn format(&self) -> crate::pq::Format {
        crate::pq::Format::Binary
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        use bytes::BufMut;

        let mut buf = bytes::BytesMut::new();

        buf.put_u32(self.len() as u32);
        for byte in self.to_bytes() {
            buf.put_u8(byte);
        }

        Ok(Some(buf.to_vec()))
    }
}

impl crate::FromSql for BitVec {
    fn from_text(
        _: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let s = crate::not_null!(raw);
        let mut bits = BitVec::from_elem(s.len(), false);

        for (x, bit) in s.chars().enumerate() {
            if bit == '1' {
                bits.set(x, true);
            }
        }

        Ok(bits)
    }

    fn from_binary(
        _: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        let mut buf = crate::not_null!(raw);
        let _size = buf.read_i32::<byteorder::BigEndian>()?;

        Ok(BitVec::from_bytes(buf))
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

    crate::sql_test!(varbit, crate::BitVec, [
        ("'00000000'", crate::BitVec::from_bytes(&[0b00000000])),
        ("'10101010'", crate::BitVec::from_bytes(&[0b10101010])),
        ("'11111111'", crate::BitVec::from_bytes(&[0b11111111])),
    ]);
}
