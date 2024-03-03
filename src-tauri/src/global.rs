/*
 * @Date: 2024-03-01 17:40:15
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-01 18:28:10
 * @FilePath: \convert-rust\src-tauri\src\global.rs
 */
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Mutex;
use jni::JavaVM;
use moka::sync::Cache;

pub static GLOBAL_DB: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

pub static GLOBAL_JVM: Lazy<Mutex<Option<JavaVM>>> = Lazy::new(|| Mutex::new(None));

// pub static GLOBAL_QUEUE: Lazy<Mutex<Option<Db>>> = Lazy::new(|| Mutex::new(None));

// pub static GLOBAL_TX: Lazy<Mutex<Option<Sender<i64>>>> = Lazy::new(|| Mutex::new(None));

pub static GLOBAL_CACHE: Lazy<Mutex<Cache<String, String>>> = Lazy::new(|| {
    let cache: Cache<String, String> = Cache::new(10_000);
    // 进行数据库初始化或设置
    Mutex::new(cache)
});
