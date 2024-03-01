/*
 * @Date: 2024-02-20 09:30:25
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-01 16:32:40
 * @FilePath: \convertRust\src-tauri\src\task.rs
 */
/*
 * @Date: 2024-02-20 09:30:25
 * @LastEditors: WWW
 * @LastEditTime: 2024-03-01 16:03:40
 * @FilePath: \convertRust\src-tauri\src\task.rs
 */
use sled::{open, Db, IVec};
use std::sync::Arc;
use std::thread;
use tokio::sync::{mpsc, Semaphore};
use crate::global::GLOBAL_QUEUE;
use crate::db::{List,DbInfo};

// fn init_db() -> Db {
//     sled::open("my_queue_db").expect("failed to open sled db")
// }


// /// 向队列添加元素。
pub(crate) fn enqueue(value: &[u8]) -> sled::Result<()> {
    if let Ok(mut queue_lock) = GLOBAL_QUEUE.try_lock() {
            println!("Lock acquired in enqueue");
                    // ...
    } else {
            println!("Failed to acquire lock in enqueue");
    }
    // let queue_lock = GLOBAL_QUEUE.lock().unwrap();
    // if let Some(ref conn) = *queue_lock {
    //     print!("conn:get success");
    //     // let key = conn.generate_id()?.to_be_bytes().to_vec(); // 转换为 Vec<u8>
    //     // conn.insert(key, value)?;   
    // }
    Ok(())

}

// pub fn enqueue(db: &Db, value: &[u8]) -> sled::Result<()> {
//     let key = db.generate_id()?.to_be_bytes().to_vec(); // 转换为 Vec<u8>
//     db.insert(key, value)?;
//     Ok(())
// }

// / 从队列中移除并返回第一个元素的值。
// fn dequeue() -> sled::Result<Option<IVec>> {
//     let queue_lock = GLOBAL_QUEUE.lock().unwrap();
//     if let Some(ref conn) = *queue_lock {
//         conn.pop_min().map(|opt| opt.map(|(_key, value)| value))
//     }
// }


// 监听数据库变更，并将变更发送到通道
pub(crate) fn watch_prefix_and_send_events(tx: mpsc::Sender<IVec>) {
    let queue_lock = GLOBAL_QUEUE.lock().unwrap();
    if let Some(conn) = queue_lock.as_ref() {
        let conn_clone = conn.clone(); // 克隆数据库连接
        // thread::spawn(move || {
            let prefix = b"";
            let watch = conn_clone.watch_prefix(prefix); 
            for event in watch {
                if let sled::Event::Insert { key: _, value } = event {
                    // if let Err(e) = tx.blocking_send(value) {
                    //     eprintln!("Error sending value through channel: {}", e);
                    // }
                }
            }
        // });
    }
}


fn watch_prefix(db: &Db) {
    let prefix = b""; // 监听所有键的变更
    let mut watch = db.watch_prefix(prefix);

    // 使用另一个线程来监听变更
    thread::spawn(move || {
        for event in watch {
            println!("Database change event: {:?}", event);
            // 根据变更事件做相应处理
        }
    });
}

// 异步处理任务
async fn process_task(value: IVec) {
    println!("Processing task: {:?}", value);
    // 在这里实现任务处理逻辑
}

// 异步执行任务处理器
pub(crate) async fn task_processor(mut rx: mpsc::Receiver<IVec>, semaphore: Arc<Semaphore>) {
    while let Some(value) = rx.recv().await {
        print!("Received task: {:?}", value);
        let semaphore_clone = semaphore.clone();
        let permit = semaphore_clone.acquire_owned().await.expect("Failed to acquire semaphore");
        tokio::spawn(async move {
            process_task(value).await;
            drop(permit); // 任务完成后释放信号量
        });
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use sled::{Config};
//     use tokio::time::{sleep, Duration};
//     #[tokio::test]
//     async fn test_enqueue() {
//             // let queue = sled::open("test").expect("failed to open sled db");
//             // 假设 GLOBAL_QUEUE 是一个全局的 Mutex<Option<sled::Db>>
//             let mut queue_lock = GLOBAL_QUEUE.lock().unwrap();
//             *queue_lock = Some("queue.clone()".to_string());
//             // print!("conn:init success");
//             // watch_prefix(&queue);
//             // // enqueue(&queue,b"Hello, world!").expect("Failed to enqueue");
//             // enqueue(b"Hello, world!").expect("Failed to enqueue");
//             // thread::sleep(Duration::from_secs(1)); // 等待一会儿，以便观察监听器输出
            
//             // 在 enqueue 函数中
//             test_lock();
            
    
//     }

//     fn test_lock(){
//         println!("Attempting to acquire lock in enqueue");
//         if let Ok(mut queue_lock) = GLOBAL_QUEUE.try_lock() {
//             println!("Lock acquired in enqueue");
//             // ...
//         } else {
//             println!("Failed to acquire lock in enqueue");
//         }
//     }

    
// }
