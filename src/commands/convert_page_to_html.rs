use lambda_runtime::Error;
use serde_json::{json, Value};

pub async fn convert_page_to_html(event: Value) -> Result<Value, Error> {
    let token = event["NOTION_API_KEY"].as_str().unwrap_or("NO_TOKEN");
    Ok(json!({ "message": "convert_page_to_html", "token": token}))
}
