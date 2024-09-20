use std::convert::{TryFrom, TryInto};

/**
 * Rust type for
 * [numeric](https://www.postgresql.org/docs/current/datatype-numeric.html).
 */
#[cfg_attr(docsrs, doc(cfg(feature = "numeric")))]
pub type Numeric = bigdecimal::BigDecimal;

#[cfg_attr(docsrs, doc(cfg(feature = "numeric")))]
impl crate::ToSql for Numeric {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::NUMERIC
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/numeric.c#L655
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        self.to_string().to_text()
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/numeric.c#L872
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let numeric = PgNumeric::try_from(self)?;

        let mut buf = Vec::new();
        crate::to_sql::write_i16(&mut buf, numeric.ndigits())?;
        crate::to_sql::write_i16(&mut buf, numeric.weight)?;
        crate::to_sql::write_i16(&mut buf, numeric.sign)?;
        crate::to_sql::write_i16(&mut buf, numeric.dscale)?;
        for digit in numeric.digits {
            crate::to_sql::write_i16(&mut buf, digit)?;
        }

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "numeric")))]
impl crate::FromSql for Numeric {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/numeric.c#L573
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        crate::from_sql::not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/numeric.c#L805
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let mut buf = crate::from_sql::not_null(raw)?;
        let ndigits = crate::from_sql::read_i16(&mut buf)?;
        let weight = crate::from_sql::read_i16(&mut buf)?;
        let sign = crate::from_sql::read_i16(&mut buf)?;
        let dscale = crate::from_sql::read_i16(&mut buf)?;

        let mut digits = Vec::new();
        for _ in 0..ndigits {
            digits.push(crate::from_sql::read_i16(&mut buf)?);
        }

        let numeric = PgNumeric {
            weight,
            sign,
            dscale,
            digits,
        };

        numeric.try_into().map_err(|_| Self::error(ty, raw))
    }
}

impl crate::entity::Simple for Numeric {}

/*
 * Credits: [Diesel](https://diesel.rs/).
 */

use num::integer::Integer;
use num::ToPrimitive;
use num::Zero;

const NUMERIC_POS: i16 = 0x0000;
const NUMERIC_NEG: i16 = 0x4000;

#[derive(Debug)]
struct PgNumeric {
    weight: i16,
    sign: i16,
    dscale: i16,
    digits: Vec<i16>,
}

impl PgNumeric {
    fn ndigits(&self) -> i16 {
        self.digits.len() as i16
    }
}

impl TryFrom<PgNumeric> for Numeric {
    type Error = ();

    fn try_from(value: PgNumeric) -> Result<Self, Self::Error> {
        let sign = match value.sign {
            NUMERIC_POS => num::bigint::Sign::Plus,
            NUMERIC_NEG => num::bigint::Sign::Minus,
            _ => return Err(()),
        };

        let mut result = num::bigint::BigUint::default();
        let count = value.ndigits() as i64;
        for digit in value.digits {
            result *= num::bigint::BigUint::from(10_000u64);
            result += num::bigint::BigUint::from(digit as u64);
        }
        // First digit got factor 10_000^(digits.len() - 1), but should get 10_000^weight
        let correction_exp = 4 * (i64::from(value.weight) - count + 1);
        let result = Numeric::new(
            num::bigint::BigInt::from_biguint(sign, result),
            -correction_exp,
        )
        .with_scale(i64::from(value.dscale));

        Ok(result)
    }
}

impl TryFrom<&Numeric> for PgNumeric {
    type Error = crate::Error;

    fn try_from(value: &Numeric) -> Result<Self, Self::Error> {
        use num::Signed;

        let (mut integer, dscale) = value.as_bigint_and_exponent();

        // Handling of negative dscale
        let dscale = if dscale < 0 {
            for _ in 0..(-dscale) {
                integer *= 10;
            }
            0
        } else {
            dscale as i16
        };

        integer = integer.abs();

        // Ensure that the decimal will always lie on a digit boundary
        for _ in 0..(4 - dscale % 4) {
            integer *= 10;
        }
        let integer = integer.to_biguint().expect("integer is always positive");

        let mut digits = ToBase10000(Some(integer)).collect::<Vec<_>>();
        digits.reverse();
        let digits_after_decimal = dscale / 4 + 1;
        let weight = digits.len() as i16 - digits_after_decimal - 1;

        let unnecessary_zeroes = digits.iter().rev().take_while(|i| i.is_zero()).count();

        let relevant_digits = digits.len() - unnecessary_zeroes;
        digits.truncate(relevant_digits);

        let sign = match value.sign() {
            num::bigint::Sign::Minus => NUMERIC_NEG,
            _ => NUMERIC_POS,
        };

        let numeric = PgNumeric {
            weight,
            sign,
            dscale,
            digits,
        };

        Ok(numeric)
    }
}

struct ToBase10000(Option<num::bigint::BigUint>);

impl Iterator for ToBase10000 {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|v| {
            let (div, rem) = v.div_rem(&num::bigint::BigUint::from(10_000u16));
            if !div.is_zero() {
                self.0 = Some(div);
            }
            rem.to_i16().expect("10000 always fits in an i16")
        })
    }
}

#[cfg(test)]
mod test {
    crate::sql_test!(
        numeric,
        crate::Numeric,
        [
            ("20000", crate::Numeric::from(20_000)),
            (
                "20000.0000019073486328125",
                crate::Numeric::try_from(20_000.000_001_907_348_632_812_5).unwrap()
            ),
            ("3900", crate::Numeric::from(3_900)),
            ("3900.5", crate::Numeric::try_from(3_900.5).unwrap()),
            ("-0.4375", crate::Numeric::try_from(-0.4375).unwrap()),
        ]
    );
}
