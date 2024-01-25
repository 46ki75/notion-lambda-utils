// using aws-lambda-rust-runtime
// @see https://github.com/awslabs/aws-lambda-rust-runtime

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

mod commands;
use crate::commands::convert_page_to_html::convert_page_to_html_command;
use crate::commands::convert_page_to_markdown::convert_page_to_markdown;

mod helpers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handle_lambda_event = service_fn(handle_lambda_event);
    lambda_runtime::run(handle_lambda_event).await?;
    Ok(())
}

async fn handle_lambda_event(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let command = event["command"]
        .as_str()
        .ok_or_else(|| Error::from("The command field is missing or not a string"))?;

    let notion_api_key = event["NOTION_API_KEY"]
        .as_str()
        .ok_or_else(|| Error::from("The NOTION_API_KEY field is missing or not a string"))?;

    let block_id = "29af298cf2b74f9190c105439ecb5b25";

    // invoke the corresponding function according to the command value
    match command {
        "convert_page_to_html" => convert_page_to_html_command(notion_api_key, block_id).await,
        "convert_page_to_markdown" => convert_page_to_markdown(event).await,
        _ => Err(Error::from(format!("Unknown command: {}", command))),
    }
}
