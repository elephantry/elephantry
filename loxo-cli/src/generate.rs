use case::CaseExt;
use std::io::Write;

pub fn schema(connection: &loxo::Connection, schema: &str) -> std::io::Result<()> {
    let relations = loxo::inspect::schema(connection, schema);

    for r in relations {
        relation(connection, schema, &r.name)?;
    }

    Ok(())
}

pub fn relation(connection: &loxo::Connection, schema: &str, relation: &str) -> std::io::Result<()> {
    let dir = format!("model/{}", schema);
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{}/{}.rs", dir, relation);
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    write_entity(&mut file, connection, schema, relation)?;

    let columns = loxo::inspect::relation(connection, schema, relation);
    let mut pk = Vec::new();
    let mut definition = Vec::new();

    for column in &columns {
        let name = column.name.to_snake();

        if column.is_primary {
            pk.push(format!("\"{}\"", name));
        }

        definition.push(format!(
            "        definition.insert(\"{name}\", \"%:{name}:%\");",
            name = name,
        ));
    }

    write!(
        file,
        r"
struct Model<'a> {{
    connection: &'a loxo::Connection,
}}


impl<'a> loxo::Model<'a> for Model<'a> {{
    type Entity = Entity;
    type Structure = Structure;

    fn new(connection: &'a loxo::Connection) -> Self {{
        Self {{ connection }}
    }}
}}

"
    )?;

    write!(
        file,
        r#"struct Structure;

impl loxo::row::Structure for Structure
{{
    fn relation() -> &'static str
    {{
        "{schema}.{relation}"
    }}

    fn primary_key() -> &'static [&'static str]
    {{
        &[{pk}]
    }}

    fn definition() -> std::collections::HashMap<&'static str, &'static str>
    {{
        let mut definition = std::collections::HashMap::new();

{definition}

        definition
    }}
}}
"#,
        pk = pk.join(","),
        schema = schema,
        relation = relation,
        definition = definition.join("\n")
    )
}

pub fn entity(connection: &loxo::Connection, schema: &str, relation: &str) -> std::io::Result<()> {
    let dir = format!("model/{}", schema);
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{}/{}.rs", dir, relation);
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    write_entity(&mut file, connection, schema, relation)
}

fn write_entity<W>(file: &mut std::io::BufWriter<W>, connection: &loxo::Connection, schema: &str, relation: &str) -> std::io::Result<()> where W: std::io::Write {
    let columns = loxo::inspect::relation(connection, schema, relation);
    let mut fields = Vec::new();

    for column in &columns {
        let name = column.name.to_snake();
        let ty = ty_to_rust(&column);

        fields.push(format!("    {}: {},", name, ty));
    }

    write!(
        file,
        r"#[derive(Clone, Debug, loxo::Entity)]
struct Entity {{
{fields}
}}
",
        fields = fields.join("\n")
    )
}

fn ty_to_rust(column: &loxo::inspect::Column) -> String {
    use loxo::pq::ToRust;
    use std::convert::TryFrom;

    let ty = loxo::pq::Type::try_from(column.oid).unwrap();

    if column.is_notnull {
        ty.to_rust()
    } else {
        format!("Option<{}>", ty.to_rust())
    }
}
