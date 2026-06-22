/**
 * Impl this trait if you would like support new function as argument for testing functions.
 */
pub trait Fn {
    /**
     * Creates new [`Pool`] from environment variables and load fixtures.
     *
     *  [`Pool`]: struct.Pool.html
     */
    fn init(fixtures: &[&str]) -> crate::Result<crate::Pool> {
        envir::try_init().ok();

        let config = crate::Config::from_env()?;
        let pool = crate::Pool::from_config(&config)?;

        for fixture in fixtures {
            Self::load_fixture(&pool, fixture)?;
        }

        Ok(pool)
    }

    /**
     * Execute fixtures.
     */
    fn load_fixture(pool: &crate::Pool, fixture: &str) -> crate::Result {
        let filename = find_fixture(fixture)
            .ok_or_else(|| crate::Error::FixtureNotFound(fixture.to_string()))?;
        let sql = std::fs::read_to_string(&filename)?;

        pool.execute(&sql)?;

        Ok(())
    }

    /**
     * Call testing function.
     *
     * You should call [`init`] before to creates connection and execute fixtures.
     *
     * [`init`]: #method.init
     */
    fn run(self, fixtures: &[&str]) -> crate::Result;
}

fn find_fixture(fixture: &str) -> Option<String> {
    [format!("fixtures/{fixture}.sql"), fixture.to_string()]
        .into_iter()
        .find(|x| std::fs::exists(x).unwrap_or_default())
}

impl Fn for fn(crate::Pool) -> crate::Result {
    fn run(self, fixtures: &[&str]) -> crate::Result {
        let pool = Self::init(fixtures)?;

        self(pool)
    }
}

impl Fn for fn(crate::Pool) {
    fn run(self, fixtures: &[&str]) -> crate::Result {
        let pool = Self::init(fixtures)?;

        self(pool);

        Ok(())
    }
}

impl Fn for fn(crate::Connection) -> crate::Result {
    fn run(self, fixtures: &[&str]) -> crate::Result {
        let pool = Self::init(fixtures)?;

        self(pool.get_default().unwrap().clone())
    }
}

impl Fn for fn(crate::Connection) {
    fn run(self, fixtures: &[&str]) -> crate::Result {
        let pool = Self::init(fixtures)?;

        self(pool.get_default().unwrap().clone());

        Ok(())
    }
}

impl Fn for fn() -> crate::Result {
    fn run(self, fixtures: &[&str]) -> crate::Result {
        Self::init(fixtures)?;

        self()
    }
}

impl Fn for fn() {
    fn run(self, fixtures: &[&str]) -> crate::Result {
        Self::init(fixtures)?;

        self();

        Ok(())
    }
}
