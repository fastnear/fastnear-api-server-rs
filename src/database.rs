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
    connection: &mut redis::aio::MultiplexedConnection,
    prefix: &str,
    account_id: &str,
) -> Result<Vec<(String, String)>, DatabaseError> {
    let start = std::time::Instant::now();

    let res: redis::RedisResult<Vec<(String, String)>> = redis::cmd("HGETALL")
        .arg(format!("{}:{}", prefix, account_id))
        .query_async(connection)
        .await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_with_prefix {}:{}",
        duration,
        prefix,
        account_id);

    Ok(res?)
}

pub(crate) async fn query_with_prefix_parse(
    connection: &mut redis::aio::MultiplexedConnection,
    prefix: &str,
    account_id: &str,
) -> Result<Vec<(String, Option<BlockHeight>)>, DatabaseError> {
    let res = query_with_prefix(connection, prefix, account_id).await?;

    Ok(res.into_iter().map(|(k, v)| (k, v.parse().ok())).collect())
}

pub(crate) async fn query_zset_by_score(
    connection: &mut redis::aio::MultiplexedConnection,
    key: &str,
    limit: usize,
) -> Result<Vec<String>, DatabaseError> {
    let start = std::time::Instant::now();

    let res: redis::RedisResult<Vec<String>> = redis::cmd("ZRANGE")
        .arg(key)
        .arg("inf")
        .arg(0)
        .arg("BYSCORE")
        .arg("REV")
        .arg("LIMIT")
        .arg(0)
        .arg(limit)
        .query_async(connection)
        .await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_zset {}",
        duration,
        key);

    Ok(res?)
}

pub(crate) async fn query_balances(
    connection: &mut redis::aio::MultiplexedConnection,
    pairs: &[(&str, &str)],
) -> Result<Vec<Option<String>>, DatabaseError> {
    let start = std::time::Instant::now();

    let mut pipe = redis::pipe();
    for (token_id, account_id) in pairs {
        pipe.cmd("HGET")
            .arg(format!("b:{}", token_id))
            .arg(account_id);
    }

    let res: redis::RedisResult<Vec<Option<String>>> = pipe.query_async(connection).await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_balances {} pairs",
        duration,
        pairs.len()
    );

    Ok(res?)
}

pub(crate) async fn query_hget(
    connection: &mut redis::aio::MultiplexedConnection,
    key: &str,
    field: &str,
) -> Result<Option<String>, DatabaseError> {
    let start = std::time::Instant::now();

    let res: redis::RedisResult<Option<String>> = redis::cmd("HGET")
        .arg(key)
        .arg(field)
        .query_async(connection)
        .await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_hget {} {}",
        duration,
        key, field);

    Ok(res?)
}

pub(crate) async fn query_get(
    connection: &mut redis::aio::MultiplexedConnection,
    key: &str,
) -> Result<Option<String>, DatabaseError> {
    let start = std::time::Instant::now();

    let res: redis::RedisResult<Option<String>> =
        redis::cmd("GET").arg(key).query_async(connection).await;

    let duration = start.elapsed().as_millis();

    tracing::debug!(target: TARGET_DB, "Query {}ms: query_get {}",
        duration,
        key);

    Ok(res?)
}
