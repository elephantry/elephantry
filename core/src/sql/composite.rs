pub trait Composite {
    fn name() -> &'static str;
    fn to_vec(&self) -> Vec<&dyn crate::ToSql>;
    fn from_text_values(
        ty: &crate::pq::Type,
        values: &[Option<&str>],
    ) -> crate::Result<Box<Self>>;
    fn from_binary_values(
        ty: &crate::pq::Type,
        values: &[Option<&[u8]>],
    ) -> crate::Result<Box<Self>>;
}

impl<C: Composite> crate::ToSql for C {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::Type {
            oid: 0,
            descr: Self::name(),
            name: Self::name(),
            kind: libpq::types::Kind::Composite,
        }
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut data = "(".as_bytes().to_vec();

        for field in self.to_vec() {
            if let Some(mut value) = field.to_sql()? {
                value.pop();
                data.append(&mut value);
            }
            data.push(b',');
        }

        data.pop();

        data.extend_from_slice(")\0".as_bytes());

        Ok(Some(data))
    }
}

impl<C: Composite> crate::FromSql for C {
    fn from_text(
        ty: &crate::pq::Type,
        raw: Option<&str>,
    ) -> crate::Result<Self> {
        let s = crate::not_null(raw)?
            .trim_start_matches('(')
            .trim_end_matches(')');
        let values = s
            .split(',')
            .map(|x| {
                if x.is_empty() {
                    None
                }
                else {
                    Some(x)
                }
            })
            .collect::<Vec<_>>();

        Self::from_text_values(ty, &values).map(|x| *x)
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rowtypes.c#L649
     */
    fn from_binary(
        ty: &crate::pq::Type,
        raw: Option<&[u8]>,
    ) -> crate::Result<Self> {
        use byteorder::ReadBytesExt;

        let mut data = raw.unwrap();
        let mut values: Vec<Option<&[u8]>> = Vec::new();

        let validcols = data.read_i32::<byteorder::BigEndian>()?;

        for _ in 0..validcols {
            let _column_type = data.read_i32::<byteorder::BigEndian>()?;
            let length = data.read_i32::<byteorder::BigEndian>()?;

            if length < 0 {
                values.push(None);
                continue;
            }

            let value = &data[..length as usize];
            values.push(Some(value));
            data = &data[length as usize..];
        }

        Self::from_binary_values(ty, &values).map(|x| *x)
    }
}

#[cfg(test)]
mod test {
    #[derive(crate::Composite, Debug, PartialEq)]
    #[composite(internal)]
    struct CompFoo {
        f1: i32,
        f2: String,
    }

    crate::sql_test!(compfoo, super::CompFoo, [(
        "'(1,foo)'",
        super::CompFoo {
            f1: 1,
            f2: "foo".to_string()
        }
    )]);
}
