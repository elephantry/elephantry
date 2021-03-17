#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum SslMode {
    Allow,
    Disable,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl std::str::FromStr for SslMode {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sslmode = match s {
            "allow" => Self::Allow,
            "disable" => Self::Disable,
            "prefer" => Self::Prefer,
            "require" => Self::Require,
            "verify-ca" => Self::VerifyCa,
            "verify-full" => Self::VerifyFull,
            _ => return Err(crate::Error::Parse(format!("Invalid sslmode: {}", s))),
        };

        Ok(sslmode)
    }
}

impl std::fmt::Display for SslMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Allow => "allow",
            Self::Disable => "disable",
            Self::Prefer => "prefer",
            Self::Require => "require",
            Self::VerifyCa => "verify-ca",
            Self::VerifyFull => "verify-full",
        };

        f.write_str(s)
    }
}
