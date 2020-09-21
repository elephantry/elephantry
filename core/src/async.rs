#[derive(Debug)]
pub struct Async<'c> {
    last_result: Option<crate::pq::Result>,
    connection: &'c std::sync::Mutex<libpq::Connection>,
}

impl<'c> std::future::Future for Async<'c> {
    type Output = crate::pq::Result;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.connection.lock().unwrap().result() {
            use std::convert::TryInto;
            self.last_result = Some(result.try_into().unwrap());
        }
        else {
            let last_result = std::mem::replace(&mut self.last_result, None);

            if let Some(result) = last_result {
                return std::task::Poll::Ready(result);
            }
        }

        ctx.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

impl<'c> Async<'c> {
    pub(crate) fn new(connection: &'c std::sync::Mutex<libpq::Connection>) -> Self {
        Self {
            last_result: None,
            connection,
        }
    }

    pub async fn execute(
        self,
        query: &str,
    ) -> crate::Result<crate::pq::Result> {
        self.connection
            .lock().unwrap()
            .send_query(&query)
            .map_err(crate::Error::Async)?;

        Ok(self.await)
    }

    pub async fn query<E: crate::Entity>(
        self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::Rows<E>> {
        Ok(self.send_query(&query, params).await?.into())
    }

    pub async fn query_one<E: crate::Entity>(
        self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<E> {
        match self.query(&query, params).await?.try_get(0) {
            Some(e) => Ok(e),
            None => Err(crate::Error::MissingField("0".to_string())),
        }
    }

    async fn send_query(
        self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::pq::Result> {
        let mut param_types = Vec::new();
        let mut param_values = Vec::new();
        let mut param_formats = Vec::new();

        for param in params.iter() {
            param_types.push(param.ty().oid);
            param_values.push(param.to_sql()?);
            param_formats.push(param.format());
        }

        self.connection
            .lock().unwrap()
            .send_query_params(
                query,
                &param_types,
                &param_values,
                &param_formats,
                crate::pq::Format::Binary,
            )
            .map_err(crate::Error::Async)?;

        Ok(self.await)
    }
}
