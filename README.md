# notion-lambda-utils

- [aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime)
- [Cargo Lambda](https://www.cargo-lambda.info/)

## Deploying the Function

First, we build the package.

```bash
cargo lambda build --release
```

Next, we proceed with the deployment.

```bash
cargo lambda deploy
```

To delete the function, remove it from the AWS Management Console.

## Invoking the Function

Invoke the function using the following command:

```bash
cargo lambda invoke --remote \
  --data-ascii '{"command": "convert_page_to_html"}' \
  --output-format json \
  notion-lambda-utils
```

```bash
cargo lambda invoke --data-ascii '{ "command": "convert_page_to_html", "NOTION_API_KEY": "secret_****" }'
```

## Testing the Function Locally

You can also test your function locally. The `invoke` subcommand allows you to send JSON payloads to the function running locally. These payloads are deserialized into strongly typed Rust structs. The `invoke` call will fail if the payload does not match the expected structure.

For basic functions that only receive events with a `command` field in the payload, use the following command for local invocation:

```bash
cargo lambda start

cargo lambda invoke --data-ascii '{ "command": "convert_page_to_html", "NOTION_API_KEY": "secret_*****", "block_id": "*****" }'
cargo lambda invoke --data-ascii '{ "command": "convert_page_to_markdown", "NOTION_API_KEY": "secret_*****", "block_id": "*****" }'

NOTION_API_KEY="secret_*****"
BLOCK_ID="*****"
JSON_PAYLOAD=$(jq -n \
                  --arg cmd "convert_page_to_html" \
                  --arg key "$NOTION_API_KEY" \
                  --arg bid "$BLOCK_ID" \
                  '{command: $cmd, NOTION_API_KEY: $key, block_id: $bid}')

cargo lambda invoke --data-ascii "$JSON_PAYLOAD"
```
