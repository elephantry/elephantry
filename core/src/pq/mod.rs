pub use libpq::state;

pub use libpq::connection::Notify;
pub use libpq::types;
pub type Format = libpq::Format;
pub type Oid = libpq::Oid;
pub type Type = libpq::Type;
pub type State = libpq::State;

#[derive(Debug)]
pub struct Result {
    inner: libpq::Result,
    current_tuple: std::cell::RefCell<usize>,
}

impl Result {
    pub fn get(&self, n: usize) -> crate::Tuple<'_> {
        self.try_get(n).unwrap()
    }

    pub fn try_get(&self, n: usize) -> Option<crate::Tuple<'_>> {
        if n + 1 > self.len() {
            return None;
        }

        let tuple = crate::Tuple::from(&self.inner, n);

        Some(tuple)
    }

    pub fn len(&self) -> usize {
        self.inner.ntuples()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn state(&self) -> State {
        State::from_code(
            &self
                .inner
                .error_field(libpq::result::ErrorField::Sqlstate)
                .unwrap(),
        )
    }
}

impl<'a> std::iter::Iterator for &'a Result {
    type Item = crate::Tuple<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let tuple = self.try_get(*self.current_tuple.borrow());
        *self.current_tuple.borrow_mut() += 1;

        tuple
    }
}

impl std::ops::Deref for Result {
    type Target = libpq::Result;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::convert::TryFrom<libpq::Result> for Result {
    type Error = crate::Error;

    fn try_from(inner: libpq::Result) -> crate::Result<Self> {
        use libpq::Status::*;

        match inner.status() {
            BadResponse | FatalError | NonFatalError => {
                Err(crate::Error::Sql(Self {
                    inner,
                    current_tuple: std::cell::RefCell::new(0),
                }))
            },
            _ => {
                Ok(Self {
                    inner,
                    current_tuple: std::cell::RefCell::new(0),
                })
            },
        }
    }
}
