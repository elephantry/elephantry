#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde_derive::Serialize, serde_derive::Deserialize)
)]
pub struct Interval {
    pub years: u32,
    pub months: u32,
    pub days: u32,
}

impl Interval {
    pub fn new(years: u32, months: u32, days: u32) -> Self {
        Self {
            years,
            months,
            days,
        }
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} years {} months {} days",
            self.years, self.months, self.days
        )
    }
}

macro_rules! caps {
    ($caps:ident, $part:ident, $ty:ident, $raw:ident) => {
        match $caps.name(stringify!($part)) {
            Some(part) => match part.as_str().parse() {
                Ok(part) => part,
                Err(_) => return Err(Self::error($ty, "elephantry::Interval", $raw)),
            },
            None => 0,
        };
    };
}

impl crate::FromSql for crate::Interval {
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        todo!()
    }

    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let s = String::from_binary(ty, raw)?;

        if s.as_str() == "00:00:00" {
            return Ok(crate::Interval::new(0, 0, 0));
        }

        let re = regex::Regex::new(
            r"((?P<years>\d+) years?)? ?((?P<months>\d+) months?)? ?((?P<days>\d+) days?)?",
        )
        .unwrap();
        let caps = match re.captures(&s) {
            Some(caps) => caps,
            None => return Err(Self::error(ty, "elephantry::Interval", raw)),
        };

        let years = caps!(caps, years, ty, raw);
        let months = caps!(caps, months, ty, raw);
        let days = caps!(caps, days, ty, raw);

        let interval = crate::Interval::new(years, months, days);

        Ok(interval)
    }
}

impl crate::ToSql for crate::Interval {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::ty::INTERVAL
    }

    fn to_sql(&self) -> crate::Result<Option<Vec<u8>>> {
        self.to_string().to_sql()
    }
}

#[cfg(test)]
mod test {
    use crate::FromSql;

    #[test]
    fn from_binary() {
        let tests = vec![
            ("00:00:00", crate::Interval::new(0, 0, 0)),
            ("1 year", crate::Interval::new(1, 0, 0)),
            ("1 years", crate::Interval::new(1, 0, 0)),
            ("1 month", crate::Interval::new(0, 1, 0)),
            ("1 year 10 days", crate::Interval::new(1, 0, 10)),
        ];

        for (value, expected) in tests {
            assert_eq!(
                crate::Interval::from_binary(&crate::pq::ty::INTERVAL, Some(&value.as_bytes()))
                    .unwrap(),
                expected,
            );
        }
    }
}
