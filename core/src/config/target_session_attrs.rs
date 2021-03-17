#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum TargetSessionAttrs {
    Any,
    ReadWrite,
}

impl std::str::FromStr for TargetSessionAttrs {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sslmode = match s {
            "any" => Self::Any,
            "read-write" => Self::ReadWrite,
            _ => {
                return Err(crate::Error::Parse(format!(
                    "Invalid gssencmode: {}",
                    s
                )))
            },
        };

        Ok(sslmode)
    }
}

impl std::fmt::Display for TargetSessionAttrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Any => "any",
            Self::ReadWrite => "read-write",
        };

        f.write_str(s)
    }
}
