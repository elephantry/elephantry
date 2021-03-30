mod bounds;

use bounds::Bounds;
use std::convert::TryInto;

macro_rules! impl_range {
    ($ty:ty) => {
        impl<T: crate::ToSql> crate::ToSql for $ty {
            fn ty(&self) -> crate::pq::Type {
                Bounds::from(self).ty()
            }

            fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
                Bounds::from(self).to_text()
            }

            fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
                Bounds::from(self).to_binary()
            }
        }

        impl<T: crate::FromSql> crate::FromSql for $ty {
            fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
                Bounds::from_text(ty, raw)?
                    .try_into()
                    .map_err(|_| Self::error(ty, raw))
            }

            fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
                Bounds::from_binary(ty, raw)?
                    .try_into()
                    .map_err(|_| Self::error(ty, raw))
            }
        }

        impl<T: crate::FromSql + crate::ToSql> crate::entity::Simple for $ty {
        }
    };
}

impl_range!(std::ops::Range<T>);
impl_range!(std::ops::RangeFrom<T>);
impl_range!(std::ops::RangeTo<T>);
impl_range!((std::ops::Bound<T>, std::ops::Bound<T>));

impl<T: crate::ToSql> crate::ToSql for std::ops::RangeToInclusive<T> {
    fn ty(&self) -> crate::pq::Type {
        Bounds::from(self).ty()
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        Bounds::from(self).to_text()
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Bounds::from(self).to_binary()
    }
}

impl crate::ToSql for std::ops::RangeFull {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::UNKNOWN
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        "(,)".to_text()
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(vec![
            (bounds::Flags::LB_INF | bounds::Flags::UB_INF).bits()
        ]))
    }
}

impl crate::FromSql for std::ops::RangeFull {
    fn from_text(_: &crate::pq::Type, _: Option<&str>) -> crate::Result<Self> {
        Ok(Self)
    }

    fn from_binary(_: &crate::pq::Type, _: Option<&[u8]>) -> crate::Result<Self> {
        Ok(Self)
    }
}

impl crate::entity::Simple for std::ops::RangeFull {
}

impl<T: crate::ToSql> crate::ToSql for std::ops::RangeInclusive<T> {
    fn ty(&self) -> crate::pq::Type {
        Bounds::from(self).ty()
    }

    fn to_text(&self) -> crate::Result<Option<Vec<u8>>> {
        Bounds::from(self).to_text()
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Bounds::from(self).to_binary()
    }
}

#[cfg(test)]
mod test {
    mod full {
        crate::sql_test!(int4range, std::ops::RangeFull, [("'(,)'", ..)]);
    }

    crate::sql_test!(int4range, std::ops::Range<i32>, [("'[0, 10)'", 0_i32..10)]);

    crate::sql_test!(
        int8range,
        (std::ops::Bound<i64>, std::ops::Bound<i64>),
        [(
            "'[0, 10]'",
            (std::ops::Bound::Included(0), std::ops::Bound::Excluded(11))
        )]
    );

    #[cfg(feature = "numeric")]
    crate::sql_test!(
        numrange,
        std::ops::RangeFrom<bigdecimal::BigDecimal>,
        [("'[3900,)'", bigdecimal::BigDecimal::from(3_900)..)]
    );

    #[cfg(feature = "date")]
    crate::sql_test!(
        daterange,
        std::ops::RangeTo<chrono::NaiveDate>,
        [(
            "'[, 2010-01-01)'",
            ..chrono::NaiveDate::from_ymd(2010, 01, 01)
        )]
    );

    #[cfg(feature = "date")]
    crate::sql_test!(
        tstzrange,
        std::ops::Range<chrono::DateTime<chrono::Utc>>,
        [(
            "'[1970-01-01 00:00:00+00, 2010-01-01 00:00:00+00)'",
            chrono::DateTime::<chrono::Utc>::from_utc(
                chrono::NaiveDate::from_ymd(1970, 01, 01).and_hms(0, 0, 0),
                chrono::Utc
            )
                ..chrono::DateTime::<chrono::Utc>::from_utc(
                    chrono::NaiveDate::from_ymd(2010, 01, 01).and_hms(0, 0, 0),
                    chrono::Utc
                )
        )]
    );
}
