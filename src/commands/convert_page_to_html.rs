use lambda_runtime::Error;
use serde_json::Value;

use crate::helpers::get_all_blocks::get_all_blocks;

pub async fn convert_page_to_html(event: Value) -> Result<Value, Error> {
    get_all_blocks(event).await
}
