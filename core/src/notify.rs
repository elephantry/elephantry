#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Notify {
    pub relname: String,
    pub be_pid: u32,
    pub extra: String,
}

impl TryFrom<libpq::connection::Notify> for Notify {
    type Error = crate::Error;

    fn try_from(value: libpq::connection::Notify) -> Result<Self, Self::Error> {
        let notify = Self {
            relname: value.relname()?,
            be_pid: value.be_pid(),
            extra: value.extra()?,
        };

        Ok(notify)
    }
}
