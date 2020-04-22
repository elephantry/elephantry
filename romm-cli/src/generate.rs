use case::CaseExt;
use std::io::Write;

pub fn schema(connection: &romm::Connection, schema: &str) -> std::io::Result<()> {
    let relations = romm::inspect::schema(connection, schema);

    for r in relations {
        relation(connection, schema, &r.name)?;
    }

    Ok(())
}

pub fn relation(connection: &romm::Connection, schema: &str, relation: &str) -> std::io::Result<()> {
    let dir = format!("model/{}", schema);
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{}/{}.rs", dir, relation);
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    write_entity(&mut file, connection, schema, relation)?;

    let columns = romm::inspect::relation(connection, schema, relation);
    let mut pk = Vec::new();
    let mut definition = Vec::new();

    for column in &columns {
        let name = column.name.to_snake();

        let ty = if column.ty.starts_with('_') {
            format!("{}_ARRAY", column.ty.trim_start_matches('_'))
        } else {
            column.ty.clone()
        };

        if column.is_primary {
            pk.push(format!("\"{}\"", name));
        }

        definition.push(format!(
            "        definition.insert(\"{name}\", romm::Row {{
            content: \"%:{name}:%\",
            ty: romm::pq::ty::{ty},
        }});",
            name = name,
            ty = ty.to_uppercase(),
        ));
    }

    write!(
        file,
        r"
struct Model<'a> {{
    connection: &'a romm::Connection,
}}


impl<'a> romm::Model<'a> for Model<'a> {{
    type Entity = Entity;
    type Structure = Structure;

    fn new(connection: &'a romm::Connection) -> Self {{
        Self {{ connection }}
    }}
}}

"
    )?;

    write!(
        file,
        r#"struct Structure;

impl romm::row::Structure for Structure
{{
    fn relation() -> &'static str
    {{
        "{schema}.{relation}"
    }}

    fn primary_key() -> &'static [&'static str]
    {{
        &[{pk}]
    }}

    fn definition() -> std::collections::HashMap<&'static str, romm::Row>
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

pub fn entity(connection: &romm::Connection, schema: &str, relation: &str) -> std::io::Result<()> {
    let dir = format!("model/{}", schema);
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{}/{}.rs", dir, relation);
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    write_entity(&mut file, connection, schema, relation)
}

fn write_entity<W>(file: &mut std::io::BufWriter<W>, connection: &romm::Connection, schema: &str, relation: &str) -> std::io::Result<()> where W: std::io::Write {
    let columns = romm::inspect::relation(connection, schema, relation);
    let mut fields = Vec::new();

    for column in &columns {
        let name = column.name.to_snake();
        let ty = ty_to_rust(&column);

        fields.push(format!("    {}: {},", name, ty));
    }

    write!(
        file,
        r"#[derive(Clone, Debug, romm::Entity)]
struct Entity {{
{fields}
}}
",
        fields = fields.join("\n")
    )
}

fn ty_to_rust(column: &romm::inspect::Column) -> String {
    use romm::pq::ToRust;
    use std::convert::TryFrom;

    let ty = romm::pq::Type::try_from(column.oid).unwrap();

    if column.is_notnull {
        ty.to_rust()
    } else {
        format!("Option<{}>", ty.to_rust())
    }
}
