#[derive(Default)]
pub struct Builder<'a>(crate::Where<'a>);

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> crate::Where<'a> {
        self.0
    }

    pub fn r#in(mut self, element: &str, params: Vec<&'a dyn crate::ToSql>) -> Self {
        self.0 = crate::Where::new_in(element, params);

        self
    }

    pub fn not_in(mut self, element: &str, params: Vec<&'a dyn crate::ToSql>) -> Self {
        self.0 = crate::Where::new_not_in(element, params);

        self
    }

    pub fn group_condition(
        mut self,
        element: &str,
        operation: &str,
        params: Vec<&'a dyn crate::ToSql>,
    ) -> Self {
        self.0 = crate::Where::new_group_condition(element, operation, params);

        self
    }

    pub fn and_where(mut self, element: &str, params: Vec<&'a dyn crate::ToSql>) -> Self {
        self.0.and_where(element, params);

        self
    }

    pub fn or_where(mut self, element: &str, params: Vec<&'a dyn crate::ToSql>) -> Self {
        self.0.or_where(element, params);

        self
    }

    pub fn add_where(
        mut self,
        element: &str,
        params: Vec<&'a dyn crate::ToSql>,
        operator: &str,
    ) -> Self {
        self.0.add_where(element, params, operator);

        self
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn builder() {
        let w = crate::Where::builder()
            .and_where("a", vec![&1])
            .and_where("b", Vec::new())
            .or_where("c", vec![&2, &3])
            .or_where("d", vec![&4])
            .add_where("e", Vec::new(), "like")
            .build();

        assert_eq!(w.to_string(), "(((a and b) or c or d) like e)");
        assert_eq!(
            w.params()
                .iter()
                .map(|x| x.to_text().unwrap())
                .collect::<Vec<_>>(),
            vec![
                Some(vec![b'1', 0]),
                Some(vec![b'2', 0]),
                Some(vec![b'3', 0]),
                Some(vec![b'4', 0])
            ],
        );
    }
}
