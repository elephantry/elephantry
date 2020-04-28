use std::collections::HashMap;

pub struct Projection {
    pub relation: String,
    pub fields: HashMap<&'static str, &'static str>,
}

impl Projection {
    pub fn new(relation: &str, fields: &HashMap<&'static str, &'static str>) -> Self {
        Self {
            relation: relation.to_string(),
            fields: fields.clone(),
        }
    }

    pub fn add_field(mut self, name: &'static str, row: &'static str) -> Projection {
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

    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }
}

impl std::fmt::Display for Projection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let regex = regex::Regex::new(r"%:(.*?):%").unwrap();

        let s = self
            .fields
            .iter()
            .map(|(alias, row)| {
                let field = regex.replace_all(
                    &row.replace("\"", "\\\""),
                    format!("{}.\"$1\"", self.relation).as_str()
                ).to_string();
                format!(r#"{} as "{}""#, field, alias)
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
