/**
 * This `struct` is created by the [`Connection::async`] method.
 *
 * [`Connection::async`]: crate::Connection::async
 */
#[derive(Debug)]
pub struct Async<'c> {
    last_result: Option<crate::Result<crate::pq::Result>>,
    connection: &'c std::sync::Mutex<libpq::Connection>,
}

impl<'c> std::future::Future for Async<'c> {
    type Output = crate::Result<crate::pq::Result>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let connection = match self.connection.lock() {
            Ok(connection) => connection,
            Err(_) => return std::task::Poll::Pending,
        };

        if let Some(result) = connection.result() {
            self.last_result = Some(result.try_into());
        } else {
            let last_result = self.last_result.take();

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

    /**
     * Async version of [`Connection::execute`].
     *
     * [`Connection::execute`]: crate::Connection::execute
     */
    pub async fn execute(self, query: &str) -> crate::Result<crate::pq::Result> {
        self.connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?
            .send_query(query)
            .map_err(crate::Error::Async)?;

        self.await
    }

    /**
     * Async version of [`Connection::query`].
     *
     * [`Connection::query`]: crate::Connection::query
     */
    pub async fn query<E: crate::Entity>(
        self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<crate::Rows<E>> {
        Ok(self.send_query(query, params).await?.into())
    }

    /**
     * Async version of [`Connection::query_one`].
     *
     * [`Connection::query_one`]: crate::Connection::query_one
     */
    pub async fn query_one<E: crate::Entity>(
        self,
        query: &str,
        params: &[&dyn crate::ToSql],
    ) -> crate::Result<E> {
        match self.query(query, params).await?.try_get(0) {
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

        for param in params.iter() {
            param_types.push(param.ty().oid);
            param_values.push(param.to_text()?);
        }

        self.connection
            .lock()
            .map_err(|e| crate::Error::Mutex(e.to_string()))?
            .send_query_params(
                query,
                &param_types,
                &param_values,
                &[],
                crate::pq::Format::Binary,
            )
            .map_err(crate::Error::Async)?;

        self.await
    }
}
