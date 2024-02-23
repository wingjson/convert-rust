/*
 * @Date: 2024-02-20 09:30:25
 * @LastEditors: WWW
 * @LastEditTime: 2024-02-20 11:19:52
 * @FilePath: \ConvertTool\src-tauri\src\task.rs
 */
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use futures::stream::{FuturesUnordered, StreamExt};

pub(crate) async fn producer(tx: tokio::sync::mpsc::Sender<u32>) {
    for i in 1..=10 {
        println!("Adding task {}", i);
        if tx.send(i).await.is_err() {
            println!("Receiver dropped");
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}

pub(crate) async fn consumer(mut rx: tokio::sync::mpsc::Receiver<u32>) {
    let mut tasks = futures::stream::FuturesUnordered::new();

    while let Some(task) = rx.recv().await {
        println!("Received task {}", task);
        tasks.push(process_task(task));

        while tasks.len() >= 3 {
            tasks.select_next_some().await;
        }
    }

    // 确保处理完所有剩余的任务
    while let Some(_) = tasks.next().await {}
}

async fn process_task(task_id: u32) {
    println!("Processing task {}", task_id);
    // 模拟异步任务处理时间
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::channel(32);

    let producer_task = tokio::spawn(producer(tx));
    let consumer_task = tokio::spawn(consumer(rx));

    // 等待生产者和消费者任务完成
    if let Err(e) = producer_task.await {
        println!("Producer task failed: {}", e);
    }

    if let Err(e) = consumer_task.await {
        println!("Consumer task failed: {}", e);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_producer_consumer() {
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        let producer_task = tokio::spawn(producer(tx));
        let consumer_task = tokio::spawn(consumer(rx));
        
        // 这里添加您的断言
        // 例如，检查是否所有任务都已处理
    }
}
