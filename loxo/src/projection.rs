use std::collections::HashMap;

pub struct Projection {
    pub fields: HashMap<&'static str, &'static str>,
}

impl Projection {
    pub fn new(fields: &HashMap<&'static str, &'static str>) -> Self {
        Self {
            fields: fields.clone(),
        }
    }

    pub fn set_field(mut self, name: &'static str, row: &'static str) -> Projection {
        self.fields.insert(name, row);

        self
    }

    pub fn unset_field(mut self, name: &str) -> Projection {
        self.fields.remove(name);

        self
    }

    pub fn fields(&self) -> &HashMap<&'static str, &'static str> {
        &self.fields
    }

    pub fn fields_name(&self) -> Vec<&str> {
        self.fields.keys().copied().collect()
    }
}

impl std::fmt::Display for Projection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .fields
            .iter()
            .map(|(alias, row)| {
                let content = row
                    .replace("\"", "\\\"")
                    .replace("%:", "\"")
                    .replace(":%", "\"");
                format!(r#"{} as "{}""#, content, alias)
            })
            .fold(String::new(), |acc, x| {
                if acc.is_empty() {
                    x
                } else {
                    format!("{}, {}", acc, x)
                }
            });

        write!(f, "{}", s)
    }
}
