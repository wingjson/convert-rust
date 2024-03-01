/*
 * @Date: 2024-02-04 09:17:54
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-01 16:32:22
 * @FilePath: \convertRust\src-tauri\src\command.rs
 */
use crate::utilts::get_target;
use crate::db::{List,DbInfo};
use crate::task::enqueue;

/**
 * @description: convert
 * @return {*}
 */
#[tauri::command]
pub fn convert(source: &str, target_type: i32) ->String{
    let _ = enqueue(b"test");
    return "default_string".to_string();
    // let target = get_target(source, target_type);
    // if let Some(t) = target {
    //     let insert_info = List {
    //         id: None,
    //         source_file: source.to_string(),
    //         target_file: t.clone(),
    //         status: 0,
    //     };
    //     let mut queue_info = insert_info.clone();
    //     if let Ok(id) = DbInfo::insert_list(insert_info)  {
    //         // queue_info.id = Some(id);
    //         // let serialized_value = serde_json::to_string(&queue_info).unwrap();
    //         // let _ = enqueue(serialized_value);
    //         return id.to_string()
            
    //     }else{
    //         return "error".to_string();
    //     }
    // } else {
    //     return "default_string".to_string();
    // }
}


/**
 * @description: set output folder
 * @return {*}
 */
#[tauri::command]
fn set_output(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/**
 * @description: get status list
 * @return {*}
 */
#[tauri::command]
fn get_list() -> String {
    format!("Hello, {}! You've been greeted from Rust!", "list")
}