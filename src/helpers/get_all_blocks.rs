use lambda_runtime::Error;
use serde_json::{from_str, json, Value};

use reqwest::{header, Client};

use crate::models::block::BlockChildren;

pub async fn get_all_blocks(event: Value) -> Result<Value, Error> {
    let notion_api_key = event["NOTION_API_KEY"].as_str().unwrap_or("NO_TOKEN");

    let client = Client::new();

    let mut has_more = true;
    let mut next_cursor: Option<String> = None;

    let mut blocks = Vec::new();

    while has_more {
        let base_url = format!(
            "https://api.notion.com/v1/blocks/{}/children",
            "29af298cf2b74f9190c105439ecb5b25"
        );
        let page_size = 100;
        let url = match &next_cursor {
            Some(cursor) => format!(
                "{}?page_size={}&start_cursor={}",
                base_url, page_size, cursor
            ),
            None => format!("{}?page_size={}", base_url, page_size),
        };

        let response = client
            .get(&url)
            .header("Notion-Version", "2022-06-28")
            .header(header::AUTHORIZATION, format!("Bearer {}", notion_api_key))
            .send()
            .await?;

        let body = response.text().await?;

        let page: BlockChildren =
            from_str(&body).map_err(|e| lambda_runtime::Error::from(e.to_string()))?;

        for result in page.results.into_iter() {
            blocks.push(result);
        }

        has_more = page.has_more;
        next_cursor = page.next_cursor;
    }

    Ok(json!(blocks))
}
