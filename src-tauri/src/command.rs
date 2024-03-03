/*
 * @Date: 2024-02-04 09:17:54
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-01 18:03:24
 * @FilePath: \convert-rust\src-tauri\src\command.rs
 */
use crate::utilts::get_target;
use crate::db::{List,DbInfo};

/**
 * @description: convert
 * @return {*}
 */
#[tauri::command]
pub fn convert(source: &str, target_type: i32) ->String{
    let target = get_target(source, target_type);
    if let Some(t) = target {
        let insert_info = List {
            id: None,
            source_file: source.to_string(),
            target_file: t.clone(),
            status: 0,
        };
        if let Ok(id) = DbInfo::insert_list(insert_info)  {
            return id.to_string()
            
        }else{
            return "error".to_string();
        }
    } else {
        return "default_string".to_string();
    }
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