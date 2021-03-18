/**
 * Tell if a transaction is open or not.
 */
pub fn is_in_transaction(transaction: &crate::Transaction<'_>) -> crate::Result<bool> {
    let status = transaction.connection.transaction_status()?;

    let in_transaction = status == libpq::transaction::Status::Active
        || status == libpq::transaction::Status::InTrans
        || status == libpq::transaction::Status::InError;

    Ok(in_transaction)
}

/**
 * In PostgreSQL, an error during a transaction cancels all the queries and
 * rollback the transaction on commit. This method returns the current
 * transaction's status. If no transactions are open, it returns `None`.
 */
pub fn is_transaction_ok(transaction: &crate::Transaction<'_>) -> crate::Result<Option<bool>> {
    if !is_in_transaction(transaction)? {
        return Ok(None);
    }

    let status = transaction.connection.transaction_status()?;

    Ok(Some(status == libpq::transaction::Status::InTrans))
}
