use std::collections::HashMap;

/**
 * Connections pool.
 */
#[derive(Debug)]
pub struct Pool {
    default: String,
    connections: HashMap<String, crate::Connection>,
}

impl Pool {
    /**
     * Create a new pool with a default connection.
     */
    pub fn new(url: &str) -> crate::Result<Self> {
        Self::default().add_default("default", url)
    }

    /**
     * Create a new pool with a default connection from [`Config`].
     *
     * [`Config`]: struct.Config.html
     */
    pub fn from_config(config: &crate::Config) -> crate::Result<Self> {
        Self::default().add_default("default", &config.to_string())
    }

    /**
     * Add a default connection.
     */
    pub fn add_default(self, name: &str, url: &str) -> crate::Result<Self> {
        self.add(name, url, true)
    }

    /**
     * Add a connection.
     */
    pub fn add_connection(self, name: &str, url: &str) -> crate::Result<Self> {
        self.add(name, url, false)
    }

    fn add(
        mut self,
        name: &str,
        url: &str,
        default: bool,
    ) -> crate::Result<Self> {
        self.connections
            .insert(name.to_string(), crate::Connection::new(url)?);

        if default {
            self.set_default(name)?;
        }

        Ok(self)
    }

    /**
     * Retreive the default connection.
     */
    pub fn get_default(&self) -> Option<&crate::Connection> {
        self.connections.get(&self.default)
    }

    /**
     * Set the connection `name` as default.
     */
    pub fn set_default(&mut self, name: &str) -> crate::Result<()> {
        if !self.connections.contains_key(name) {
            return Err(crate::Error::Connect {
                dsn: name.to_string(),
                message: format!("Unable to set {} connection as default, unknow connection", name),
            });
        }

        self.default = name.to_string();

        Ok(())
    }

    /**
     * Retreive the connection `name`, on `None` if not exists.
     */
    pub fn get(&self, name: &str) -> Option<&crate::Connection> {
        self.connections.get(&name.to_string())
    }

    /**
     * Remove the connection `name`.
     */
    pub fn remove(&mut self, name: &str) {
        self.connections.remove(&name.to_string());
    }
}

impl Default for Pool {
    fn default() -> Self {
        Self {
            default: String::new(),
            connections: HashMap::new(),
        }
    }
}

impl std::ops::Index<&str> for Pool {
    type Output = crate::Connection;

    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl std::ops::Deref for Pool {
    type Target = crate::Connection;

    fn deref(&self) -> &Self::Target {
        self.get_default().unwrap()
    }
}
