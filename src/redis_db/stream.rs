use redis::{from_redis_value, FromRedisValue, RedisResult, Value};

pub struct Stream {
    id: Value,
    pub entries: Vec<Entry>,
}

impl Stream {
    #[allow(dead_code)]
    pub fn id<RV: FromRedisValue>(&self) -> RedisResult<RV> {
        from_redis_value(&self.id)
    }
}

impl FromRedisValue for Stream {
    fn from_redis_value(v: &Value) -> RedisResult<Stream> {
        let (id, entries): (Value, Vec<Entry>) = from_redis_value(v)?;
        Ok(Stream { id, entries })
    }
}

pub struct Entry {
    id: Value,
    pub key_values: Vec<Value>,
}

impl FromRedisValue for Entry {
    fn from_redis_value(v: &Value) -> RedisResult<Entry> {
        let (id, key_values): (Value, Vec<Value>) = from_redis_value(v)?;
        Ok(Entry { id, key_values })
    }
}

impl Entry {
    pub fn id<RV: FromRedisValue>(&self) -> RedisResult<RV> {
        from_redis_value(&self.id)
    }
}
