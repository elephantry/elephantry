pub trait ToRust {
    fn to_rust(&self) -> String;
}

impl ToRust for elephantry::pq::Type {
    fn to_rust(&self) -> String {
        let rust = match self.name {
            "bigint" | "int8" => "i64",
            "bigserial" | "serial8" => "i64",
            #[cfg(feature = "bit")]
            "bit" => "u8",
            #[cfg(feature = "bit")]
            "bit varying" | "varbit" => "bit_vec::BitVec",
            "boolean" | "bool" => "bool",
            #[cfg(feature = "geo")]
            "box" => "elephantry::Box",
            "bytea" => "elephantry::Bytea",
            "character" | "char" => "i8",
            "character varying" | "varchar" => "String",
            #[cfg(feature = "network")]
            "cidr" => todo!(),
            #[cfg(feature = "geo")]
            "circle" => "elephantry::Circle",
            #[cfg(feature = "chrono")]
            "date" => "chrono::NaiveDate",
            "double precision" | "float8" => "f64",
            "hstore" => "elephantry::Hstore",
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
            #[cfg(feature = "net")]
            "macaddr8" => "macaddr::MacAddr8",
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
            #[cfg(feature = "chrono")]
            "timestamp" | "timestamp without time zone" => {
                "chrono::NaiveDateTime"
            },
            #[cfg(feature = "chrono")]
            "timestamp with time zone" | "timestamptz" => {
                "chrono::DateTime<chrono::FixedOffset>"
            },
            #[cfg(feature = "uuid")]
            "uuid" => "uuid::Uuid",

            _ => "String",
        };

        rust.to_string()
    }
}
