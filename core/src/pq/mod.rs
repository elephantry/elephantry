pub use libpq::state;

pub use libpq::connection::Notify;
pub use libpq::types;
pub type Format = libpq::Format;
pub type Oid = libpq::Oid;
pub type State = libpq::State;

pub type Type = libpq::Type;

pub(crate) trait ToArray {
    fn to_array(&self) -> Self;
}

impl ToArray for Type {
    fn to_array(&self) -> Self {
        match *self {
            types::ACLITEM => types::ACLITEM_ARRAY,
            types::BIT => types::BIT_ARRAY,
            types::BOOL => types::BOOL_ARRAY,
            types::BOX => types::BOX_ARRAY,
            types::BPCHAR => types::BPCHAR_ARRAY,
            types::BYTEA => types::BYTEA_ARRAY,
            types::CHAR => types::CHAR_ARRAY,
            types::CID => types::CID_ARRAY,
            types::CIDR => types::CIDR_ARRAY,
            types::CIRCLE => types::CIRCLE_ARRAY,
            types::CSTRING => types::CSTRING_ARRAY,
            types::DATE => types::DATE_ARRAY,
            types::DATE_RANGE => types::DATE_RANGE_ARRAY,
            types::FLOAT4 => types::FLOAT4_ARRAY,
            types::FLOAT8 => types::FLOAT8_ARRAY,
            types::GTS_VECTOR => types::GTS_VECTOR_ARRAY,
            types::INET => types::INET_ARRAY,
            types::INT2 => types::INT2_ARRAY,
            types::INT2_VECTOR => types::INT2_VECTOR_ARRAY,
            types::INT4 => types::INT4_ARRAY,
            types::INT4_RANGE => types::INT4_RANGE_ARRAY,
            types::INT8 => types::INT8_ARRAY,
            types::INT8_RANGE => types::INT8_RANGE_ARRAY,
            types::INTERVAL => types::INTERVAL_ARRAY,
            types::JSON => types::JSON_ARRAY,
            types::JSONB => types::JSONB_ARRAY,
            types::JSONPATH => types::JSONPATH_ARRAY,
            types::LINE => types::LINE_ARRAY,
            types::LSEG => types::LSEG_ARRAY,
            types::MACADDR => types::MACADDR_ARRAY,
            types::MACADDR8 => types::MACADDR8_ARRAY,
            types::MONEY => types::MONEY_ARRAY,
            types::NAME => types::NAME_ARRAY,
            types::NUMERIC => types::NUMERIC_ARRAY,
            types::NUM_RANGE => types::NUM_RANGE_ARRAY,
            types::OID => types::OID_ARRAY,
            types::OID_VECTOR => types::OID_VECTOR_ARRAY,
            types::PATH => types::PATH_ARRAY,
            types::PG_LSN => types::PG_LSN_ARRAY,
            types::POINT => types::POINT_ARRAY,
            types::POLYGON => types::POLYGON_ARRAY,
            types::RECORD => types::RECORD_ARRAY,
            types::REFCURSOR => types::REFCURSOR_ARRAY,
            types::REGCLASS => types::REGCLASS_ARRAY,
            types::REGCONFIG => types::REGCONFIG_ARRAY,
            types::REGDICTIONARY => types::REGDICTIONARY_ARRAY,
            types::REGNAMESPACE => types::REGNAMESPACE_ARRAY,
            types::REGOPER => types::REGOPER_ARRAY,
            types::REGOPERATOR => types::REGOPERATOR_ARRAY,
            types::REGPROC => types::REGPROC_ARRAY,
            types::REGPROCEDURE => types::REGPROCEDURE_ARRAY,
            types::REGROLE => types::REGROLE_ARRAY,
            types::REGTYPE => types::REGTYPE_ARRAY,
            types::TEXT => types::TEXT_ARRAY,
            types::TID => types::TID_ARRAY,
            types::TIMESTAMP => types::TIMESTAMP_ARRAY,
            types::TIMESTAMPTZ => types::TIMESTAMPTZ_ARRAY,
            types::TIME => types::TIME_ARRAY,
            types::TIMETZ => types::TIMETZ_ARRAY,
            types::TSQUERY => types::TSQUERY_ARRAY,
            types::TSTZ_RANGE => types::TSTZ_RANGE_ARRAY,
            types::TS_RANGE => types::TS_RANGE_ARRAY,
            types::TS_VECTOR => types::TS_VECTOR_ARRAY,
            types::TXID_SNAPSHOT => types::TXID_SNAPSHOT_ARRAY,
            types::UUID => types::UUID_ARRAY,
            types::VARBIT => types::VARBIT_ARRAY,
            types::VARCHAR => types::VARCHAR_ARRAY,
            types::XID => types::XID_ARRAY,
            types::XML => types::XML_ARRAY,
            _ => self.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Result {
    pub(crate) inner: libpq::Result,
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

    pub fn state(&self) -> Option<crate::pq::State> {
        self.inner
            .error_field(libpq::result::ErrorField::Sqlstate)
            .map(crate::pq::State::from_code)
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
