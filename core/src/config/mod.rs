mod builder;
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

use builder::Builder;

/**
 * Connection configuration.
 *
 * See <https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PARAMKEYWORDS>.
 */
#[derive(Clone, Debug, Default, PartialEq, envir::Deserialize)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub struct Config {
    #[envir(name = "PGAPPNAME")]
    pub application_name: Option<String>,
    #[envir(name = "PGCHANNELBINDING")]
    pub channel_binding: Option<ChannelBinding>,
    #[envir(name = "PGCLIENTENCODING")]
    pub client_encoding: Option<String>,
    #[envir(name = "PGCONNECT_TIMEOUT")]
    pub connect_timeout: Option<i32>,
    #[envir(name = "PGDATABASE")]
    pub dbname: Option<String>,
    pub fallback_application_name: Option<String>,
    #[envir(name = "PGGSSDELEGATION")]
    #[cfg(feature = "pg16")]
    pub gssdelegation: Option<String>,
    #[envir(name = "PGGSSENCMODE")]
    pub gssencmode: Option<GssEncMode>,
    #[envir(name = "PGGSSLIB")]
    pub gsslib: Option<String>,
    #[envir(name = "PGHOSTADDR")]
    pub hostaddr: Option<String>,
    #[envir(name = "PGHOST")]
    pub host: Option<String>,
    pub keepalives_count: Option<i32>,
    pub keepalives_idle: Option<i32>,
    pub keepalives_interval: Option<i32>,
    pub keepalives: Option<bool>,
    #[envir(name = "PGKRBSRVNAME")]
    pub krbsrvname: Option<String>,
    #[cfg(feature = "pg16")]
    #[envir(name = "PGLOADBALANCEHOSTS")]
    pub load_balance_hosts: Option<LoadBalanceHosts>,
    #[envir(name = "PGMAXPROTOCOLVERSION")]
    #[cfg(feature = "pg18")]
    pub max_protocol_version: Option<String>,
    #[envir(name = "PGMINPROTOCOLVERSION")]
    #[cfg(feature = "pg18")]
    pub min_protocol_version: Option<String>,
    #[envir(name = "PGOPTIONS")]
    pub options: Option<String>,
    #[cfg(feature = "pg18")]
    pub oauth_client_id: Option<String>,
    #[cfg(feature = "pg18")]
    pub oauth_client_secret: Option<String>,
    #[cfg(feature = "pg18")]
    pub oauth_issuer: Option<String>,
    #[cfg(feature = "pg18")]
    pub oauth_scope: Option<String>,
    #[envir(name = "PGPASSFILE")]
    pub passfile: Option<String>,
    #[envir(name = "PGPASSWORD")]
    pub password: Option<String>,
    #[envir(name = "PGPORT")]
    pub port: Option<String>,
    pub replication: Option<String>,
    #[envir(name = "PGREQUIREPEER")]
    pub requirepeer: Option<String>,
    #[cfg(feature = "pg16")]
    #[envir(name = "PGREQUIREAUTH")]
    pub require_auth: Option<String>,
    #[cfg(feature = "pg18")]
    pub scram_client_key: Option<String>,
    #[cfg(feature = "pg18")]
    pub scram_server_key: Option<String>,
    #[envir(name = "PGSERVICE")]
    pub service: Option<String>,
    #[envir(name = "PGSSLCERT")]
    pub sslcert: Option<String>,
    #[cfg(feature = "pg16")]
    #[envir(name = "PGSSLCERTMODE")]
    pub sslcertmode: Option<SslCertMode>,
    #[envir(name = "PGSSLCOMPRESSION")]
    pub sslcompression: Option<bool>,
    #[envir(name = "PGSSLCRL")]
    pub sslcrl: Option<String>,
    #[envir(name = "PGSSLCRLDIR")]
    #[cfg(feature = "pg14")]
    pub sslcrldir: Option<String>,
    #[envir(name = "PGSSLKEY")]
    pub sslkey: Option<String>,
    #[cfg(feature = "pg18")]
    pub sslkeylogfile: Option<String>,
    #[envir(name = "PGSSLMAXPROTOCOLVERSION")]
    pub ssl_max_protocol_version: Option<String>,
    #[envir(name = "PGSSLMINPROTOCOLVERSION")]
    pub ssl_min_protocol_version: Option<String>,
    #[envir(name = "PGSSLMODE")]
    pub sslmode: Option<SslMode>,
    #[cfg(feature = "pg17")]
    #[envir(name = "PGSSLNEGOTIATION")]
    pub sslnegotiation: Option<SslNegotiation>,
    pub sslpassword: Option<String>,
    #[envir(name = "PGSSLROOTCERT")]
    pub sslrootcert: Option<String>,
    #[envir(name = "PGSSLSNI")]
    #[cfg(feature = "pg14")]
    pub sslsni: Option<bool>,
    #[envir(name = "PGTARGETSESSIONATTRS")]
    pub target_session_attrs: Option<TargetSessionAttrs>,
    pub tcp_user_timeout: Option<i32>,
    #[envir(name = "PGUSER")]
    pub user: Option<String>,
}

impl Config {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn from_env() -> crate::Result<Self> {
        envir::from_env().map_err(crate::Error::from)
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
        #[cfg(feature = "pg16")]
        display!(f, self.gssdelegation);
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
        #[cfg(feature = "pg18")]
        display!(f, self.max_protocol_version);
        #[cfg(feature = "pg18")]
        display!(f, self.min_protocol_version);
        display!(f, self.options);
        #[cfg(feature = "pg18")]
        display!(f, self.oauth_client_id);
        #[cfg(feature = "pg18")]
        display!(f, self.oauth_client_secret);
        #[cfg(feature = "pg18")]
        display!(f, self.oauth_issuer);
        #[cfg(feature = "pg18")]
        display!(f, self.oauth_scope);
        display!(f, self.passfile);
        display!(f, self.password);
        display!(f, self.port);
        display!(f, self.replication);
        display!(f, self.requirepeer);
        #[cfg(feature = "pg16")]
        display!(f, self.require_auth);
        #[cfg(feature = "pg18")]
        display!(f, self.scram_client_key);
        #[cfg(feature = "pg18")]
        display!(f, self.scram_server_key);
        display!(f, self.service);
        display!(f, self.sslcert);
        #[cfg(feature = "pg16")]
        display!(f, self.sslcertmode);
        display!(f, self.sslcompression);
        display!(f, self.sslcrl);
        #[cfg(feature = "pg14")]
        display!(f, self.sslcrldir);
        display!(f, self.sslkey);
        #[cfg(feature = "pg18")]
        display!(f, self.sslkeylogfile);
        display!(f, self.ssl_max_protocol_version);
        display!(f, self.ssl_min_protocol_version);
        #[cfg(feature = "pg14")]
        display!(f, self.sslsni);
        display!(f, self.sslmode);
        #[cfg(feature = "pg17")]
        display!(f, self.sslnegotiation);
        display!(f, self.sslpassword);
        display!(f, self.sslrootcert);
        display!(f, self.target_session_attrs);
        display!(f, self.tcp_user_timeout);
        display!(f, self.user);

        Ok(())
    }
}

impl std::str::FromStr for Config {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let conninfo = libpq::connection::Info::from(s)?;

        conninfo.try_into()
    }
}

impl TryFrom<Vec<libpq::connection::Info>> for Config {
    type Error = crate::Error;

    fn try_from(values: Vec<libpq::connection::Info>) -> Result<Self, Self::Error> {
        let mut config = Self::default();

        for value in values {
            let Some(ref val) = value.val else {
                continue;
            };

            match value.keyword.as_str() {
                "application_name" => config.application_name = value.val,
                "channel_binding" => config.channel_binding = Some(val.parse()?),
                "client_encoding" => config.client_encoding = value.val,
                "connect_timeout" => config.connect_timeout = Some(val.parse()?),
                "dbname" => config.dbname = value.val,
                "fallback_application_name" => config.fallback_application_name = value.val,
                "gssencmode" => config.gssencmode = Some(val.parse()?),
                "gsslib" => config.gsslib = value.val,
                "hostaddr" => config.hostaddr = value.val,
                "host" => config.host = value.val,
                "keepalives_count" => config.keepalives_count = Some(val.parse()?),
                "keepalives_idle" => config.keepalives_idle = Some(val.parse()?),
                "keepalives_interval" => config.keepalives_interval = Some(val.parse()?),
                "keepalives" => config.keepalives = Some(val.parse()?),
                "krbsrvname" => config.krbsrvname = value.val,
                #[cfg(feature = "pg16")]
                "load_balance_hosts" => config.load_balance_hosts = Some(val.parse()?),
                "options" => config.options = value.val,
                "passfile" => config.passfile = value.val,
                "password" => config.password = value.val,
                "port" => config.port = value.val,
                "replication" => config.replication = value.val,
                "requirepeer" => config.requirepeer = value.val,
                #[cfg(feature = "pg16")]
                "require_auth" => config.require_auth = value.val,
                "service" => config.service = value.val,
                "sslcert" => config.sslcert = value.val,
                #[cfg(feature = "pg16")]
                "sslcertmode" => config.sslcertmode = Some(val.parse()?),
                "sslcompression" => config.sslcompression = Some(val.parse()?),
                "sslcrl" => config.sslcrl = value.val,
                "sslkey" => config.sslkey = value.val,
                "ssl_max_protocol_version" => config.ssl_max_protocol_version = value.val,
                "ssl_min_protocol_version" => config.ssl_min_protocol_version = value.val,
                "sslmode" => config.sslmode = Some(val.parse()?),
                #[cfg(feature = "pg17")]
                "sslnegotiation" => config.sslnegotiation = Some(val.parse()?),
                "sslpassword" => config.sslpassword = value.val,
                "sslrootcert" => config.sslrootcert = value.val,
                "target_session_attrs" => config.target_session_attrs = Some(val.parse()?),
                "tcp_user_timeout" => config.tcp_user_timeout = Some(val.parse()?),
                "user" => config.user = value.val,
                _ => {
                    return Err(crate::Error::Parse(format!(
                        "Invalid config field {}",
                        value.keyword
                    )));
                }
            }
        }

        Ok(config)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn builder() {
        let actual = crate::Config::builder().host("localhost").build();

        let expected = crate::Config {
            host: Some("localhost".to_string()),
            ..Default::default()
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_key_value() -> crate::Result {
        let dsn = "host=localhost port=123 application_name='my app'";

        let actual = dsn.parse()?;

        let expected = crate::Config {
            host: Some("localhost".to_string()),
            port: Some("123".to_string()),
            application_name: Some("my app".to_string()),

            ..Default::default()
        };

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn parse_uri() -> crate::Result {
        let dsn = "postgresql://host1:123,host2:456/somedb?target_session_attrs=any&application_name=myapp";

        let actual = dsn.parse()?;

        let expected = crate::Config {
            host: Some("host1,host2".to_string()),
            port: Some("123,456".to_string()),
            dbname: Some("somedb".to_string()),
            target_session_attrs: Some(crate::config::TargetSessionAttrs::Any),
            application_name: Some("myapp".to_string()),

            ..Default::default()
        };

        assert_eq!(expected, actual);

        Ok(())
    }
}
