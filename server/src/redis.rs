use crate::config::redis::Redis;
use chrono::Local;
use log::error;
use redis::{Client, Cmd};
use std::sync::{Arc, Mutex};

const ROOM_QUEUE_KEY: &str = "room_queue";
const ROOM_POOL_KEY: &str = "room_pool";
const GUARD_WHITELIST_KEY: &str = "guard_whitelist";
const GUARD_BLACKLIST_KEY: &str = "guard_blacklist";
const GUARD_WARN_MAP_KEY: &str = "guard_warn_map";

pub struct Connection {
    con: Arc<Mutex<redis::aio::Connection>>,
}

impl Connection {
    /// 打开连接，失败直接panic
    pub async fn connect(cfg: &Redis) -> Connection {
        match Client::open(&cfg.connection_parameter) {
            Ok(cli) => match cli.get_async_connection().await {
                Ok(con) => {
                    return Connection {
                        con: Arc::new(Mutex::new(con)),
                    }
                }
                Err(e) => error!("Redis connect error! {}", e),
            },
            Err(e) => error!("Redis open error! {}", e),
        }
        panic!()
    }

    fn query_bool(&self, cmd: Cmd) -> bool {
        let mut con = self.con.lock().unwrap();
        match cmd.query_async(&mut con).await {
            Ok(v) => v,
            Err(e) => {
                error!(e);
                false
            }
        }
    }

    fn query_i64(&self, cmd: Cmd) -> i64 {
        let mut con = self.con.lock().unwrap();
        match cmd.query_async(&mut con).await {
            Ok(v) => v,
            Err(e) => {
                error!(e);
                -1
            }
        }
    }

    /// 获取接下来的 n 个房间
    pub async fn get_next_rooms(&self, n: usize) -> Vec<String> {
        let mut con = self.con.lock().unwrap();
        let mut arr = vec![];
        for _ in 0..n {
            match Cmd::lpop(ROOM_QUEUE_KEY).query_async(&mut con).await {
                Ok(s) => arr.push(s),
                Err(e) => {
                    error!(e);
                    arr.push(String::new())
                }
            }
        }
        arr
    }

    /// 记录指定的房间，返回对应房间的记录值，失败记为 -1
    pub async fn record_rooms(&self, arr: &Vec<String>) -> Vec<i32> {
        let mut con = self.con.lock().unwrap();
        let mut ret = vec![];
        for s in arr.iter() {
            ret.push(
                match Cmd::hincr(ROOM_POOL_KEY, s, 1).query_async(&mut con).await {
                    Ok(v) => v,
                    Err(e) => {
                        error!(e);
                        -1
                    }
                },
            );
        }
        ret
    }

    /// 设置过期时间，单位为小时
    #[inline]
    fn expire(&self, key: &str, expire: usize) -> bool {
        self.query_bool(Cmd::expire(key, expire * 3600))
    }

    /// 获取哈希表键的次数，失败返回 -1 ，返回i64
    #[inline]
    fn hget(&self, key: &str, s: &str) -> i64 {
        self.query_i64(Cmd::hget(key, s))
    }

    /// 加入哈希表，值为当前时间戳，返回bool
    #[inline]
    fn hset_ts(&self, key: &str, s: &str) -> bool {
        self.query_bool(Cmd::hset(key, s, Local::now().timestamp()))
    }

    /// 从哈希表删除，返回bool
    #[inline]
    fn hdel(&self, key: &str, s: &str) -> bool {
        self.query_bool(Cmd::hdel(key, s))
    }

    /// 加入哈希表，值为原先值+1，返回新的值
    #[inline]
    fn hincr(&self, key: &str, s: &str) -> i64 {
        self.query_i64(Cmd::hincr(key, s, 1))
    }

    /// 设置白名单过期时间，单位为小时
    #[inline]
    pub async fn whitelist_expire(&self, expire: usize) -> bool {
        self.expire(GUARD_WHITELIST_KEY, expire)
    }

    /// 获取白名单过期时间，失败返回 -1
    #[inline]
    pub async fn whitelist_get(&self, s: &str) -> i64 {
        self.hget(GUARD_WHITELIST_KEY, s)
    }

    /// 加入到白名单
    #[inline]
    pub async fn whitelist_add(&self, s: &str) -> bool {
        self.hset_ts(GUARD_WHITELIST_KEY, s)
    }

    /// 从白名单删除
    #[inline]
    pub async fn whitelist_remove(&self, s: &str) -> bool {
        self.hdel(GUARD_WHITELIST_KEY, s)
    }

    /// 设置黑名单过期时间，单位为小时
    #[inline]
    pub async fn blacklist_expire(&self, expire: usize) -> bool {
        self.expire(GUARD_BLACKLIST_KEY, expire)
    }

    /// 获取黑名单过期时间，失败返回 -1
    #[inline]
    pub async fn blacklist_get(&self, s: &str) -> i64 {
        self.hget(GUARD_BLACKLIST_KEY, s)
    }

    /// 加入到黑名单
    #[inline]
    pub async fn blacklist_add(&self, s: &str) -> bool {
        self.hset_ts(GUARD_BLACKLIST_KEY, s)
    }

    /// 从黑名单删除
    #[inline]
    pub async fn blacklist_remove(&self, s: &str) -> bool {
        self.hdel(GUARD_BLACKLIST_KEY, s)
    }

    /// 设置警告名单过期时间，单位为小时
    #[inline]
    pub async fn warn_map_expire(&self, expire: usize) -> bool {
        self.expire(GUARD_WARN_MAP_KEY, expire)
    }

    /// 获取警告次数，失败返回 -1
    #[inline]
    pub async fn warn_map_get(&self, s: &str) -> i64 {
        self.hget(GUARD_WARN_MAP_KEY, s)
    }

    /// 加入到警告名单
    #[inline]
    pub async fn warn_map_add(&self, s: &str) -> i64 {
        self.hincr(GUARD_WARN_MAP_KEY, s)
    }

    /// 从警告名单删除
    #[inline]
    pub async fn warn_map_remove(&self, s: &str) -> bool {
        self.hdel(GUARD_WARN_MAP_KEY, s)
    }
}
