/*
 * @Date: 2024-02-23 16:20:28
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-01 16:06:56
 * @FilePath: \convertRust\src-tauri\src\main.rs
 */
/*
 * @Date: 2024-02-01 08:55:36
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-01 09:51:10
 * @FilePath: \convertRust\src-tauri\src\main.rs
 */
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod convert;
mod global;
mod jvm;
mod utilts;
mod command;
mod task;
mod test;

use std::env;
use std::sync::Arc;
use global::{GLOBAL_CACHE, GLOBAL_DB, GLOBAL_JVM, GLOBAL_QUEUE};
use jni::JNIVersion;
use jni::JavaVM;
use db::DbInfo;
use rusqlite::Connection;
use command::{convert};
use tokio::sync::{mpsc, Semaphore};

fn main() {
    // create runtime
    tauri::Builder::default()
        .setup(move |app| {

            let resource_path = app.path_resolver().resource_dir();            
            if let Some(lib_path) = resource_path  {
                 //#################################### cache dir ###########################################
                let cache = GLOBAL_CACHE.lock().unwrap();
                let db_dir = lib_path.join("libs");
                cache.insert("DataPath".to_owned(), db_dir.to_string_lossy().into_owned());
                
                //#################################### db dir ###########################################
                let db_path = db_dir.join("db.db");
                let connection = Connection::open(db_path).expect("Failed to open database");
                let mut db_lock = GLOBAL_DB.lock().unwrap();
                *db_lock = Some(connection);
                tauri::async_runtime::spawn(async move {
                    //init db
                    match DbInfo::init_table() {
                        Ok(_) => {
                            println!("Database initialized successfully.");
                        }
                        Err(e) => {
                            eprintln!("Failed to initialize database: {}", e);
                        }
                    }
                });
                

                //####################################start queue ###########################################
                let queue_path = db_dir.join("queue");
                tauri::async_runtime::spawn(async move {
                    let queue = sled::open(queue_path).expect("failed to open sled db");
                    // 假设 GLOBAL_QUEUE 是一个全局的 Mutex<Option<sled::Db>>
                    let mut queue_lock = GLOBAL_QUEUE.lock().unwrap();
                    *queue_lock = Some(queue.clone());
                    let semaphore = Arc::new(Semaphore::new(3)); // 最多三个并发任务
                    let (tx, rx) = mpsc::channel(32);
                    // 这里假设 watch_prefix_and_send_events 和 task_processor 是适配异步的
                    task::watch_prefix_and_send_events(tx);
                    tokio::spawn(task::task_processor(rx, semaphore));
                });

                //#################################### jvm dir ###########################################
                // let mut java_dir = lib_path.join("libs/jre/bin").to_string_lossy().to_string();
                // // need remove \\\\?\\
                // if java_dir.starts_with("\\\\?\\") {
                //     java_dir = java_dir.trim_start_matches("\\\\?\\").to_string();
                // }
                // env::set_var("JAVA_HOME", &java_dir);
                // let jar_path = db_dir.join("file.jar").to_string_lossy().to_string();
                // let class_path_option = format!("-Djava.class.path={}", jar_path);
                // let jvm_args = jni::InitArgsBuilder::new()
                //     .version(JNIVersion::V8)
                //     .option(&class_path_option)
                //     .build().unwrap();

                // let jvm = JavaVM::new(jvm_args).unwrap();
                // let mut jvm_lock = GLOBAL_JVM.lock().unwrap();
                // *jvm_lock = Some(jvm);
            
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
