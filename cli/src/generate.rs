use case::CaseExt;
use std::io::Write;

pub fn schema(
    connection: &elephantry::Connection,
    prefix_dir: &str,
    schema: &str,
) -> crate::Result {
    let relations = elephantry::inspect::schema(connection, schema)?;

    add_mod(&format!("{prefix_dir}/model"), schema)?;

    for r in relations {
        relation(connection, prefix_dir, schema, &r.name)?;
    }

    Ok(())
}

pub fn relation(
    connection: &elephantry::Connection,
    prefix_dir: &str,
    schema: &str,
    relation: &str,
) -> crate::Result {
    let dir = format!("{prefix_dir}/model/{schema}");
    add_mod(&dir, relation)?;

    let filename = format!("{dir}/{relation}.rs");
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    let columns = elephantry::inspect::relation(connection, schema, relation)?;

    let mut fields = Vec::new();

    for column in &columns {
        let name = name_to_rust(column);
        let ty = ty_to_rust(column);

        if column.is_primary {
            fields.push("    #[elephantry(pk)]".to_string());
        }
        fields.push(format!("    pub {name}: {ty},"));
    }

    write!(
        file,
        r#"#[derive(elephantry::Entity)]
#[elephantry(model = "Model", structure = "Structure", relation = "{relation}")]
pub struct Entity {{
{fields}
}}
"#,
        relation = relation,
        fields = fields.join("\n")
    )?;

    Ok(())
}

pub fn entity(
    connection: &elephantry::Connection,
    prefix_dir: &str,
    schema: &str,
    relation: &str,
) -> crate::Result {
    let dir = format!("{prefix_dir}/model/{schema}");
    add_mod(&dir, relation)?;

    let filename = format!("{dir}/{relation}.rs");
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    write_entity(&mut file, connection, schema, relation)?;

    Ok(())
}

fn write_entity<W>(
    file: &mut std::io::BufWriter<W>,
    connection: &elephantry::Connection,
    schema: &str,
    relation: &str,
) -> crate::Result
where
    W: std::io::Write,
{
    let columns = elephantry::inspect::relation(connection, schema, relation)?;

    let mut fields = Vec::new();

    for column in &columns {
        let name = name_to_rust(column);
        let ty = ty_to_rust(column);

        fields.push(format!("    pub {name}: {ty},"));
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

pub fn enums(connection: &elephantry::Connection, prefix_dir: &str, schema: &str) -> crate::Result {
    let dir = format!("{prefix_dir}/enums");
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{dir}/{schema}.rs");
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    for enumeration in &elephantry::inspect::enums(connection, schema)? {
        write_enum(&mut file, enumeration)?;
    }

    Ok(())
}

fn write_enum<W>(
    file: &mut std::io::BufWriter<W>,
    enumeration: &elephantry::inspect::Enum,
) -> crate::Result
where
    W: std::io::Write,
{
    let elements = enumeration
        .elements
        .iter()
        .map(|x| format!("    {x},"))
        .collect::<Vec<_>>();

    write!(
        file,
        r"#[derive(elephantry::Enum)]
pub struct {name} {{
{elements}
}}
",
        name = enumeration.name,
        elements = elements.join("\n"),
    )?;

    Ok(())
}

pub fn composites(
    connection: &elephantry::Connection,
    prefix_dir: &str,
    schema: &str,
) -> crate::Result {
    let dir = format!("{prefix_dir}/composites");
    std::fs::create_dir_all(&dir)?;

    let filename = format!("{dir}/{schema}.rs");
    let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);

    for composite in &elephantry::inspect::composites(connection, schema)? {
        write_composite(&mut file, composite)?;
    }

    Ok(())
}

fn write_composite<W>(
    file: &mut std::io::BufWriter<W>,
    composite: &elephantry::inspect::Composite,
) -> crate::Result
where
    W: std::io::Write,
{
    let fields = composite
        .fields
        .iter()
        .map(|f| {
            format!(
                "    {}: {},",
                f.name,
                elephantry::pq::sql_to_rust(&f.oid.try_into().unwrap())
            )
        })
        .collect::<Vec<_>>();

    write!(
        file,
        r"#[derive(elephantry::Composite)]
pub struct {name} {{
{fields}
}}
",
        name = composite.name.to_camel(),
        fields = fields.join("\n")
    )?;

    Ok(())
}

fn ty_to_rust(column: &elephantry::inspect::Column) -> String {
    let mut rty = match elephantry::pq::Type::try_from(column.oid) {
        Ok(elephantry::pq::types::BIT) => match column.len {
            Some(1) => "u8".to_string(),
            Some(len) => format!("[u8; {len}]"),
            _ => unreachable!(),
        },
        Ok(ty) => elephantry::pq::sql_to_rust(&ty),
        Err(_) => {
            if column.ty() == "public.hstore" {
                "elephantry::Hstore".to_string()
            } else if column.ty() == "public.ltree" {
                "elephantry::Ltree".to_string()
            } else if column.ty() == "public.lquery" {
                "elephantry::Lquery".to_string()
            } else if column.ty() == "public.ltxtquery" {
                "elephantry::Ltxtquery".to_string()
            } else {
                column.ty().replace('.', "::").to_string()
            }
        }
    };

    if !column.is_notnull {
        rty = format!("Option<{rty}>");
    }

    rty
}

fn name_to_rust(column: &elephantry::inspect::Column) -> String {
    let mut name = column.name.to_snake();

    if is_keyword(&name) {
        name = format!("r#{name}");
    }

    name
}

fn is_keyword(name: &str) -> bool {
    static KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
    ];

    KEYWORDS.contains(&name)
}

fn add_mod(dir: &str, name: &str) -> crate::Result {
    std::fs::create_dir_all(dir)?;

    let mod_filename = format!("{dir}/mod.rs");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(mod_filename)?;
    file.write_all(format!("mod {name};\n").as_bytes())?;

    Ok(())
}
