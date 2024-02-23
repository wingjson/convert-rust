/*
 * @Date: 2024-02-01 08:55:36
 * @LastEditors: WWW
 * @LastEditTime: 2024-02-23 15:39:41
 * @FilePath: \ConvertTool\src-tauri\src\main.rs
 */
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::Connection;
use tauri::{api::path::app_data_dir, Manager};
mod db;
mod convert;
mod global;
mod jvm;
mod utilts;
mod command;
mod task;

use std::env;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::runtime::Runtime;
use global::{GLOBAL_CACHE, GLOBAL_DB, GLOBAL_JVM};
use tauri::api::path;
use jni::JNIVersion;
use jni::JavaVM;

fn main() {
    // create runtime
    let rt = Arc::new(Runtime::new().expect("Failed to create Tokio runtime"));
    tauri::Builder::default()
        .setup(move |app| {
            //#################################### start Queue ###########################################
            let rt = rt.clone();
            let (_tx, rx) = mpsc::channel(32);
            // rt.spawn(task::producer(tx));
            rt.spawn(task::consumer(rx));

            //#################################### cache dir ###########################################
            let resource_path = app.path_resolver().resource_dir();
            let cache = GLOBAL_CACHE.lock().unwrap();

            println!("资源目录路径: {:?}", resource_path);
            if let Some(lib_path) = resource_path  {
                let db_dir = lib_path.join("libs");
                cache.insert("DataPath".to_owned(), db_dir.to_string_lossy().into_owned());
                
                let db_path = db_dir.join("db.db");
                let connection = Connection::open(db_path).expect("Failed to open database");
                let mut db_lock = GLOBAL_DB.lock().unwrap();
                *db_lock = Some(connection);


            //*******************************jvm dir************************ */
            let java_path = db_dir.join("myjre/bin").to_string_lossy().to_string();
            env::set_var("JAVA_HOME", java_path);
            let jar_path = db_dir.join("libs/file.jar").to_string_lossy().to_string();
            let class_path_option = format!("-Djava.class.path={}", jar_path);
            let jvm_args = jni::InitArgsBuilder::new()
                .version(JNIVersion::V8)
                .option(&class_path_option)
                .build().unwrap();

            let jvm = JavaVM::new(jvm_args).unwrap();
            let mut jvm_lock = GLOBAL_JVM.lock().unwrap();
            *jvm_lock = Some(jvm);
            
            }











            // let cache = GLOBAL_CACHE.lock().unwrap();
            // let current_dir_str = current_dir.to_string_lossy().into_owned();
            // cache.insert("DataPath".to_owned(), current_dir_str);
            
            //**************************************** db **************************************** */
            // let db_path = current_dir.join("db.db");
            // let connection = Connection::open(db_path).expect("Failed to open database");
            // let mut db_lock = GLOBAL_DB.lock().unwrap();
            // *db_lock = Some(connection);



            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
