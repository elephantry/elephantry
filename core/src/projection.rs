use std::collections::BTreeMap;

/**
 * Define the content of SELECT or RETURNING (projection) statements.
 */
#[derive(Debug)]
pub struct Projection {
    relation: String,
    alias: Option<String>,
    fields: BTreeMap<String, String>,
}

impl Projection {
    /**
     * Create a new projection with `fields`.
     */
    pub fn new(relation: &str, fields: &[&str]) -> Self {
        let mut map = BTreeMap::new();
        for field in fields {
            map.insert((*field).to_string(), format!("%:{field}:%"));
        }

        Self {
            relation: relation.to_string(),
            alias: None,
            fields: map,
        }
    }

    /**
     * Add alias for the relation name.
     */
    pub fn alias(mut self, alias: &str) -> Projection {
        self.alias = Some(alias.to_string());

        self
    }

    /**
     * Add a field from the projection.
     */
    pub fn add_field(mut self, name: &str, row: &str) -> Projection {
        self.fields.insert(name.to_string(), row.to_string());

        self
    }

    /**
     * Unset an existing field.
     */
    pub fn unset_field(mut self, name: &str) -> Projection {
        self.fields.remove(name);

        self
    }

    /**
     * Return the list of fields.
     */
    pub fn fields(&self) -> &BTreeMap<String, String> {
        &self.fields
    }

    /**
     * Return fields names list.
     */
    pub fn field_names(&self) -> Vec<String> {
        self.fields.keys().cloned().collect()
    }

    /**
     * Return if the given field exist.
     */
    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }
}

impl std::fmt::Display for Projection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        lazy_static::lazy_static! {
            static ref REGEX: regex::Regex = regex::Regex::new(r"%:(.*?):%").unwrap();
        }

        let relation = self.alias.as_ref().unwrap_or(&self.relation);

        let s = self
            .fields
            .iter()
            .map(|(alias, row)| {
                let field = REGEX
                    .replace_all(
                        &row.replace('"', "\\\""),
                        format!("{relation}.\"$1\"").as_str(),
                    )
                    .to_string();
                format!(r#"{field} as "{alias}""#)
            })
            .fold(String::new(), |acc, x| {
                if acc.is_empty() {
                    x
                } else {
                    format!("{acc}, {x}")
                }
            });

        write!(f, "{s}")
    }
}
