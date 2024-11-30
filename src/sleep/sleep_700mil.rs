use std::time::Duration;
use async_std::task;

pub async fn sleep_700mil() {
    task::sleep(Duration::from_millis(700)).await;
}