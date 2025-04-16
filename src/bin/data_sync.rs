use std::error::Error;
use arch_map::etl::extract;
use arch_map::etl::transform_load::InternalData;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use std::path::Path;
use std::process::Command;

async fn data_sync_task() -> Result<(), Box<dyn Error>> {
    let path = extract::get_feishu_data().await?;
    // call python script to handle hyper link
    Command::new("python3").args(&["src/bin/handle_hyperlink.py", path.as_path().to_str().unwrap()]).output()?;
    let mut data = InternalData::new();
    data.import_and_load(&path).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // call data_sync_task every 5 minutes
    // let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5 * 60));
    // loop {
        // interval.tick().await;
    data_sync_task().await?;
    // }
    Ok(())
}