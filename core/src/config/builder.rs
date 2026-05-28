pub struct Builder {
    config: super::Config,
}

impl Builder {
    #[must_use]
    pub(crate) fn new() -> Self {
        Self {
            config: super::Config::default(),
        }
    }

    #[must_use]
    pub fn build(self) -> super::Config {
        self.config
    }

    pub fn application_name<S: ToString>(mut self, application_name: S) -> Self {
        self.config.application_name = Some(application_name.to_string());

        self
    }

    pub fn channel_binding(mut self, channel_binding: super::ChannelBinding) -> Self {
        self.config.channel_binding = Some(channel_binding);

        self
    }

    pub fn client_encoding<S: ToString>(mut self, client_encoding: S) -> Self {
        self.config.client_encoding = Some(client_encoding.to_string());

        self
    }

    pub fn connect_timeout(mut self, connect_timeout: i32) -> Self {
        self.config.connect_timeout = Some(connect_timeout);

        self
    }

    pub fn dbname<S: ToString>(mut self, dbname: S) -> Self {
        self.config.dbname = Some(dbname.to_string());

        self
    }

    pub fn fallback_application_name<S: ToString>(mut self, fallback_application_name: S) -> Self {
        self.config.fallback_application_name = Some(fallback_application_name.to_string());

        self
    }

    #[cfg(feature = "pg16")]
    pub fn gssdelegation<S: ToString>(mut self, gssdelegation: S) -> Self {
        self.config.gssdelegation = Some(gssdelegation.to_string());

        self
    }

    pub fn gssencmode(mut self, gssencmode: super::GssEncMode) -> Self {
        self.config.gssencmode = Some(gssencmode);

        self
    }

    pub fn gsslib<S: ToString>(mut self, gsslib: S) -> Self {
        self.config.gsslib = Some(gsslib.to_string());

        self
    }

    pub fn hostaddr<S: ToString>(mut self, hostaddr: S) -> Self {
        self.config.hostaddr = Some(hostaddr.to_string());

        self
    }

    pub fn host<S: ToString>(mut self, host: S) -> Self {
        self.config.host = Some(host.to_string());

        self
    }

    pub fn keepalives_count(mut self, keepalives_count: i32) -> Self {
        self.config.keepalives_count = Some(keepalives_count);

        self
    }

    pub fn keepalives_idle(mut self, keepalives_idle: i32) -> Self {
        self.config.keepalives_idle = Some(keepalives_idle);

        self
    }

    pub fn keepalives_interval(mut self, keepalives_interval: i32) -> Self {
        self.config.keepalives_interval = Some(keepalives_interval);

        self
    }

    pub fn keepalives(mut self, keepalives: bool) -> Self {
        self.config.keepalives = Some(keepalives);

        self
    }

    pub fn krbsrvname<S: ToString>(mut self, krbsrvname: S) -> Self {
        self.config.krbsrvname = Some(krbsrvname.to_string());

        self
    }

    #[cfg(feature = "pg16")]
    pub fn load_balance_hosts(mut self, load_balance_hosts: super::LoadBalanceHosts) -> Self {
        self.config.load_balance_hosts = Some(load_balance_hosts);

        self
    }

    #[cfg(feature = "pg18")]
    pub fn max_protocol_version<S: ToString>(mut self, max_protocol_version: S) -> Self {
        self.config.max_protocol_version = Some(max_protocol_version.to_string());

        self
    }

    #[cfg(feature = "pg18")]
    pub fn min_protocol_version<S: ToString>(mut self, min_protocol_version: S) -> Self {
        self.config.min_protocol_version = Some(min_protocol_version.to_string());

        self
    }

    pub fn options<S: ToString>(mut self, options: S) -> Self {
        self.config.options = Some(options.to_string());

        self
    }

    #[cfg(feature = "pg18")]
    pub fn oauth_client_id<S: ToString>(mut self, oauth_client_id: S) -> Self {
        self.config.oauth_client_id = Some(oauth_client_id.to_string());

        self
    }

    #[cfg(feature = "pg18")]
    pub fn oauth_client_secret<S: ToString>(mut self, oauth_client_secret: S) -> Self {
        self.config.oauth_client_secret = Some(oauth_client_secret.to_string());

        self
    }

    #[cfg(feature = "pg18")]
    pub fn oauth_issuer<S: ToString>(mut self, oauth_issuer: S) -> Self {
        self.config.oauth_issuer = Some(oauth_issuer.to_string());

        self
    }

    #[cfg(feature = "pg18")]
    pub fn oauth_scope<S: ToString>(mut self, oauth_scope: S) -> Self {
        self.config.oauth_scope = Some(oauth_scope.to_string());

        self
    }

    pub fn passfile<S: ToString>(mut self, passfile: S) -> Self {
        self.config.passfile = Some(passfile.to_string());

        self
    }

    pub fn password<S: ToString>(mut self, password: S) -> Self {
        self.config.password = Some(password.to_string());

        self
    }

    pub fn port<S: ToString>(mut self, port: S) -> Self {
        self.config.port = Some(port.to_string());

        self
    }

    pub fn replication<S: ToString>(mut self, replication: S) -> Self {
        self.config.replication = Some(replication.to_string());

        self
    }

    pub fn requirepeer<S: ToString>(mut self, requirepeer: S) -> Self {
        self.config.requirepeer = Some(requirepeer.to_string());

        self
    }

    #[cfg(feature = "pg16")]
    pub fn require_auth<S: ToString>(mut self, require_auth: S) -> Self {
        self.config.require_auth = Some(require_auth.to_string());

        self
    }

    #[cfg(feature = "pg18")]
    pub fn scram_client_key<S: ToString>(mut self, scram_client_key: S) -> Self {
        self.config.scram_client_key = Some(scram_client_key.to_string());

        self
    }

    #[cfg(feature = "pg18")]
    pub fn scram_server_key<S: ToString>(mut self, scram_server_key: S) -> Self {
        self.config.scram_server_key = Some(scram_server_key.to_string());

        self
    }

    pub fn service<S: ToString>(mut self, service: S) -> Self {
        self.config.service = Some(service.to_string());

        self
    }

    pub fn sslcert<S: ToString>(mut self, sslcert: S) -> Self {
        self.config.sslcert = Some(sslcert.to_string());

        self
    }

    #[cfg(feature = "pg16")]
    pub fn sslcertmode(mut self, sslcertmode: super::SslCertMode) -> Self {
        self.config.sslcertmode = Some(sslcertmode);

        self
    }

    pub fn sslcompression(mut self, sslcompression: bool) -> Self {
        self.config.sslcompression = Some(sslcompression);

        self
    }

    pub fn sslcrl<S: ToString>(mut self, sslcrl: S) -> Self {
        self.config.sslcrl = Some(sslcrl.to_string());

        self
    }

    #[cfg(feature = "pg14")]
    pub fn sslcrldir<S: ToString>(mut self, sslcrldir: S) -> Self {
        self.config.sslcrldir = Some(sslcrldir.to_string());

        self
    }

    pub fn sslkey<S: ToString>(mut self, sslkey: S) -> Self {
        self.config.sslkey = Some(sslkey.to_string());

        self
    }

    #[cfg(feature = "pg18")]
    pub fn sslkeylogfile<S: ToString>(mut self, sslkeylogfile: S) -> Self {
        self.config.sslkeylogfile = Some(sslkeylogfile.to_string());

        self
    }

    pub fn ssl_max_protocol_version<S: ToString>(mut self, ssl_max_protocol_version: S) -> Self {
        self.config.ssl_max_protocol_version = Some(ssl_max_protocol_version.to_string());

        self
    }

    pub fn ssl_min_protocol_version<S: ToString>(mut self, ssl_min_protocol_version: S) -> Self {
        self.config.ssl_min_protocol_version = Some(ssl_min_protocol_version.to_string());

        self
    }

    pub fn sslmode(mut self, sslmode: super::SslMode) -> Self {
        self.config.sslmode = Some(sslmode);

        self
    }

    #[cfg(feature = "pg17")]
    pub fn sslnegotiation(mut self, sslnegotiation: super::SslNegotiation) -> Self {
        self.config.sslnegotiation = Some(sslnegotiation);

        self
    }

    pub fn sslpassword<S: ToString>(mut self, sslpassword: S) -> Self {
        self.config.sslpassword = Some(sslpassword.to_string());

        self
    }

    pub fn sslrootcert<S: ToString>(mut self, sslrootcert: S) -> Self {
        self.config.sslrootcert = Some(sslrootcert.to_string());

        self
    }

    #[cfg(feature = "pg14")]
    pub fn sslsni(mut self, sslsni: bool) -> Self {
        self.config.sslsni = Some(sslsni);

        self
    }

    pub fn target_session_attrs(mut self, target_session_attrs: super::TargetSessionAttrs) -> Self {
        self.config.target_session_attrs = Some(target_session_attrs);

        self
    }

    pub fn tcp_user_timeout(mut self, tcp_user_timeout: i32) -> Self {
        self.config.tcp_user_timeout = Some(tcp_user_timeout);

        self
    }

    pub fn user<S: ToString>(mut self, user: S) -> Self {
        self.config.user = Some(user.to_string());

        self
    }
}
