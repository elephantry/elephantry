use case::CaseExt;
use std::io::Write;

pub fn schema(
    connection: &elephantry::Connection,
    schema: &str,
) -> crate::Result<()> {
    let relations = elephantry::inspect::schema(connection, schema);

    for r in relations {
        relation(connection, schema, &r.name)?;
    }

    Ok(())
}

pub fn relation(
    connection: &elephantry::Connection,
    schema: &str,
    relation: &str,
) -> crate::Result<()> {
    let dir = format!("model/{}", schema);
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{}/{}.rs", dir, relation);
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    write_entity(&mut file, connection, schema, relation)?;

    let columns = elephantry::inspect::relation(connection, schema, relation);
    let mut pk = Vec::new();
    let mut definition = Vec::new();

    for column in &columns {
        let name = column.name.to_snake();

        if column.is_primary {
            pk.push(format!("\"{}\"", name));
        }

        definition.push(format!("            \"{name}\",", name = name,));
    }

    write!(
        file,
        r"
pub struct Model<'a> {{
    connection: &'a elephantry::Connection,
}}

impl<'a> elephantry::Model<'a> for Model<'a> {{
    type Entity = Entity;
    type Structure = Structure;

    fn new(connection: &'a elephantry::Connection) -> Self {{
        Self {{ connection }}
    }}
}}

"
    )?;

    write!(
        file,
        r#"pub struct Structure;

impl elephantry::Structure for Structure {{
    fn relation() -> &'static str {{
        "{schema}.{relation}"
    }}

    fn primary_key() -> &'static [&'static str] {{
        &[{pk}]
    }}

    fn definition() -> &'static [&'static str] {{
        &[
{definition}
        ]
    }}
}}
"#,
        pk = pk.join(","),
        schema = schema,
        relation = relation,
        definition = definition.join("\n")
    )?;

    Ok(())
}

pub fn entity(
    connection: &elephantry::Connection,
    schema: &str,
    relation: &str,
) -> crate::Result<()> {
    let dir = format!("model/{}", schema);
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{}/{}.rs", dir, relation);
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    write_entity(&mut file, connection, schema, relation)?;

    Ok(())
}

fn write_entity<W>(
    file: &mut std::io::BufWriter<W>,
    connection: &elephantry::Connection,
    schema: &str,
    relation: &str,
) -> crate::Result<()>
where
    W: std::io::Write,
{
    let columns = elephantry::inspect::relation(connection, schema, relation);

    if columns.is_empty() {
        return Err(crate::Error::MissingRelation(format!("{}.{}", schema, relation)));
    }

    let mut fields = Vec::new();

    for column in &columns {
        let name = column.name.to_snake();
        let ty = ty_to_rust(&column);

        fields.push(format!("    pub {}: {},", name, ty));
    }

    write!(
        file,
        r"#[derive(elephantry::Entity)]
pub struct Entity {{
{fields}
}}
",
        fields = fields.join("\n")
    )?;

    Ok(())
}

fn ty_to_rust(column: &elephantry::inspect::Column) -> String {
    use crate::pq::ToRust;
    use std::convert::TryFrom;

    let ty = elephantry::pq::Type::try_from(column.oid).unwrap();

    if column.is_notnull {
        ty.to_rust()
    }
    else {
        format!("Option<{}>", ty.to_rust())
    }
}
