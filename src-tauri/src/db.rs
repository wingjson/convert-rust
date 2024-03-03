extern crate rusqlite;
use std::sync::MutexGuard;
use crate::global::GLOBAL_DB;
use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};

pub struct DbInfo{
    status: i32,
    conn: Connection
}

#[derive(Serialize, Deserialize,Clone)]
pub struct List{
    pub id: Option<i64>,
    pub source_file: String,
    pub target_file: String,
    pub status: i32
}

// Key TEXT PRIMARY KEY,
// Value TEXT NOT NULL,
// Description TEXT,
pub struct Config{
    pub key: String,
    pub value: String,
    pub description: String,
}

impl DbInfo{

    /**
     * @description: get convert list
     * @param {*} self
     * @return {*}
     */    
    pub fn get_list() -> Result<Vec<List>> {
        let db_lock = GLOBAL_DB.lock().unwrap();
        if let Some(ref conn) = *db_lock {
            let mut stmt = conn.prepare("SELECT * FROM info")?;
            let list_iter = stmt.query_map([], |row| {
                Ok(List {
                    id: row.get(0)?,
                    source_file: row.get(1)?,
                    target_file: row.get(2)?,
                    status: row.get(3)?
                })
            })?;
        
            let mut result = Vec::new();
            for list in list_iter {
                match list {
                    Ok(list_item) => {
                        result.push(list_item);
                    }
                    Err(err) => {
                        // 处理错误
                        eprintln!("Error: {:?}", err);
                    }
                }
            }
    
            Ok(result)
        }else {
            Ok(Vec::new())
        }
        
    }

    /**
     * @description: init table when enter
     * @param {*} Result
     * @return {*}
     */    
    pub fn init_table() -> Result<()> {
        let db_lock = GLOBAL_DB.lock().unwrap();
        if let Some(ref conn) = *db_lock {
            //first drop table
            let _ = conn.execute("DROP TABLE IF EXISTS info", []);
            let _ = conn.execute(
                "CREATE TABLE IF NOT EXISTS info (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    source_file TEXT NOT NULL,
                    target_file TEXT NOT NULL,
                    status INTEGER NOT NULL
                )",
                [],
            );
    
            // config table
            let _ = conn.execute(
                "CREATE TABLE IF NOT EXISTS config (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL,
                    description TEXT,
                    updatedAt DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            );
        }
        Ok(())
    }

    /**
     * @description: insert 
     * @param {*} self
     * @param {List} info
     * @return {*}
     */    
    pub fn insert_list(info: List) -> Result<i64> {
        let db_lock = GLOBAL_DB.lock().unwrap();
        if let Some(ref conn) = *db_lock {
            conn.execute(
                "INSERT INTO info (source_file, target_file, status) VALUES (?1, ?2, ?3)",
                params![info.source_file, info.target_file, info.status],
            )?;
            let last_inserted_id = conn.last_insert_rowid();
            Ok(last_inserted_id)
        }else{
            Ok(0)
        }
        
    }


    pub fn insert_config(info: Config) -> Result<i64> {
        let db_lock = GLOBAL_DB.lock().unwrap();
        if let Some(ref conn) = *db_lock {
            conn.execute(
                "INSERT INTO config (key, value, description) VALUES (?1, ?2, ?3)",
                params![info.key, info.value, info.description],
            )?;
            let last_inserted_id = conn.last_insert_rowid();
            Ok(last_inserted_id)
        }else {
            Ok(0)
        }
        
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        // 创建要插入的 List
        let test_info = List {
            id: None,
            source_file: "test".to_string(),
            target_file: "test".to_string(),
            status: 0,
        };

        // 调用 insert 方法，并对结果进行断言
        assert_eq!(DbInfo::insert_list(test_info).is_ok(), true);
    }

    #[test]
    fn test_get() {
        assert_eq!(DbInfo::get_list().is_ok(), true);
    }

    // 如果您希望测试失败的情况，您可以使用以下注释：
    // #[should_panic]
    // #[test]
    // fn test_fail() {
    //     assert_eq!(add(2, 2), 5); // 这将导致测试失败
    // }
}