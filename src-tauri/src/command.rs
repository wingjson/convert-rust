/*
 * @Date: 2024-02-04 09:17:54
 * @LastEditors: WWW
 * @LastEditTime: 2024-02-18 15:38:53
 * @FilePath: \ConvertTool\src-tauri\src\command.rs
 */
use crate::utilts::get_target;
use crate::db::{List,DbInfo};

/**
 * @description: convert
 * @return {*}
 */
#[tauri::command]
fn convert(source: &str, target_type: i32) -> String {
    let target = get_target(source, target_type);
    if let Some(t) = target {
        let insert_info = List {
            id: None,
            file: source.to_string(),
            original_file: t.clone(),
            status: 0,
        };
        DbInfo::insert_list(insert_info);
        return t;
    } else {
        // 如果 target 为 None，可以返回一个默认字符串，或者进行其他处理
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