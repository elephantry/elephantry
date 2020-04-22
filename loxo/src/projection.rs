use std::collections::HashMap;

pub struct Projection {
    pub fields: HashMap<&'static str, crate::Row>,
}

impl Projection {
    pub fn new(fields: &HashMap<&'static str, crate::Row>) -> Self {
        Self {
            fields: fields.clone(),
        }
    }

    pub fn set_field(mut self, name: &'static str, row: crate::Row) -> Projection {
        self.fields.insert(name, row);

        self
    }

    pub fn set_field_type(mut self, name: &str, ty: crate::pq::Type) -> Projection {
        if let Some(row) = self.fields.get_mut(name) {
            row.ty = ty;
        }

        self
    }

    pub fn unset_field(mut self, name: &str) -> Projection {
        self.fields.remove(name);

        self
    }

    pub fn fields(&self) -> &HashMap<&'static str, crate::Row> {
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
                    .content
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
