#[derive(Clone, Debug, PartialEq)]
pub struct Coordinate(geo_types::Coord<f64>);

impl Coordinate {
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self(geo_types::Coord { x, y })
    }
}

impl std::ops::Deref for Coordinate {
    type Target = geo_types::Coord<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for Coordinate {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let coordinate = Self::new(f64::arbitrary(u)?, f64::arbitrary(u)?);

        Ok(coordinate)
    }
}

#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Coordinates(Vec<Coordinate>);

impl Coordinates {
    pub(crate) fn coordinate(r#match: &Option<regex::Match<'_>>) -> Option<f64> {
        if let Some(r#match) = r#match {
            r#match.as_str().parse().ok()
        } else {
            None
        }
    }
}

impl std::ops::Deref for Coordinates {
    type Target = Vec<Coordinate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Coordinates {
    type Err = ();

    fn from_str(s: &str) -> Result<Coordinates, Self::Err> {
        let regex = crate::regex!(r"([\d\.]+)");

        let mut coordinates = Vec::new();

        let mut matches = regex.find_iter(s);

        while let Some(x) = Self::coordinate(&matches.next()) {
            let y = match Self::coordinate(&matches.next()) {
                Some(y) => y,
                None => break,
            };

            let coordinate = Coordinate::new(x, y);

            coordinates.push(coordinate);
        }

        Ok(Coordinates(coordinates))
    }
}

impl From<Vec<Coordinate>> for Coordinates {
    fn from(v: Vec<Coordinate>) -> Self {
        Self(v)
    }
}
