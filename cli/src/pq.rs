pub trait ToRust {
    fn to_rust(&self) -> String;
}

impl ToRust for elephantry::pq::Type {
    fn to_rust(&self) -> String {
        sql_to_rust(self.name)
    }
}

pub(crate) fn sql_to_rust(sql: &str) -> String {
    let rust = match sql {
        "bigint" | "int8" => "i64",
        "bigserial" | "serial8" => "i64",
        "bit" => "u8",
        "bit varying" | "varbit" => "bit_vec::BitVec",
        "boolean" | "bool" => "bool",
        "box" => "elephantry::Box",
        "bytea" => "elephantry::Bytea",
        "character" | "char" => "i8",
        "character varying" | "varchar" => "String",
        "cidr" => "ipnetwork::IpNetwork",
        "circle" => "elephantry::Circle",
        "date" => "chrono::NaiveDate",
        "double precision" | "float8" => "f64",
        "hstore" => "elephantry::Hstore",
        "inet" => "std::net::IpAddr",
        "integer" | "int" | "int4" => "i32",
        "json" | "jsonb" => "serde::value::Value",
        "line" => "elephantry::Line",
        "lseg" => "elephantry::Segment",
        "macaddr" => "macaddr::MacAddr6",
        "macaddr8" => "macaddr::MacAddr8",
        "money" => "f32",
        "numeric" | "decimal" => "bigdecimal::BigDecimal",
        "path" => "elephantry::Path",
        "point" => "elephantry::Point",
        "polygon" => "elephantry::Polygon",
        "real" | "float4" => "f32",
        "smallint" | "int2" => "i16",
        "smallserial" | "serial2" => "i16",
        "serial" | "serial4" => "i32",
        "text" => "String",
        "time" | "time without time zone" => "chrono::NaiveTime",
        "time with time zone" | "timetz" => "elephantry::TimeTz",
        "timestamp" | "timestamp without time zone" => "chrono::NaiveDateTime",
        "timestamp with time zone" | "timestamptz" => {
            "chrono::DateTime<chrono::FixedOffset>"
        },
        "uuid" => "uuid::Uuid",
        "xml" => "xmltree::Element",

        _ => "String",
    };

    rust.to_string()
}
