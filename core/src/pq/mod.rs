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
            #[cfg(feature = "bit")]
            "bit" => "u8",
            #[cfg(feature = "bit")]
            "bit varying" | "varbit" => "elephantry::BitVec",
            "boolean" | "bool" => "bool",
            #[cfg(feature = "geo")]
            "box" => "elephantry::Box",
            "bytea" => "Vec<u8>",
            "character" | "char" => "i8",
            "character varying" | "varchar" => "String",
            #[cfg(feature = "network")]
            "cidr" => todo!(),
            #[cfg(feature = "geo")]
            "circle" => "elephantry::Circle",
            #[cfg(feature = "chrono")]
            "date" => "chrono::NaiveDate",
            "double precision" | "float8" => "f64",
            #[cfg(feature = "net")]
            "inet" => "std::net::IpAddr",
            "integer" | "int" | "int4" => "i32",
            #[cfg(feature = "json")]
            "json" | "jsonb" => "serde::value::Value",
            #[cfg(feature = "geo")]
            "line" => "elephantry::Line",
            #[cfg(feature = "geo")]
            "lseg" => "elephantry::Segment",
            #[cfg(feature = "net")]
            "macaddr" => "macaddr::MacAddr6",
            #[cfg(feature = "network")]
            #[cfg(feature = "network")]
            "macaddr8" => "eui48::MacAddress",
            "money" => "f32",
            #[cfg(feature = "numeric")]
            "numeric" | "decimal" => "bigdecimal::BigDecimal",
            #[cfg(feature = "geo")]
            "path" => "elephantry::Path",
            "pg_lsn" => "String",
            #[cfg(feature = "geo")]
            "point" => "elephantry::Point",
            #[cfg(feature = "geo")]
            "polygon" => "elephantry::Polygon",
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
            "timestamp" | "timestamp without time zone" => {
                "std::time::SystemTime"
            },
            #[cfg(feature = "chrono")]
            "timestamp" | "timestamp without time zone" => {
                "chrono::NaiveDateTime"
            },
            #[cfg(not(feature = "chrono"))]
            "timestamp with time zone" | "timestamptz" => {
                "std::time::SystemTime"
            },
            #[cfg(feature = "chrono")]
            "timestamp with time zone" | "timestamptz" => {
                "chrono::DateTime<chrono::FixedOffset>"
            },
            #[cfg(feature = "uuid")]
            "uuid" => "elephantry::Uuid",
            "xml" => "String",

            "hstore" => "std::collection::HashMap<String, Option<String>>",

            _ => "String",
        };

        rust.to_string()
    }
}

#[derive(Debug)]
pub struct Result {
    inner: libpq::Result,
    current_tuple: std::cell::RefCell<usize>,
}

impl Result {
    pub fn get(&self, n: usize) -> crate::Tuple<'_> {
        self.try_get(n).unwrap()
    }

    pub fn try_get(&self, n: usize) -> Option<crate::Tuple<'_>> {
        if n + 1 > self.len() {
            return None;
        }

        let tuple = crate::Tuple::from(&self.inner, n);

        Some(tuple)
    }

    pub fn len(&self) -> usize {
        self.inner.ntuples()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn state(&self) -> State {
        State::from_code(
            &self
                .inner
                .error_field(libpq::result::ErrorField::Sqlstate)
                .unwrap(),
        )
    }
}

impl<'a> std::iter::Iterator for &'a Result {
    type Item = crate::Tuple<'a>;

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
            BadResponse | FatalError | NonFatalError => {
                Err(crate::Error::Sql(Self {
                    inner,
                    current_tuple: std::cell::RefCell::new(0),
                }))
            },
            _ => {
                Ok(Self {
                    inner,
                    current_tuple: std::cell::RefCell::new(0),
                })
            },
        }
    }
}
