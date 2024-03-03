use rusqlite::params;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::global::GLOBAL_DB;
use crate::convert::convert;

async fn process_task(id: i64, source_file: String,target_file: String) {

    println!("Processing task {}: {}: {}", id, source_file, target_file);

    // load db
    let db = GLOBAL_DB.lock().unwrap();
    if let Some(conn) = db.as_ref() {
        conn.execute("UPDATE info SET status = 1 WHERE id = ?", params![id]).expect("Failed to update task status.");
        let convert_result = convert(source_file, target_file);
        if let Ok(result) = convert_result {
            if result == "success" {
                conn.execute("UPDATE info SET status = 2 WHERE id = ?", params![id])
            .expect("Failed to update task status.");
            }else{
                conn.execute("UPDATE info SET status = 3 WHERE id = ?", params![id]).expect("Failed to update task status.");
            }
        }
        
    }
}

pub(crate) async fn monitor_tasks() -> Result<(), rusqlite::Error> {
    let semaphore = Arc::new(Semaphore::new(3));
    println!("Starting task monitor...");
    loop {
        let available_permits = semaphore.available_permits();
        if available_permits == 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            continue;
        }

        //the max number of tasks to fetch
        let tasks_to_fetch = available_permits.min(3);

        let tasks: Vec<(i64, String, String)> = {
            let db_lock = GLOBAL_DB.lock().unwrap();
            if let Some(conn) = db_lock.as_ref() {
                let mut stmt = conn.prepare("SELECT id, source_file, target_file FROM info WHERE status = 0 LIMIT ?")?;
                let results: rusqlite::Result<Vec<(i64, String, String)>> = stmt.query_map(params![tasks_to_fetch], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                })?.collect();
                match results {
                    Ok(tasks) => tasks,
                    Err(e) => {
                        eprintln!("Query error: {}", e);
                        vec![]
                    },
                }
            } else {
                vec![]
            }
        };

        for (id, source_file, target_file) in tasks {
            let semaphore = semaphore.clone();
            tokio::spawn(async move {
                let _permit = semaphore.acquire().await.expect("Failed to acquire semaphore.");
                process_task(id, source_file, target_file).await;
            });
        }
        //await for next check
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}


