# Auto Login
Rust写的自动登录校园网玩具程序，适用于Windows  

## 食用方法
进入校园网登录界面，F12打开控制台  
选择网络，勾上保留日志，点击登录
复制第一个的url到程序所在位置文件夹中的assets/url.txt下  
开机自启动请把快捷方式放在自启动文件夹中
Win+R，输入shell:common startup可以快速找到目录  
连接完成后右下角会有通知

### 依赖库
[tokio](https://github.com/tokio-rs/tokio)  
[reqwest](https://github.com/seanmonstar/reqwest)  
[winrt-notification](https://github.com/allenbenz/winrt-notification)  
[regex](https://github.com/rust-lang/regex)  