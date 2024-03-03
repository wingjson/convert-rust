/*
 * @Date: 2024-02-23 16:20:28
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-01 20:36:09
 * @FilePath: \convert-rust\src-tauri\src\main.rs
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

use std::env;
use global::{GLOBAL_CACHE, GLOBAL_DB, GLOBAL_JVM};
use jni::JNIVersion;
use jni::JavaVM;
use db::DbInfo;
use task::monitor_tasks;
use rusqlite::Connection;
use command::convert;

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
                            let _ = monitor_tasks().await;
                        }
                        Err(e) => {
                            eprintln!("Failed to initialize database: {}", e);
                        }
                    }
                });
                

                //#################################### jvm dir ###########################################
                let mut java_dir = lib_path.join("libs/env/bin").to_string_lossy().to_string();
                // need remove \\\\?\\
                if java_dir.starts_with("\\\\?\\") {
                    java_dir = java_dir.trim_start_matches("\\\\?\\").to_string();
                }
                print!("java dir: {}", java_dir);
                env::set_var("JAVA_HOME", &java_dir);
                let mut jar_path = db_dir.join("file.jar").to_string_lossy().to_string();
                // need remove \\\\?\\
                if jar_path.starts_with("\\\\?\\") {
                    jar_path = jar_path.trim_start_matches("\\\\?\\").to_string();
                }
                print!("jar path: {}", jar_path);
                let class_path_option = format!("-Djava.class.path={}", jar_path);
                let jvm_args = jni::InitArgsBuilder::new()
                    .version(JNIVersion::V8)
                    .option(&class_path_option)
                    .build().unwrap();

                let jvm = JavaVM::new(jvm_args).unwrap();
                let mut jvm_lock = GLOBAL_JVM.lock().unwrap();
                *jvm_lock = Some(jvm);
            
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
