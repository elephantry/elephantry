/**
 * Determines if the connection is no longer usable.
 */
pub fn has_broken(connection: &crate::Connection) -> crate::Result<bool> {
    let status = connection
        .connection
        .lock()
        .map_err(|e| crate::Error::Mutex(e.to_string()))?
        .status();

    Ok(status == libpq::connection::Status::Bad)
}

/**
 * Check if a notification is pending. If so, the payload is returned.
 * Otherwise, `None` is returned.
 */
pub fn notifies(connection: &crate::Connection) -> crate::Result<Option<crate::pq::Notify>> {
    let connection = connection
        .connection
        .lock()
        .map_err(|e| crate::Error::Mutex(e.to_string()))?;

    connection.consume_input().ok();
    Ok(connection.notifies())
}
