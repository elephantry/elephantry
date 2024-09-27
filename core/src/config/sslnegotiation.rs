#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum SslNegotiation {
    Direct,
    #[default]
    Postgres,
}

impl std::str::FromStr for SslNegotiation {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sslmode = match s {
            "direct" => Self::Direct,
            "postgres" => Self::Postgres,
            _ => return Err(crate::Error::Parse(format!("Invalid sslnegociation: {s}"))),
        };

        Ok(sslmode)
    }
}

impl std::fmt::Display for SslNegotiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Direct => "direct",
            Self::Postgres => "postgres",
        };

        f.write_str(s)
    }
}
