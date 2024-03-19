#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum SslCertMode {
    #[default]
    Allow,
    Disable,
    Require,
}

impl std::str::FromStr for SslCertMode {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sslmode = match s {
            "allow" => Self::Allow,
            "disable" => Self::Disable,
            "require" => Self::Require,
            _ => return Err(crate::Error::Parse(format!("Invalid sslcertmode: {s}"))),
        };

        Ok(sslmode)
    }
}

impl std::fmt::Display for SslCertMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Allow => "allow",
            Self::Disable => "disable",
            Self::Require => "require",
        };

        f.write_str(s)
    }
}
