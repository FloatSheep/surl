use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::io::stdin;
use tokio::fs as async_fs;

const CONFIG_FILE: &str = "config.json";

#[derive(Serialize, Deserialize)]
struct Config {
    encrypted_token: String,
    target: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // 获取当前可执行文件的完整路径
    let exe_path = env::current_exe()?;
    // 获取可执行文件所在目录的路径
    let dir_path = exe_path.parent().ok_or("无法获取可执行文件的目录")?;

    // 构造 config.json 文件的完整路径
    let config_file_path = dir_path.join(CONFIG_FILE);

    // 检查命令行参数的数量
    match args.len() {
        // 没有参数，默认执行 init_config
        1 => {
            init_config().await?;
        }
        // 有一个参数，认为它是一个网址，尝试添加 issue
        2 => {
            let url = &args[1];
            let config_content = async_fs::read_to_string(config_file_path).await?;
            let config: Config = serde_json::from_str(&config_content)?;
            let decrypted_token = decrypt_token(&config.encrypted_token)?;
            add_issue_to_repo(&config.target, &url, &decrypted_token).await?;
        }
        // 参数太多，返回错误
        _ => return Err("错误：参数使用不正确。".into()),
    }
    Ok(())
}

async fn init_config() -> Result<Config, Box<dyn std::error::Error>> {
    println!("请输入您的GitHub Token:");
    let mut token = String::new();
    stdin().read_line(&mut token)?;

    println!("请输入目标仓库（例如：FloatSheep/external-links-shortened-list）:");
    let mut target = String::new();
    stdin().read_line(&mut target)?;

    let encrypted_token = encrypt_token(&token.trim())?;
    let config = Config {
        encrypted_token,
        target: target.trim().into(),
    };

    async_fs::write(CONFIG_FILE, serde_json::to_string(&config)?).await?;
    Ok(config)
}

async fn add_issue_to_repo(
    repo: &str,
    url: &str,
    token: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let client = Client::new();
    let request_url = format!("https://api.github.com/repos/{}/issues", repo);

    let issue_title = json!({
        "title": url,
        "labels": ["Authentication"],
    });

    let response = client
        .post(&request_url)
        .bearer_auth(token)
        .header("User-Agent", "surl.app")
        .json(&issue_title)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    let issue_id = json
        .get("number")
        .ok_or("No id found")?
        .as_u64()
        .ok_or("Invalid id format")?;

    println!("Issue ID: {} 添加成功！", issue_id);
    Ok(issue_id)
}

fn encrypt_token(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted: String = token
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_ascii_lowercase() { 'a' } else { 'A' };
                let offset = (c as u8 - base as u8 + 3) % 26;
                (base as u8 + offset) as char
            } else {
                c
            }
        })
        .collect();
    Ok(encrypted)
}

fn decrypt_token(encrypted_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let decrypted: String = encrypted_token
        .chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_ascii_lowercase() { 'a' } else { 'A' };
                let offset = (c as u8 - base as u8 + 23) % 26; // 使用 +23 实现向后移3位
                (base as u8 + offset) as char
            } else {
                c
            }
        })
        .collect();
    Ok(decrypted)
}
