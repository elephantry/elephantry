pub fn database(connection: &romm::Connection) {
    let mut table = term_table::Table::new();
    table.style = term_table::TableStyle::rounded();

    table.add_row(term_table::row::Row::new(vec![
        term_table::table_cell::TableCell::new("name"),
        term_table::table_cell::TableCell::new("oid"),
        term_table::table_cell::TableCell::new("relations"),
        term_table::table_cell::TableCell::new("comment"),
    ]));

    for schema in romm::inspect::database(connection).iter() {
        table.add_row(term_table::row::Row::new(vec![
            term_table::table_cell::TableCell::new(&schema.name),
            term_table::table_cell::TableCell::new(&schema.oid),
            term_table::table_cell::TableCell::new(&schema.relations),
            term_table::table_cell::TableCell::new(&schema.comment),
        ]));
    }

    println!("{}", table.render());
}

pub fn schema(connection: &romm::Connection, schema: &str) {
    let relations = romm::inspect::schema(connection, schema);
    let mut table = term_table::Table::new();
    table.style = term_table::TableStyle::rounded();

    table.add_row(term_table::row::Row::new(vec![
        term_table::table_cell::TableCell::new("name"),
        term_table::table_cell::TableCell::new("type"),
        term_table::table_cell::TableCell::new("oid"),
        term_table::table_cell::TableCell::new("comment"),
    ]));

    for relation in relations.iter() {
        table.add_row(term_table::row::Row::new(vec![
            term_table::table_cell::TableCell::new(&relation.name),
            term_table::table_cell::TableCell::new(&relation.ty),
            term_table::table_cell::TableCell::new(&relation.oid),
            term_table::table_cell::TableCell::new(relation.comment.clone().unwrap_or_default()),
        ]));
    }

    println!(
        "\nFound {} relation(s) in schema '{}'.",
        relations.len(),
        schema
    );
    println!("{}", table.render());
}

pub fn relation(connection: &romm::Connection, schema: &str, relation: &str) {
    let mut table = term_table::Table::new();
    table.style = term_table::TableStyle::rounded();

    table.add_row(term_table::row::Row::new(vec![
        term_table::table_cell::TableCell::new("pk"),
        term_table::table_cell::TableCell::new("name"),
        term_table::table_cell::TableCell::new("type"),
        term_table::table_cell::TableCell::new("default"),
        term_table::table_cell::TableCell::new("notnull"),
        term_table::table_cell::TableCell::new("comment"),
    ]));

    for column in romm::inspect::relation(connection, schema, relation) {
        let primary = if column.is_primary {
            "*".to_string()
        } else {
            String::new()
        };

        let not_null = if column.is_notnull {
            "yes".to_string()
        } else {
            "no".to_string()
        };

        table.add_row(term_table::row::Row::new(vec![
            term_table::table_cell::TableCell::new(primary),
            term_table::table_cell::TableCell::new(&column.name),
            term_table::table_cell::TableCell::new(&column.ty),
            term_table::table_cell::TableCell::new(column.default.clone().unwrap_or_default()),
            term_table::table_cell::TableCell::new(not_null),
            term_table::table_cell::TableCell::new(column.comment.clone().unwrap_or_default()),
        ]));
    }

    println!("\nRelation {}.{}", schema, relation);
    println!("{}", table.render());
}
