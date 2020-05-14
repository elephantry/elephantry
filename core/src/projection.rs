use std::collections::HashMap;

#[derive(Debug)]
pub struct Projection {
    pub relation: String,
    pub alias: Option<String>,
    pub fields: HashMap<String, String>,
}

impl Projection {
    pub fn new(relation: &str, fields: &[&str]) -> Self {
        let mut map = HashMap::new();
        for field in fields {
            map.insert((*field).to_string(), format!("%:{}:%", field));
        }

        Self {
            relation: relation.to_string(),
            alias: None,
            fields: map,
        }
    }

    pub fn alias(mut self, alias: &str) -> Projection {
        self.alias = Some(alias.to_string());

        self
    }

    pub fn add_field(mut self, name: &str, row: &str) -> Projection {
        self.fields.insert(name.to_string(), row.to_string());

        self
    }

    pub fn unset_field(mut self, name: &str) -> Projection {
        self.fields.remove(name);

        self
    }

    pub fn fields(&self) -> &HashMap<String, String> {
        &self.fields
    }

    pub fn fields_name(&self) -> Vec<String> {
        self.fields.keys().cloned().collect()
    }

    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }
}

impl std::fmt::Display for Projection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let regex = regex::Regex::new(r"%:(.*?):%").unwrap();
        let relation = self.alias.as_ref().unwrap_or(&self.relation);

        let s = self
            .fields
            .iter()
            .map(|(alias, row)| {
                let field = regex
                    .replace_all(
                        &row.replace("\"", "\\\""),
                        format!("{}.\"$1\"", relation).as_str(),
                    )
                    .to_string();
                format!(r#"{} as "{}""#, field, alias)
            })
            .fold(String::new(), |acc, x| {
                if acc.is_empty() {
                    x
                }
                else {
                    format!("{}, {}", acc, x)
                }
            });

        write!(f, "{}", s)
    }
}
