#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum LoadBalanceHosts {
    #[default]
    Disable,
    Random,
}

impl std::str::FromStr for GssEncMode {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sslmode = match s {
            "disable" => Self::Disable,
            "random" => Self::Random,
            _ => {
                return Err(crate::Error::Parse(format!(
                    "Invalid load_balance_hosts: {s}"
                )))
            }
        };

        Ok(sslmode)
    }
}

impl std::fmt::Display for GssEncMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Disable => "disable",
            Self::Random => "random",
        };

        f.write_str(s)
    }
}
