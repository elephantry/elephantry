use std::convert::TryInto;

mod to_sql;
mod from_sql;

pub use to_sql::ToSql;
pub use from_sql::FromSql;

pub type Type = libpq::Type;
pub type Format = libpq::Format;

pub struct Connection {
    inner: libpq::Connection,
}

impl Connection {
    pub fn new(dsn: &str) -> crate::Result<Self> {
        Ok(Self {
            inner: libpq::Connection::new(dsn)?,
        })
    }

    pub fn query(&self, query: &str, params: &[&dyn ToSql]) -> crate::Result<Result> {
        if params.is_empty() {
            self.inner.exec(query).try_into()
        } else {
            let param_types = params.iter()
                .map(|x| x.ty())
                .collect::<Vec<_>>();

            let param_values = params.iter()
                .map(|x| x.to_sql().ok())
                .collect::<Vec<_>>();

            let param_formats = params.iter()
                .map(|x| x.format())
                .collect::<Vec<_>>();

            self.inner.exec_params(query, &param_types, &param_values, &param_formats, Format::Text).try_into()
        }
    }
}

pub struct Result {
    inner: libpq::Result,
    current_tuple: usize,
}

impl Result {
    pub fn get(&self, n: usize) -> Option<Tuple> {
        if n + 1 > self.len() {
            return None;
        }

        let mut values = std::collections::HashMap::new();

        for x in 0..self.inner.nfields() {
            let name = self.inner.field_name(x).unwrap();
            let value = match self.inner.value(n, x) {
                Some(value) => value,
                None => continue,
            };

            values.insert(name, value);
        }

        let tuple = Tuple::from(&values);

        Some(tuple)
    }

    pub fn len(&self) -> usize {
        self.inner.ntuples()
    }
}

impl std::iter::Iterator for Result {
    type Item = Tuple;

    fn next(&mut self) -> Option<Self::Item> {
        let tuple = self.get(self.current_tuple);
        self.current_tuple += 1;

        tuple
    }
}

impl std::convert::TryFrom<libpq::Result> for Result {
    type Error = String;

    fn try_from(inner: libpq::Result) -> crate::Result<Self> {
        use libpq::Status::*;

        match inner.status() {
            BadResponse | FatalError | NonFatalError => Err(inner.error_message().unwrap_or_else(|| "Unknow error".to_string())),
            _ => Ok(Self { inner, current_tuple: 0 }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tuple {
    values: std::collections::HashMap<String, String>,
}

impl Tuple {
    pub fn from(values: &std::collections::HashMap<String, String>) -> Self {
        Self {
            values: values.clone(),
        }
    }

    pub fn get<T>(&self, name: &str) -> crate::Result<T> where T: FromSql
    {
        let value = self.values.get(&name.to_string());

        FromSql::from_sql(&Type::TEXT, value)
    }

    pub fn get_bytes(&self, name: &str) -> Option<Vec<u8>>
    {
        self.values.get(&name.to_string())
            .map(|x| x.as_bytes().to_vec())
    }
}
