pub use libpq::types;

pub type Type = libpq::Type;

use std::collections::HashMap;

impl crate::FromSql for Type {
    fn from_binary(ty: &Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        String::from_binary(ty, raw)?
            .parse()
            .map_err(|_| Self::error(ty, raw))
    }

    fn from_text(ty: &Type, raw: Option<&str>) -> crate::Result<Self> {
        crate::from_sql::not_null(raw)?
            .parse()
            .map_err(|_| Self::error(ty, raw))
    }
}

impl crate::ToSql for Type {
    fn ty(&self) -> Type {
        types::TEXT
    }

    fn to_text(&self) -> crate::Result<Option<String>> {
        self.name.to_text()
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        Ok(Some(self.oid.to_be_bytes().to_vec()))
    }
}

static TYPES: std::sync::LazyLock<HashMap<&'static str, &'static str>> =
    std::sync::LazyLock::new(|| {
        use std::any::type_name as t;

        let mut types = HashMap::new();
        types.insert(types::BIT.name, t::<u8>());
        types.insert(types::BOOL.name, t::<bool>());
        types.insert(types::CHAR.name, t::<char>());
        types.insert(types::FLOAT4.name, t::<f32>());
        types.insert(types::FLOAT8.name, t::<f64>());
        types.insert(types::INT2.name, t::<i16>());
        types.insert(types::INT4.name, t::<i32>());
        types.insert(types::INT8.name, t::<i64>());
        types.insert(types::TEXT.name, "String");
        types.insert(types::VARCHAR.name, "String");

        types.insert(types::VARBIT.name, "elephantry::Bits");
        types.insert(
            types::BYTEA.name,
            #[cfg(feature = "bit")]
            t::<crate::Bytea>(),
            #[cfg(not(feature = "bit"))]
            "elephantry::Bytea",
        );

        if cfg!(feature = "chrono") {
            types.insert(types::DATE.name, "chrono::NaiveDate");
            types.insert(types::TIMESTAMP.name, "chrono::NaiveDateTime");
            types.insert(
                types::TIMESTAMPTZ.name,
                "chrono::DateTime<chrono::offset::Local>",
            );
        } else if cfg!(feature = "jiff") {
            types.insert(types::DATE.name, "jiff::civil::Date");
            types.insert(types::TIMESTAMP.name, "jiff::civil::DateTime");
            types.insert(types::TIMESTAMPTZ.name, "jiff::Zoned");
        } else {
            types.insert(types::DATE.name, "elephantry::Date");
            types.insert(types::TIMESTAMP.name, "elephantry::Timestamp");
            types.insert(types::TIMESTAMPTZ.name, "elephantry::TimestampTz");
        }

        types.insert(
            types::BOX.name,
            #[cfg(feature = "geo")]
            t::<crate::Box>(),
            #[cfg(not(feature = "geo"))]
            "elephantry::Box",
        );
        types.insert(
            types::CIRCLE.name,
            #[cfg(feature = "geo")]
            t::<crate::Circle>(),
            #[cfg(not(feature = "geo"))]
            "elephantry::Circle",
        );
        types.insert(
            types::LINE.name,
            #[cfg(feature = "geo")]
            t::<crate::Line>(),
            #[cfg(not(feature = "geo"))]
            "elephantry::Line",
        );
        types.insert(
            types::LSEG.name,
            #[cfg(feature = "geo")]
            t::<crate::Segment>(),
            #[cfg(not(feature = "geo"))]
            "elephantry::Segment",
        );
        types.insert(
            types::PATH.name,
            #[cfg(feature = "geo")]
            t::<crate::Path>(),
            #[cfg(not(feature = "geo"))]
            "elephantry::Path",
        );
        types.insert(
            types::POINT.name,
            #[cfg(feature = "geo")]
            t::<crate::Point>(),
            #[cfg(not(feature = "geo"))]
            "elephantry::Point",
        );
        types.insert(
            types::POLYGON.name,
            #[cfg(feature = "geo")]
            t::<crate::Polygon>(),
            #[cfg(not(feature = "geo"))]
            "elephantry::Polygon",
        );

        types.insert(types::CIDR.name, "elephantry::Cidr");
        types.insert(types::INET.name, "std::net::IpAddr");
        types.insert(types::MACADDR.name, "elephantry::MacAddr");
        types.insert(types::MACADDR8.name, "elephantry::MacAddr8");

        types.insert(types::JSON.name, "elephantry::Json");
        types.insert(
            types::JSONB.name,
            #[cfg(feature = "json")]
            t::<crate::Jsonb>(),
            #[cfg(not(feature = "json"))]
            "elephantry::Jsonb",
        );

        types.insert(
            "lquery",
            #[cfg(feature = "ltree")]
            t::<crate::Lquery>(),
            #[cfg(not(feature = "ltree"))]
            "elephantry::Lquery",
        );
        types.insert(
            "ltree",
            #[cfg(feature = "ltree")]
            t::<crate::Ltree>(),
            #[cfg(not(feature = "ltree"))]
            "elephantry::Ltree",
        );
        types.insert(
            "ltxtquery",
            #[cfg(feature = "ltree")]
            t::<crate::Ltxtquery>(),
            #[cfg(not(feature = "ltree"))]
            "elephantry::Ltxtquery",
        );

        types.insert(types::MONEY.name, "elephantry::Money");

        types.insert(types::NUMERIC.name, "elephantry::Numeric");

        if cfg!(feature = "time") {
            types.insert(types::TIME.name, "time::Time");
            types.insert(types::TIMETZ.name, "(time::Time, time::UtcOffset)");
        } else if cfg!(feature = "chrono") {
            types.insert(types::TIME.name, "chrono::NaiveTime");
            types.insert(
                types::TIMETZ.name,
                "(chrono::NaiveTime, chrono::FixedOffset)",
            );
        } else if cfg!(feature = "jiff") {
            types.insert(types::TIME.name, "jiff::civil::Time");
            types.insert(types::TIME.name, "(jiff::civil::Time, jiff::tz::TimeZone)");
        } else {
            types.insert(types::TIME.name, "elephantry::Time");
            types.insert(types::TIMETZ.name, "elephantry::TimeTz");
        }

        types.insert(types::UUID.name, "elephantry::Uuid");

        types.insert(types::XML.name, "elephantry::Xml");

        types
    });

#[doc(hidden)]
#[must_use]
pub fn sql_to_rust(ty: &crate::pq::Type) -> String {
    let rty = TYPES.get(ty.name).unwrap_or(&"String");

    if matches!(ty.kind, crate::pq::types::Kind::Array(_)) {
        format!("Vec<{rty}>")
    } else {
        (*rty).to_string()
    }
}

pub(crate) trait ToArray {
    fn to_array(&self) -> Self;
    fn to_range(&self) -> Self;
    #[cfg(feature = "multirange")]
    fn to_multi_range(&self) -> Self;
    fn elementype(&self) -> Self;
    fn is_text(&self) -> bool;
}

impl ToArray for Type {
    fn to_range(&self) -> Self {
        match *self {
            types::ANY => types::ANY_RANGE,
            types::ANYCOMPATIBLE => types::ANYCOMPATIBLE_RANGE,
            types::INT4 => types::INT4_RANGE,
            types::INT8 => types::INT8_RANGE,
            types::NUMERIC => types::NUM_RANGE,
            types::TIMESTAMP => types::TS_RANGE,
            types::TIMESTAMPTZ => types::TSTZ_RANGE,
            types::DATE => types::DATE_RANGE,
            _ => self.clone(),
        }
    }

    #[cfg(feature = "multirange")]
    fn to_multi_range(&self) -> Self {
        match *self {
            types::ANY_RANGE => types::ANYMULTI_RANGE,
            types::ANYCOMPATIBLE_RANGE => types::ANYCOMPATIBLEMULTI_RANGE,
            types::INT4_RANGE => types::INT4MULTI_RANGE,
            types::INT8_RANGE => types::INT8MULTI_RANGE,
            types::NUM_RANGE => types::NUMMULTI_RANGE,
            types::TS_RANGE => types::TSMULTI_RANGE,
            types::TSTZ_RANGE => types::TSTZMULTI_RANGE,
            types::DATE_RANGE => types::DATEMULTI_RANGE,
            _ => self.clone(),
        }
    }

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

    fn elementype(&self) -> Self {
        match self.kind {
            crate::pq::types::Kind::Array(oid) => {
                crate::pq::Type::try_from(oid).unwrap_or(crate::pq::types::UNKNOWN)
            }
            _ => crate::pq::types::UNKNOWN,
        }
    }

    fn is_text(&self) -> bool {
        self == &types::TEXT || self == &types::VARCHAR
    }
}
