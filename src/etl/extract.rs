/// Extract data from feishu
/// Then output to local file

use super::feishu_config::Config;
use reqwest;
use serde_json;
use serde_json::json;
use serde_json::Value;
use std::error::Error;
use std::path;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use log::{info, warn, error};

pub async fn get_feishu_data() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let mut cfg = Config::new();

    get_tenant_access_token(&mut cfg).await?;
    create_export_task(&mut cfg).await?;
    poll_export_task_result(&mut cfg).await?;
    download_file(&mut cfg).await?;

    let mut path = std::env::current_dir()?;
    path.push(format!("{}.{}", cfg.file_name.as_ref().unwrap(), cfg.doc["file_extension"]));
    Ok(path)
}

async fn get_tenant_access_token(cfg: &mut Config) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let payload = json!({
        "app_id": &cfg.app_id,
        "app_secret": &cfg.app_secret,
    });

    let res = client
        .post(&cfg.tenant_access_token_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let text = res.text().await?;
    let parsed: Value = serde_json::from_str(&text)?;
    assert_eq!(parsed["code"], 0);
    cfg.tenant_access_token = Some(parsed["tenant_access_token"].as_str().unwrap().to_string());

    Ok(())
}

async fn create_export_task(cfg: &mut Config) -> Result<(), Box<dyn Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", cfg.tenant_access_token.as_ref().unwrap())
            .parse()
            .unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let payload = json!({
        "file_extension": "xlsx",
        "token": &cfg.doc["token"],
        "type": "bitable",
    });

    let client = reqwest::Client::new();
    let res = client
        .post(&cfg.create_export_task_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let text = res.text().await?;
    let parsed: Value = serde_json::from_str(&text)?;
    assert_eq!(parsed["code"], 0);
    cfg.export_task_ticket = Some(parsed["data"]["ticket"].as_str().unwrap().to_string());

    Ok(())
}

enum JobStatus {
    Ready = 0,
    Initialing = 1,
    Processing = 2,
    _OtherStatue = 3,
}

async fn poll_export_task_result(cfg: &mut Config) -> Result<(), Box<dyn Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", cfg.tenant_access_token.as_ref().unwrap())
            .parse()
            .unwrap(),
    );
    let url = cfg
        .query_task_result_url
        .replace(":ticket", &cfg.export_task_ticket.as_ref().unwrap());
    let params = [("token", &cfg.doc["token"])];
    dbg!(&url);

    let client = reqwest::Client::new();

    let mut parsed: Value;
    loop {
        let res = client
            .get(&url)
            .query(&params)
            .headers(headers.clone())
            .send()
            .await?;

        let text = res.text().await?;
        parsed = serde_json::from_str(&text)?;

        if JobStatus::Processing as i32 != parsed["data"]["result"]["job_status"]
            && JobStatus::Initialing as i32 != parsed["data"]["result"]["job_status"]
        {
            break;
        } else {
            println!("Task is not ready, wait 1s...");
            dbg!(&parsed);
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }

    if parsed["data"]["result"]["job_status"] != JobStatus::Ready as i32 {
        dbg!(&parsed);
        error!("Feishu export task is error");
        return Err("Job status error".into());
    }

    cfg.file_token = Some(
        parsed["data"]["result"]["file_token"]
            .as_str()
            .unwrap()
            .to_string(),
    );
    cfg.file_name = Some(
        parsed["data"]["result"]["file_name"]
            .as_str()
            .unwrap()
            .to_string(),
    );

    info!("Feishu export task is Ok");
    Ok(())
}

async fn download_file(cfg: &mut Config) -> Result<(), Box<dyn Error>> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", cfg.tenant_access_token.as_ref().unwrap())
            .parse()
            .unwrap(),
    );

    let url = cfg
        .get_exported_file_url
        .replace(":file_token", &cfg.file_token.as_ref().unwrap());

    let client = reqwest::Client::new();
    let res = client.get(url).headers(headers).send().await?;

    let bytes = res.bytes().await?;
    //dbg!(&text);

    let mut path = std::env::current_dir()?;
    path.push(format!("{}.{}", cfg.file_name.as_ref().unwrap(), cfg.doc["file_extension"]));
    let mut file = File::create(&path).await?;
    file.write_all(&bytes).await?;

    Ok(())
}