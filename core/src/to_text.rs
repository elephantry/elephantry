pub trait ToText {
    fn to_text(&self) -> crate::Result<String>;

    fn error(&self, message: &str) -> crate::Error {
        crate::Error::ToSql {
            pg_type: crate::pq::types::TEXT,
            rust_type: std::any::type_name::<Self>().to_string(),
            message: message.to_string(),
        }
    }
}

impl<T: ToText> crate::ToSql for T {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::TEXT
    }

    fn to_text(&self) -> crate::Result<Option<String>> {
        ToText::to_text(self).and_then(|x| x.to_text())
    }

    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        ToText::to_text(self).and_then(|x| x.to_binary())
    }
}
