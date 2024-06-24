#![windows_subsystem = "windows"]
use std::time;

use regex::Regex;
use reqwest::Client;
use tokio::fs;
use winrt_notification::{Sound, Toast};

fn send_notification(title: &str, text1: &str, text2: &str) {
    Toast::new(Toast::POWERSHELL_APP_ID)
                    .title(title)
                    .text1(text1)
                    .text2(text2)
                    .sound(Some(Sound::SMS))
                    .duration(winrt_notification::Duration::Short)
                    .show()
                    .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prog_start = time::Instant::now();
    let client = Client::builder()
        .timeout(time::Duration::from_secs(3))
        .build()?;
    let url = fs::read_to_string("./assets/url.txt").await?;
    let regex = Regex::new(r#""result":\s*([01])"#).unwrap();
    let mut count: i64 = 0;

    loop {
        count += 1;
        let response = client.get(url.clone())
            .send()
            .await;
        if response.is_err() {
            continue;
        }
        let content = response.unwrap().text().await?;
        let result = regex.captures(&content)
            .and_then(|cap| cap.get(1))
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();

        if result == 1 {
            let prog_end = time::Instant::now();
                send_notification(
                    "通知",
                    "校园网连接成功",
                    format!("尝试{}次，用时：{:?}", count, prog_end - prog_start).as_str()
                );
                break;
        } else if result == 0 {
            send_notification(
                "通知",
                "校园网连接失败",
                ""
            );
        }
    };
    Ok(())
}
