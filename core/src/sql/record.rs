pub(crate) fn vec_to_sql(
    vec: &[&dyn crate::ToSql],
) -> crate::Result<Option<Vec<u8>>> {
    let mut data = b"(".to_vec();

    for ref field in vec {
        if let Some(mut value) = field.to_sql()? {
            value.pop();
            data.append(&mut value);
        }
        data.push(b',');
    }

    data.pop();

    data.extend_from_slice(b")\0");

    Ok(Some(data))
}

pub(crate) fn text_to_vec(
    raw: Option<&str>,
) -> crate::Result<Vec<Option<&str>>> {
    let s = crate::not_null(raw)?;

    if !s.starts_with('(') && !s.ends_with(')') {
        return Err(crate::Error::FromSql {
            pg_type: crate::pq::types::UNKNOWN,
            rust_type: "tuple".to_string(),
            value: s.to_string(),
        });
    }

    let values = s
        .trim_start_matches('(')
        .trim_end_matches(')')
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

    Ok(values)
}

/*
 * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rowtypes.c#L649
 */
pub(crate) fn binary_to_vec(
    raw: Option<&[u8]>,
) -> crate::Result<Vec<Option<&[u8]>>> {
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

    Ok(values)
}

macro_rules! tuple_impls {
    ($(
        $name:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T: crate::ToSql,)+> crate::ToSql for ($($T,)+) {
                fn ty(&self) -> crate::pq::Type {
                    crate::pq::types::RECORD
                }

                fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
                    let vec = vec![$(&self.$idx as &dyn crate::ToSql),+];

                    vec_to_sql(&vec)
                }
            }

            impl<$($T: crate::FromSql,)+> crate::FromSql for ($($T,)+) {
                fn from_text(
                    ty: &crate::pq::Type,
                    raw: Option<&str>,
                ) -> crate::Result<Self> {
                    let values = text_to_vec(raw)?;

                    if values.len() != tuple_len::tuple_len!(1, $($T,)+) {
                        return Err(
                            crate::Error::FromSql {
                                pg_type: ty.clone(),
                                rust_type: stringify!($name).to_string(),
                                value: format!("{:?}", values),
                            }
                        );
                    }

                    let tuple = (
                        $($T::from_text(ty, values[$idx])?),+
                    );

                    Ok(tuple)
                }

                fn from_binary(
                    ty: &crate::pq::Type,
                    raw: Option<&[u8]>,
                ) -> crate::Result<Self> {
                    let values = binary_to_vec(raw)?;

                    if values.len() != tuple_len::tuple_len!(1, $($T,)+) {
                        return Err(
                            crate::Error::FromSql {
                                pg_type: ty.clone(),
                                rust_type: stringify!($name).to_string(),
                                value: format!("{:?}", values),
                            }
                        );
                    }

                    let tuple = (
                        $($T::from_binary(ty, values[$idx])?),+
                    );

                    Ok(tuple)
                }
            }
        )+
    }
}

tuple_impls! {
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}

#[cfg(test)]
mod test {
    crate::sql_test_from!(record, (i32, String), [(
        "(1, 'foo')",
        (1, "foo".to_string())
    ),]);
}
