#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum TargetSessionAttrs {
    Any,
    #[cfg(feature = "pg14")]
    PreferStandby,
    #[cfg(feature = "pg14")]
    Primary,
    #[cfg(feature = "pg14")]
    ReadOnly,
    ReadWrite,
    #[cfg(feature = "pg14")]
    Standby,
}

impl std::str::FromStr for TargetSessionAttrs {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sslmode = match s {
            "any" => Self::Any,
            #[cfg(feature = "pg14")]
            "prefer-standby" => Self::PreferStandby,
            #[cfg(feature = "pg14")]
            "primary" => Self::Primary,
            #[cfg(feature = "pg14")]
            "read-only" => Self::ReadOnly,
            "read-write" => Self::ReadWrite,
            #[cfg(feature = "pg14")]
            "standby" => Self::Standby,
            _ => return Err(crate::Error::Parse(format!("Invalid gssencmode: {s}"))),
        };

        Ok(sslmode)
    }
}

impl std::fmt::Display for TargetSessionAttrs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Any => "any",
            #[cfg(feature = "pg14")]
            Self::PreferStandby => "prefer-standby",
            #[cfg(feature = "pg14")]
            Self::Primary => "primary",
            #[cfg(feature = "pg14")]
            Self::ReadOnly => "read-only",
            Self::ReadWrite => "read-write",
            #[cfg(feature = "pg14")]
            Self::Standby => "standby",
        };

        f.write_str(s)
    }
}
