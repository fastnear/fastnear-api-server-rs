use crate::api::BlockHeight;

const TARGET_DB: &str = "database";

#[derive(Debug)]
pub enum DatabaseError {
    RedisError(redis::RedisError),
}

impl From<redis::RedisError> for DatabaseError {
    fn from(error: redis::RedisError) -> Self {
        DatabaseError::RedisError(error)
    }
}

pub(crate) async fn query_with_prefix(
    mut connection: redis::aio::MultiplexedConnection,
    prefix: &str,
    account_id: &str,
) -> Result<Vec<(String, String)>, DatabaseError> {
    let start = std::time::Instant::now();

    let res: redis::RedisResult<Vec<(String, String)>> = redis::cmd("HGETALL")
        .arg(format!("{}:{}", prefix, account_id))
        .query_async(&mut connection)
        .await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_with_prefix {}:{}",
        duration,
        prefix,
        account_id);

    Ok(res?)
}

pub(crate) async fn query_with_prefix_parse(
    connection: redis::aio::MultiplexedConnection,
    prefix: &str,
    account_id: &str,
) -> Result<Vec<(String, Option<BlockHeight>)>, DatabaseError> {
    let res = query_with_prefix(connection, prefix, account_id).await?;

    Ok(res.into_iter().map(|(k, v)| (k, v.parse().ok())).collect())
}
