use lambda_runtime::Error;
use serde_json::{from_str, json, Value};

use reqwest::{header, Client};

use crate::models::block::BlockChildren;

pub async fn convert_page_to_html(event: Value) -> Result<Value, Error> {
    let notion_api_key = event["NOTION_API_KEY"].as_str().unwrap_or("NO_TOKEN");

    let client = Client::new();

    let response = client
        .get(format!(
            "https://api.notion.com/v1/blocks/{}/children?page_size=100",
            "",
        ))
        .header("Notion-Version", "2022-06-28")
        .header(header::AUTHORIZATION, format!("Bearer {}", notion_api_key))
        .send()
        .await?;

    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body: {}", body);
    let page: BlockChildren =
        from_str(&body).map_err(|e| lambda_runtime::Error::from(e.to_string()))?;

    // println!("Body: {}", page);

    Ok(json!(page))
}
