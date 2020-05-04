use std::collections::HashMap;

#[derive(Debug)]
pub struct Pool {
    default: String,
    connections: HashMap<String, crate::Connection>,
}

impl Pool {
    pub fn new(url: &str) -> crate::Result<Self> {
        Self::default().add_default("default", url)
    }

    pub fn add_default(self, name: &str, url: &str) -> crate::Result<Self> {
        self.add(name, url, true)
    }

    pub fn add_connection(self, name: &str, url: &str) -> crate::Result<Self> {
        self.add(name, url, false)
    }

    fn add(mut self, name: &str, url: &str, default: bool) -> crate::Result<Self> {
        self.connections
            .insert(name.to_string(), crate::Connection::new(url)?);

        if default {
            self.set_default(name);
        }

        Ok(self)
    }

    pub fn get_default(&self) -> Option<&crate::Connection> {
        self.connections.get(&self.default)
    }

    pub fn set_default(&mut self, name: &str) {
        self.default = name.to_string();
    }

    pub fn get(&self, name: &str) -> Option<&crate::Connection> {
        self.connections.get(&name.to_string())
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
