// using aws-lambda-rust-runtime
// @see https://github.com/awslabs/aws-lambda-rust-runtime

use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

mod commands;
mod helpers;
mod models;

use crate::commands::convert_page_to_html::convert_page_to_html_command;
use crate::commands::convert_page_to_markdown::convert_page_to_markdown_command;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handle_lambda_event = service_fn(handle_lambda_event);
    lambda_runtime::run(handle_lambda_event).await?;
    Ok(())
}

async fn handle_lambda_event(event: LambdaEvent<Value>) -> Result<String, Error> {
    let (event, _context) = event.into_parts();
    let command = event["command"]
        .as_str()
        .ok_or_else(|| Error::from("The command field is missing or not a string"))?;

    event["NOTION_API_KEY"]
        .as_str()
        .ok_or_else(|| Error::from("The NOTION_API_KEY field is missing or not a string"))?;

    match command {
        "convert_page_to_html" => convert_page_to_html_command(event).await,
        "convert_page_to_markdown" => convert_page_to_markdown_command(event).await,
        _ => Err(Error::from(format!("Unknown command: {}", command))),
    }
}
