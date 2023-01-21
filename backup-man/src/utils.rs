pub async fn handle_retry(retry_count: u8) {
    if retry_count > 5 {
        tracing::error!("Retry count limit reached. Exiting.");
        panic!();
    }
    if retry_count > 0 {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

pub fn get_seconds(date_str: String) -> u64 {
    match date_str.as_str() {
        "hourly" => 3600,
        "daily" | _ => 86400,
    }
}
