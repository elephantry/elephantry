#[cfg_attr(docsrs, doc(cfg(feature = "xml")))]
impl crate::ToSql for xmltree::Element {
    fn ty(&self) -> crate::pq::Type {
        crate::pq::types::XML
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/xml.c#L336
     */
    fn to_text(&self) -> crate::Result<Option<String>> {
        let mut vec = Vec::new();

        self.write(&mut vec)
            .map_err(|e| self.error(&e.to_string()))?;

        Ok(Some(String::from_utf8(vec)?))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/xml.c#L418
     */
    fn to_binary(&self) -> crate::Result<Option<Vec<u8>>> {
        let mut buf = Vec::new();

        self.write(&mut buf)
            .map_err(|e| self.error(&e.to_string()))?;

        Ok(Some(buf))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "xml")))]
impl crate::FromSql for xmltree::Element {
    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/xml.c#L258
     */
    fn from_text(ty: &crate::pq::Type, raw: Option<&str>) -> crate::Result<Self> {
        xmltree::Element::parse(crate::from_sql::not_null(raw)?.as_bytes())
            .map_err(|_| Self::error(ty, raw))
    }

    /*
     * https://github.com/postgres/postgres/blob/REL_12_0/src/backend/utils/adt/xml.c#L351
     */
    fn from_binary(ty: &crate::pq::Type, raw: Option<&[u8]>) -> crate::Result<Self> {
        let s = String::from_binary(ty, raw)?;

        Self::from_text(ty, Some(&s))
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "xml")))]
impl crate::entity::Simple for xmltree::Element {}

#[cfg(test)]
mod test {
    static XML: &str = r#"<?xml version="1.0"?>
<!-- Awesome data incoming -->
<data awesome="true">
  <datum>Science</datum>
  <datum><![CDATA[Literature]]></datum>
  <datum>Math &gt; others</datum>
</data>"#;

    crate::sql_test!(
        xml,
        xmltree::Element,
        [(
            &format!("'{}'", super::XML),
            xmltree::Element::parse(super::XML.as_bytes()).unwrap()
        )]
    );
}
