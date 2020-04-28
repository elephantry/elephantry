use std::convert::TryInto;

mod from_sql;
mod to_sql;

pub use from_sql::FromSql;
pub use to_sql::ToSql;
pub use libpq::state;

pub use libpq::ty;
pub type Format = libpq::Format;
pub type Oid = libpq::Oid;
pub type Type = libpq::Type;
pub type State = libpq::State;

pub trait ToRust {
    fn to_rust(&self) -> String;
}

impl ToRust for Type {
    fn to_rust(&self) -> String {
        let rust = match self.name {
            "bigint" | "int8" => "i64",
            "bigserial" | "serial8" => "i64",
            "bit" => "bytes::Bytes",
            "bit varying" | "varbit" => "bytes::Bytes",
            "boolean" | "bool" => "bool",
            #[cfg(feature = "geo")]
            "box" => "geo_types::Rect<f64>",
            "bytea" => "Vec<u8>",
            "character" | "char" => "i8",
            "character varying" | "varchar" => "String",
            #[cfg(feature = "network")]
            "cidr" => todo!(),
            #[cfg(feature = "geo")]
            "circle" => todo!(),
            #[cfg(feature = "chrono")]
            "date" => "chrono::NaiveDate",
            "double precision" | "float8" => "f64",
            "inet" => "std::net::IpAddr",
            "integer" | "int" | "int4" => "i32",
            #[cfg(feature = "json")]
            "json" | "jsonb" => "serde::value::Value",
            #[cfg(feature = "geo")]
            "line" => todo!(),
            #[cfg(feature = "geo")]
            "lseg" => todo!(),
            #[cfg(feature = "network")]
            "macaddr" => "eui48::MacAddress",
            #[cfg(feature = "network")]
            "macaddr8" => "eui48::MacAddress",
            "money" => "f32",
            "numeric" | "decimal" => "f32",
            #[cfg(feature = "geo")]
            "path" => "geo_types::LineString<f64>",
            "pg_lsn" => "String",
            #[cfg(feature = "geo")]
            "point" => "geo_types::Point<f64>",
            #[cfg(feature = "geo")]
            "polygon" => todo!(),
            "real" | "float4" => "f32",
            "smallint" | "int2" => "i16",
            "smallserial" | "serial2" => "i16",
            "serial" | "serial4" => "i32",
            "text" => "String",
            #[cfg(feature = "chrono")]
            "time" | "time without time zone" => "chrono::NaiveTime",
            #[cfg(feature = "chrono")]
            "time with time zone" | "timetz" => todo!(),
            #[cfg(not(feature = "chrono"))]
            "timestamp" | "timestamp without time zone" => "std::time::SystemTime",
            #[cfg(feature = "chrono")]
            "timestamp" | "timestamp without time zone" => "chrono::NaiveDateTime",
            #[cfg(not(feature = "chrono"))]
            "timestamp with time zone" | "timestamptz" => "std::time::SystemTime",
            #[cfg(feature = "chrono")]
            "timestamp with time zone" | "timestamptz" => "chrono::DateTime<chrono::FixedOffset>",
            #[cfg(feature = "uuid")]
            "uuid" => "uuid::Uuid",
            "xml" => "String",

            "hstore" => "std::collection::HashMap<String, Option<String>>",

            _ => "String",
        };

        rust.to_string()
    }
}

pub struct Connection {
    inner: libpq::Connection,
}

impl Connection {
    pub fn new(dsn: &str) -> crate::Result<Self> {
        let inner = match libpq::Connection::new(dsn) {
            Ok(inner) => inner,
            Err(message) => return Err(crate::Error::Connect {
                dsn: dsn.to_string(),
                message,
            }),
        };

        inner.set_error_verbosity(libpq::Verbosity::Terse);
        inner.set_client_encoding(libpq::Encoding::UTF8);

        Ok(Self {
            inner,
        })
    }

    pub fn execute(&self, query: &str) -> crate::Result<Result> {
        self.inner.exec(query).try_into()
    }

    pub fn query(&self, query: &str, params: &[&dyn ToSql]) -> crate::Result<Result> {
        let mut param_types = Vec::new();
        let mut param_values = Vec::new();
        let mut param_formats = Vec::new();

        for param in params.iter() {
            param_types.push(param.ty());
            param_values.push(param.to_sql().ok());
            param_formats.push(param.format());
        }

        self.inner
            .exec_params(
                query,
                &param_types,
                &param_values,
                &param_formats,
                Format::Binary,
            )
            .try_into()
    }
}

#[derive(Debug)]
pub struct Result {
    inner: libpq::Result,
    current_tuple: std::cell::RefCell<usize>,
}

impl Result {
    pub fn get<'a>(&'a self, n: usize) -> Tuple<'a> {
        self.try_get(n).unwrap()
    }

    pub fn try_get<'a>(&'a self, n: usize) -> Option<Tuple<'a>> {
        if n + 1 > self.len() {
            return None;
        }

        let tuple = Tuple::from(&self.inner, n);

        Some(tuple)
    }

    pub fn len(&self) -> usize {
        self.inner.ntuples()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn state(&self) -> State {
        State::from_code(&self.inner.error_field(libpq::result::ErrorField::Sqlstate).unwrap())
    }
}

impl<'a> std::iter::Iterator for &'a Result {
    type Item = Tuple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let tuple = self.try_get(*self.current_tuple.borrow());
        *self.current_tuple.borrow_mut() += 1;

        tuple
    }
}

impl std::ops::Deref for Result {
    type Target = libpq::Result;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::convert::TryFrom<libpq::Result> for Result {
    type Error = crate::Error;

    fn try_from(inner: libpq::Result) -> crate::Result<Self> {
        use libpq::Status::*;

        match inner.status() {
            BadResponse | FatalError | NonFatalError => Err(crate::Error::Sql(Self {
                inner,
                current_tuple: std::cell::RefCell::new(0),
            })),
            _ => Ok(Self {
                inner,
                current_tuple: std::cell::RefCell::new(0),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tuple<'a> {
    result: &'a libpq::Result,
    index: usize,
}

impl<'a> Tuple<'a> {
    pub fn from(result: &'a libpq::Result, index: usize) -> Self {
        Self {
            result,
            index,
        }
    }

    pub fn get<T>(&self, name: &str) -> T
    where
        T: FromSql,
    {
        self.try_get(name)
            .unwrap_or_else(|err| panic!("Unable to retreive '{}' field: {}", name, err))
    }

    pub fn try_get<T>(&self, name: &str) -> crate::Result<T>
    where
        T: FromSql,
    {
        let n = self.result.field_number(name).unwrap();
        let ty = self.result.field_type(n).unwrap_or(ty::TEXT);
        let value = self.result.value(self.index, n);

        FromSql::from_sql(&ty, value)
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub ty: Type,
    pub value: Option<Vec<u8>>,
}
