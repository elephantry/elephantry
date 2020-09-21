#[derive(Clone, Debug, PartialEq)]
pub struct Coordinate(geo_types::Coordinate<f64>);

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Self {
        Self(geo_types::Coordinate {
            x,
            y,
        })
    }
}

impl std::ops::Deref for Coordinate {
    type Target = geo_types::Coordinate<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Coordinates(Vec<Coordinate>);

impl Coordinates {
    pub(crate) fn coordinate(
        r#match: &Option<regex::Match<'_>>,
    ) -> Option<f64> {
        if let Some(r#match) = r#match {
            r#match.as_str().parse().ok()
        }
        else {
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
        lazy_static::lazy_static! {
            static ref REGEX: regex::Regex = regex::Regex::new(r"([\d\.]+)").unwrap();
        }

        let mut coordinates = Vec::new();

        let mut matches = REGEX.find_iter(s);

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
