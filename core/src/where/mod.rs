pub mod builder;

use builder::Builder;

/**
 * This struct represents a WHERE clause of a SQL statement. It deals with AND &
 * OR operator you can add using handy methods. This allows you to build
 * queries dynamically.
 */
#[derive(Clone, Default)]
pub struct Where<'a> {
    element: Option<String>,
    operator: String,
    stack: Vec<Self>,
    params: Vec<&'a dyn crate::ToSql>,
}

impl<'a> Where<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(element: &str, params: Vec<&'a dyn crate::ToSql>) -> Self {
        Self {
            element: Some(element.to_string()),
            operator: String::new(),
            stack: Vec::new(),
            params,
        }
    }

    pub fn builder() -> Builder<'a> {
        Builder::new()
    }

    /**
     * Create an escaped IN clause.
     */
    pub fn new_in(element: &str, params: Vec<&'a dyn crate::ToSql>) -> Self {
        Self::new_group_condition(element, "in", params)
    }

    /**
     * Create an escaped NOT IN clause.
     */
    pub fn new_not_in(element: &str, params: Vec<&'a dyn crate::ToSql>) -> Self {
        Self::new_group_condition(element, "not in", params)
    }

    pub fn new_group_condition(
        element: &str,
        operation: &str,
        params: Vec<&'a dyn crate::ToSql>,
    ) -> Self {
        let element = format!(
            "{element} {operation} ({})",
            params
                .iter()
                .map(|_| "$*".to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );

        Self::from(&element, params)
    }

    /**
     * Is it a fresh brand new object?
     */
    pub fn is_empty(&self) -> bool {
        self.element.is_none() && self.stack.len() == 0
    }

    pub fn and_where(&mut self, element: &str, params: Vec<&'a dyn crate::ToSql>) {
        self.add_where(element, params, "and");
    }

    pub fn or_where(&mut self, element: &str, params: Vec<&'a dyn crate::ToSql>) {
        self.add_where(element, params, "or");
    }

    /**
     * You can add a new WHERE clause with your own operator.
     */
    pub fn add_where(&mut self, element: &str, params: Vec<&'a dyn crate::ToSql>, operator: &str) {
        self.op(&Self::from(element, params), operator);
    }

    fn op(&mut self, rhs: &Self, operator: &str) {
        if rhs.is_empty() {
            return;
        }

        if self.is_empty() {
            *self = rhs.clone();
            return;
        }

        if let Some(element) = &self.element {
            self.stack = vec![Self::from(element, self.params.clone()), rhs.clone()];
            self.element = None;
            self.params = Vec::new();
        } else if self.operator == operator {
            self.stack.push(rhs.clone());
        } else {
            let mut new = Self::new();
            new.stack = self.stack.clone();
            new.operator = self.operator.clone();

            self.stack = vec![new, rhs.clone()];
        }

        self.operator = operator.to_string();
    }

    fn parse(&self) -> String {
        if let Some(element) = &self.element {
            return element.clone();
        }

        let mut stack = Vec::<String>::new();
        for w in &self.stack {
            stack.push(w.parse());
        }

        format!("({})", stack.join(&format!(" {} ", self.operator)))
    }

    /**
     * Get all the params back for the prepared statement.
     */
    pub fn params(&self) -> Vec<&dyn crate::ToSql> {
        if self.is_empty() {
            return Vec::new();
        }

        if self.element.is_some() {
            return self.params.to_vec();
        }

        let mut params = Vec::new();

        for w in &self.stack {
            params.extend(w.params())
        }

        params
    }
}

impl<'a> ToString for Where<'a> {
    fn to_string(&self) -> String {
        if self.is_empty() {
            return "true".to_string();
        }

        self.parse()
    }
}

impl<'a> std::ops::BitAnd for Where<'a> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        result &= rhs;

        result
    }
}

impl<'a> std::ops::BitAndAssign for Where<'a> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.op(&rhs, "and");
    }
}

impl<'a> std::ops::BitOr for Where<'a> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        result |= rhs;

        result
    }
}

impl<'a> std::ops::BitOrAssign for Where<'a> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.op(&rhs, "or");
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn create() {
        crate::Where::new();
        crate::Where::from("a = pika($*, $*)", vec![&1, &2]);
    }

    #[test]
    fn create_in() {
        let w = crate::Where::new_in("b", vec![&1, &2, &3, &4]);
        assert_eq!(w.to_string(), "b in ($*, $*, $*, $*)");
    }

    #[test]
    fn create_not_in() {
        let w = crate::Where::new_not_in("b", vec![&1, &2]);
        assert_eq!(w.to_string(), "b not in ($*, $*)");
    }

    #[test]
    fn empty() {
        let mut w = crate::Where::new();
        assert!(w.is_empty());

        w.and_where("a", Vec::new());
        assert!(!w.is_empty());
    }

    #[test]
    fn and_where() {
        let w = crate::Where::from("a", vec![&1]);

        let mut a = w.clone() & crate::Where::new();
        assert_eq!(a.to_string(), "a");

        a.and_where("b", Vec::new());
        assert_eq!(a.to_string(), "(a and b)");

        let b = a.clone() & crate::Where::from("c", vec![&2, &3]);
        assert_eq!(b.to_string(), "(a and b and c)");

        assert_eq!(b.params().len(), 3);
    }

    #[test]
    fn or_where() {
        let w = crate::Where::from("a", vec![&1]);

        let mut a = w.clone() | crate::Where::new();
        assert_eq!(a.to_string(), "a");

        a.or_where("b", Vec::new());
        assert_eq!(a.to_string(), "(a or b)");

        let b = a.clone() | crate::Where::from("c", vec![&2, &3]);
        assert_eq!(b.to_string(), "(a or b or c)");

        assert_eq!(
            b.params()
                .iter()
                .map(|x| x.to_text().unwrap())
                .collect::<Vec<_>>(),
            vec![
                Some(vec![b'1', 0]),
                Some(vec![b'2', 0]),
                Some(vec![b'3', 0])
            ],
        );
    }

    #[test]
    fn and_or_where() {
        let mut w = crate::Where::from("a", vec![&1]);
        w.and_where("b", Vec::new());
        w.or_where("c", vec![&2, &3]);
        w.or_where("d", vec![&4]);
        w.add_where("e", Vec::new(), "like");

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
