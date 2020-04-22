#[derive(Clone, Debug, romm::Entity)]
struct Serie {
    n: i32,
}

fn main() -> romm::Result<()> {
    let romm = romm::Romm::new().add_default("romm", "postgres://sanpi@localhost/romm")?;
    let connection = romm.default().unwrap();

    let series = connection.query::<Serie>(
        "select generate_series as n from generate_series(1, 10)",
        &[],
    )?;

    for serie in series {
        dbg!(serie);
    }

    Ok(())
}
