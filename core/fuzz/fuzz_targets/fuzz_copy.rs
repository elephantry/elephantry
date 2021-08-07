#![no_main]
use libfuzzer_sys::fuzz_target;

#[derive(Debug, arbitrary::Arbitrary, elephantry::Entity)]
#[elephantry(model="Model", structure="Structure")]
struct Entity {
    bigint: i64,
    bit: u8,
    boolean: bool,
    r#box: elephantry::Box,
    bytea: elephantry::Bytea,
    char: char,
    varchar: String,
    circle: elephantry::Circle,
    float8: f64,
    hstore: elephantry::Hstore,
    integer: i32,
    line: elephantry::Line,
    lseg: elephantry::Segment,
    money: f32,
    path: elephantry::Path,
    point: elephantry::Point,
    polygon: elephantry::Polygon,
    float4: f32,
    smallint: i16,
    text: String,
}

fuzz_target!(|entity: Entity| {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url).unwrap();
    elephantry.execute("create extension if not exists hstore").unwrap();
    elephantry.execute("create temporary table entity(
        bit bit,
        bigint bigint,
        box box,
        bytea bytea,
        boolean boolean,
        char char,
        varchar varchar,
        circle circle,
        float8 float8,
        hstore hstore,
        integer integer,
        line line,
        lseg lseg,
        money money,
        path path,
        point point,
        polygon polygon,
        float4 float4,
        smallint smallint,
        text text
);").unwrap();

    let _ = elephantry.copy::<Model, _>(vec![entity].into_iter());
});
