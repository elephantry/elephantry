/*
 * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rowtypes.c#L74
 */
#[doc(hidden)]
pub fn vec_to_text(vec: &[&dyn crate::ToSql]) -> crate::Result<Option<Vec<u8>>> {
    let mut data = b"(".to_vec();

    for field in vec {
        if let Some(mut value) = field.to_text()? {
            value.pop();
            data.append(&mut value);
        }
        data.push(b',');
    }

    data.pop();

    data.extend_from_slice(b")\0");

    Ok(Some(data))
}

/*
 * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rowtypes.c#L649
 */
#[doc(hidden)]
pub fn vec_to_binary(vec: &[&dyn crate::ToSql]) -> crate::Result<Option<Vec<u8>>> {
    use crate::ToSql;

    let mut buf = Vec::new();

    crate::to_sql::write_i32(&mut buf, vec.len() as i32)?;

    for elem in vec {
        let mut column_type = elem.ty().to_binary()?.unwrap();
        buf.append(&mut column_type);

        if let Some(mut raw) = elem.to_binary()? {
            crate::to_sql::write_i32(&mut buf, raw.len() as i32)?;
            buf.append(&mut raw);
        } else {
            crate::to_sql::write_i32(&mut buf, -1)?;
        }
    }

    Ok(Some(buf))
}

/*
 * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rowtypes.c#L302
 */
#[doc(hidden)]
pub fn text_to_vec(raw: Option<&str>) -> crate::Result<Vec<Option<&str>>> {
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
        .map(|x| if x.is_empty() { None } else { Some(x) })
        .collect::<Vec<_>>();

    Ok(values)
}

/*
 * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/rowtypes.c#L453
 */
#[doc(hidden)]
pub fn binary_to_vec<'a>(
    rust_type: &'a str,
    pg_type: &crate::pq::Type,
    raw: Option<&'a [u8]>,
) -> crate::Result<Vec<Option<&'a [u8]>>> {
    let mut buf = crate::not_null(raw)?;

    let mut values: Vec<Option<&[u8]>> = Vec::new();

    let validcols = crate::from_sql::read_i32(&mut buf)?;

    let error = || crate::Error::FromSql {
        pg_type: pg_type.clone(),
        rust_type: rust_type.to_string(),
        value: format!("{:?}", raw),
    };

    for _ in 0..validcols {
        let _column_type = crate::from_sql::read_i32(&mut buf)?;
        let length = crate::from_sql::read_i32(&mut buf)?;

        if length < 0 {
            values.push(None);
            continue;
        }

        let value = &buf.get(..length as usize).ok_or_else(error)?;
        values.push(Some(value));
        buf = buf.get(length as usize..).ok_or_else(error)?;
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

                fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
                    let vec = vec![$(&self.$idx as &dyn crate::ToSql),+];

                    vec_to_text(&vec)
                }

                fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
                    let vec = vec![$(&self.$idx as &dyn crate::ToSql),+];

                    vec_to_binary(&vec)
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
                    let values = binary_to_vec(stringify!($name), ty, raw)?;

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

            impl<$($T: crate::FromSql + crate::ToSql,)+> crate::entity::Simple for ($($T,)+) {
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
    crate::sql_test_from!(
        record,
        (i32, String),
        [("(1, 'foo')", (1, "foo".to_string())),]
    );
}
