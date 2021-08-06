#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = String::from_utf8(data.to_vec()) {
        let database_url =
            std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost".to_string());
        let elephantry = elephantry::Pool::new(&database_url).unwrap();
        let _ = elephantry.query::<String>("select $1", &[&s]);
    }
});
