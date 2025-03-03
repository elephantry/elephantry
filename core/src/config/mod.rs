mod channel_binding;
mod gssencmode;
#[cfg(feature = "pg16")]
mod load_balance_hosts;
#[cfg(feature = "pg16")]
mod sslcertmode;
mod sslmode;
#[cfg(feature = "pg17")]
mod sslnegotiation;
mod target_session_attrs;

pub use channel_binding::*;
pub use gssencmode::*;
#[cfg(feature = "pg16")]
pub use load_balance_hosts::*;
#[cfg(feature = "pg16")]
pub use sslcertmode::*;
pub use sslmode::*;
#[cfg(feature = "pg17")]
pub use sslnegotiation::*;
pub use target_session_attrs::*;

/**
 * Connection configuration.
 *
 * See <https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PARAMKEYWORDS>.
 */
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub struct Config {
    pub application_name: Option<String>,
    pub channel_binding: Option<ChannelBinding>,
    pub client_encoding: Option<String>,
    pub connect_timeout: Option<i32>,
    pub dbname: Option<String>,
    pub fallback_application_name: Option<String>,
    pub gssencmode: Option<GssEncMode>,
    pub gsslib: Option<String>,
    pub hostaddr: Option<String>,
    pub host: Option<String>,
    pub keepalives_count: Option<i32>,
    pub keepalives_idle: Option<i32>,
    pub keepalives_interval: Option<i32>,
    pub keepalives: Option<bool>,
    pub krbsrvname: Option<String>,
    #[cfg(feature = "pg16")]
    pub load_balance_hosts: Option<LoadBalanceHosts>,
    pub options: Option<String>,
    pub passfile: Option<String>,
    pub password: Option<String>,
    pub port: Option<String>,
    pub replication: Option<String>,
    pub requirepeer: Option<String>,
    #[cfg(feature = "pg16")]
    pub require_auth: Option<String>,
    pub service: Option<String>,
    pub sslcert: Option<String>,
    #[cfg(feature = "pg16")]
    pub sslcertmode: Option<SslCertMode>,
    pub sslcompression: Option<bool>,
    pub sslcrl: Option<String>,
    pub sslkey: Option<String>,
    pub ssl_max_protocol_version: Option<String>,
    pub ssl_min_protocol_version: Option<String>,
    pub sslmode: Option<SslMode>,
    #[cfg(feature = "pg17")]
    pub sslnegotiation: Option<SslNegotiation>,
    pub sslpassword: Option<String>,
    pub sslrootcert: Option<String>,
    pub target_session_attrs: Option<TargetSessionAttrs>,
    pub tcp_user_timeout: Option<i32>,
    pub user: Option<String>,
}

impl Config {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

macro_rules! display {
    ($f:ident, $config:ident . $name:ident) => {
        if let Some($name) = &$config.$name {
            write!($f, "{}={} ", stringify!($name), $name)?;
        }
    };
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        display!(f, self.application_name);
        display!(f, self.channel_binding);
        display!(f, self.client_encoding);
        display!(f, self.connect_timeout);
        display!(f, self.dbname);
        display!(f, self.fallback_application_name);
        display!(f, self.gssencmode);
        display!(f, self.gsslib);
        display!(f, self.hostaddr);
        display!(f, self.host);
        display!(f, self.keepalives_count);
        display!(f, self.keepalives_idle);
        display!(f, self.keepalives_interval);
        display!(f, self.keepalives);
        display!(f, self.krbsrvname);
        #[cfg(feature = "pg16")]
        display!(f, self.load_balance_hosts);
        display!(f, self.options);
        display!(f, self.passfile);
        display!(f, self.password);
        display!(f, self.port);
        display!(f, self.replication);
        display!(f, self.requirepeer);
        #[cfg(feature = "pg16")]
        display!(f, self.require_auth);
        display!(f, self.service);
        display!(f, self.sslcert);
        #[cfg(feature = "pg16")]
        display!(f, self.sslcertmode);
        display!(f, self.sslcompression);
        display!(f, self.sslcrl);
        display!(f, self.sslkey);
        display!(f, self.ssl_max_protocol_version);
        display!(f, self.ssl_min_protocol_version);
        display!(f, self.sslmode);
        display!(f, self.sslpassword);
        display!(f, self.sslrootcert);
        display!(f, self.target_session_attrs);
        display!(f, self.tcp_user_timeout);
        display!(f, self.user);

        Ok(())
    }
}
