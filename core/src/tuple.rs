/**
 * Represent a set of values, a row of results.
 */
#[derive(Clone, Debug)]
pub struct Tuple<'a> {
    result: &'a libpq::Result,
    index: usize,
}

impl<'a> Tuple<'a> {
    pub(crate) fn from(result: &'a libpq::Result, index: usize) -> Self {
        Self { result, index }
    }

    /**
     * Retreive the value of field `name` of the tuple.
     *
     * # Panics
     *
     * Panics if `n` is greater than or equal to tuple length.
     */
    pub fn get<T>(&self, name: &str) -> T
    where
        T: crate::FromSql,
    {
        self.try_get(name)
            .unwrap_or_else(|err| panic!("Unable to retreive '{name}' field: {err}"))
    }

    /**
     * Retreive the value of field `name` of the tuple, or `None` if `n` is
     * greater than or equal to the length of the tuple.
     */
    pub fn try_get<T>(&self, name: &str) -> crate::Result<T>
    where
        T: crate::FromSql,
    {
        let n = match self.result.field_number(name) {
            Some(n) => n,
            None => return Err(crate::Error::MissingField(name.to_string())),
        };

        self.try_nth(n)
    }

    /**
     * Retreive the nth field.
     *
     * # Panics
     *
     * Panics if `n` is greater than or equal to the length of the tuple.
     */
    pub fn nth<T>(&self, n: usize) -> T
    where
        T: crate::FromSql,
    {
        self.try_nth(n)
            .unwrap_or_else(|err| panic!("Unable to retreive field {n}: {err}"))
    }

    /**
     * Retreive the nth field, or `None` if `n` is greater than or equal to the
     * length of the tuple.
     */
    pub fn try_nth<T>(&self, n: usize) -> crate::Result<T>
    where
        T: crate::FromSql,
    {
        let ty = self.field_type(n);
        let format = self.result.field_format(n);
        let value = self.result.value(self.index, n);

        crate::FromSql::from_sql(&ty, format, value)
    }

    /**
     * Number of field.
     */
    pub fn len(&self) -> usize {
        self.result.nfields()
    }

    /**
     * Is the tuple is empty (doesn’t contain field)?
     */
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /**
     * Retreive the name of field at position `n`, or `None` if `n` is greater
     * than or equal to the length of the tuple.
     */
    pub fn field_name(&self, n: usize) -> crate::Result<Option<String>> {
        Ok(self.result.field_name(n)?)
    }

    fn field_type(&self, n: usize) -> crate::pq::Type {
        let oid = self.result.field_type(n);

        match crate::pq::Type::try_from(oid) {
            Ok(ty) => ty,
            Err(_) => crate::pq::Type {
                oid,
                name: "unknow",
                descr: "Unknow type",
                kind: libpq::types::Kind::Composite,
            },
        }
    }
}
