#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum ChannelBinding {
    Disable,
    Prefer,
    Require,
}

impl std::str::FromStr for ChannelBinding {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sslmode = match s {
            "disable" => Self::Disable,
            "prefer" => Self::Prefer,
            "require" => Self::Require,
            _ => {
                return Err(crate::Error::Parse(format!(
                    "Invalid channel_binding: {}",
                    s
                )))
            }
        };

        Ok(sslmode)
    }
}

impl std::fmt::Display for ChannelBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Disable => "disable",
            Self::Prefer => "prefer",
            Self::Require => "require",
        };

        f.write_str(s)
    }
}
