pub mod result {
    pub fn state(result: &crate::pq::Result) -> Option<crate::pq::State> {
        result.inner.error_field(libpq::result::ErrorField::Sqlstate)
            .map(crate::pq::State::from_code)
    }
}
