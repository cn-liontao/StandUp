use std::time::Duration;
use tauri::api::notification::Notification;
use tauri::async_runtime::spawn;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

async fn create_notification(identifier: String, title: String, body: String, timeout: u64, cancel_token: CancellationToken) {
    println!("start, {}", timeout);
    tokio::select! {
        _ = cancel_token.cancelled() => {},
        _ = sleep(Duration::from_secs(timeout)) => {
            println!("go");
           let notify = Notification::new(identifier).title(title).body(body);
            notify.show().unwrap();
        }
    }
}

pub fn schedule_notification(identifier: String, title: String, body: String, timeout: u64, cancel_token: CancellationToken) {
    spawn(create_notification(identifier, title, body, timeout, cancel_token));
}