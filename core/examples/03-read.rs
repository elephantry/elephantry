mod employee {
    #[derive(Debug, elephantry::Entity)]
    #[elephantry(model = "Model", structure = "Structure", relation = "employee")]
    pub struct Entity {
        #[elephantry(pk)]
        pub employee_id: i32,
        pub first_name: String,
        pub last_name: String,
        pub birth_date: chrono::NaiveDate,
        pub is_manager: bool,
        pub day_salary: bigdecimal::BigDecimal,
        pub department_id: i32,
    }
}

fn main() -> elephantry::Result {
    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
    let elephantry = elephantry::Pool::new(&database_url)?;
    elephantry.execute(include_str!("structure.sql"))?;

    find_by_pk(&elephantry)?;
    find_all(&elephantry)?;
    find_where(&elephantry)?;
    count_where(&elephantry)?;
    exist_where(&elephantry)?;

    Ok(())
}

fn find_by_pk(elephantry: &elephantry::Pool) -> elephantry::Result {
    println!("# Find by primary key\n");

    let employee = elephantry.find_by_pk::<employee::Model>(&elephantry::pk!(employee_id => 1))?;
    println!("{employee:?}\n");

    Ok(())
}

fn find_all(elephantry: &elephantry::Pool) -> elephantry::Result {
    println!("# Find all\n");
    let employees = elephantry.find_all::<employee::Model>(Some("order by birth_date desc"))?;

    for employee in employees {
        println!("{} {}", employee.first_name, employee.last_name);
    }
    println!();

    Ok(())
}

fn find_where(elephantry: &elephantry::Pool) -> elephantry::Result {
    println!("# Find where\n");

    let managers = elephantry.find_where::<employee::Model>("is_manager = $1", &[&true], None)?;

    for manager in managers {
        println!("{} {}", manager.first_name, manager.last_name);
    }
    println!();

    Ok(())
}

fn count_where(elephantry: &elephantry::Pool) -> elephantry::Result {
    println!("# Count where\n");

    let n = elephantry.count_where::<employee::Model>("is_manager = $1", &[&true])?;
    println!("{n}\n");

    Ok(())
}

fn exist_where(elephantry: &elephantry::Pool) -> elephantry::Result {
    println!("# Exist where\n");

    let exist = elephantry.exist_where::<employee::Model>("day_salary < $1", &[&10_000])?;
    println!("{exist}\n");

    Ok(())
}
