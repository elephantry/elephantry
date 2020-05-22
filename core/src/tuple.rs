#[derive(Clone, Debug)]
pub struct Tuple<'a> {
    result: &'a libpq::Result,
    index: usize,
}

impl<'a> Tuple<'a> {
    pub fn from(result: &'a libpq::Result, index: usize) -> Self {
        Self {
            result,
            index,
        }
    }

    pub fn get<T>(&self, name: &str) -> T
    where
        T: crate::FromSql,
    {
        self.try_get(name).unwrap_or_else(|err| {
            panic!("Unable to retreive '{}' field: {}", name, err)
        })
    }

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

    pub fn nth<T>(&self, n: usize) -> T
    where
        T: crate::FromSql,
    {
        self.try_nth(n).unwrap_or_else(|err| {
            panic!("Unable to retreive field {}: {}", n, err)
        })
    }

    pub fn try_nth<T>(&self, n: usize) -> crate::Result<T>
    where
        T: crate::FromSql,
    {
        let ty = self.result.field_type(n).unwrap_or(crate::pq::ty::TEXT);
        let format = self.result.field_format(n);
        let value = self.result.value(self.index, n);

        crate::FromSql::from_sql(&ty, format, value)
    }

    pub fn len(&self) -> usize {
        self.result.nfields()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn field_name(&self, n: usize) -> Option<String> {
        self.result.field_name(n)
    }
}
