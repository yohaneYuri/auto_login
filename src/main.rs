use std::time;

use regex::Regex;
use reqwest::Client;
use tokio::fs;
use winrt_notification::{Sound, Toast};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = time::Instant::now();
    let client = Client::new();
    let url = fs::read_to_string("./assets/url.txt").await?;
    let regex = Regex::new(r#""result":\s*([01])"#).unwrap();
    let mut count: i64 = 0;
    
    loop {
        let content = client.get(url.clone())
            .send()
            .await?
            .text()
            .await?;
        let result = regex.captures(&content)
            .map(|cap| cap.get(1))
            .flatten()
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap();

        match result {
            1 => {
                let end = time::Instant::now();
                Toast::new(Toast::POWERSHELL_APP_ID)
                    .title("通知")
                    .text1("校园网连接成功")
                    .text2(format!("用时：{:?}", end - start).as_str())
                    .sound(Some(Sound::SMS))
                    .duration(winrt_notification::Duration::Short)
                    .show()
                    .unwrap();
                break;
            },
            0 | _  => {
                if count == 2 {
                    Toast::new(Toast::POWERSHELL_APP_ID)
                        .title("通知")
                        .text1("校园网连接失败")
                        .sound(Some(Sound::SMS))
                        .duration(winrt_notification::Duration::Short)
                        .show()
                        .unwrap();
                }

                count += 1;
            }
        };
    };
    Ok(())
}
