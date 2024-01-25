use async_recursion::async_recursion;
use lambda_runtime::Error;
use serde_json::Value;

use crate::helpers::get_all_blocks::get_all_blocks;

use crate::models::block::Block;

use crate::models::objects::FileObject;

use crate::helpers::fetch_title::fetch_title;

pub async fn convert_page_to_markdown_command(event: Value) -> Result<String, Error> {
    let notion_api_key = event["NOTION_API_KEY"]
        .as_str()
        .ok_or_else(|| Error::from("The NOTION_API_KEY field is missing or not a string"))?;

    let block_id = event["block_id"]
        .as_str()
        .ok_or_else(|| Error::from("The NOTION_API_KEY field is missing or not a string"))?;

    let html = convert_page_to_markdown(notion_api_key, block_id).await;
    match html {
        Ok(html) => Ok(html),
        Err(e) => Err(Error::from(e)),
    }
}

#[async_recursion]
pub async fn convert_page_to_markdown(
    notion_api_key: &str,
    block_id: &str,
) -> Result<String, Error> {
    let mut markdown: String = String::new();
    let blocks = get_all_blocks(notion_api_key, block_id).await.unwrap();

    for block in &blocks {
        match block {
            Block::Bookmark(bookmark_block) => {
                let title = fetch_title(&bookmark_block.bookmark.url).await.unwrap();
                markdown.push_str(&format!("\n[{}]({})\n", title, bookmark_block.bookmark.url));
            }

            Block::Breadcrumb(_) => {
                println!("Breadcrumb is unsupported!");
            }

            Block::BulletedListItem(bulleted_list_item_block) => {
                markdown.push_str(&format!(
                    "- {}\n",
                    bulleted_list_item_block.bulleted_list_item.to_markdown()
                ));
            }

            Block::Callout(_) => {
                println!("Callout is unsupported!");
            }

            Block::ChildDatabase(_) => {
                println!("ChildDatabase is unsupported!");
            }

            Block::ChildPage(_) => {
                println!("ChildPage is unsupported!");
            }

            Block::Code(code_block) => {
                let mut code_text = String::new();
                for rich_text in &code_block.code.rich_text {
                    code_text.push_str(&rich_text.to_plaintext());
                }
                markdown.push_str(&format!(
                    "\n```{}\n{}\n```\n",
                    code_text,
                    code_block.code.language.to_class_name(),
                ));
            }

            Block::Column(column_block) => {
                let child_markdown =
                    convert_page_to_markdown(notion_api_key, &column_block.base.id).await?;
                markdown.push_str(&format!("\n{}\n\n", child_markdown));
            }

            Block::ColumnList(colmn_list_block) => {
                let child_markdown =
                    convert_page_to_markdown(notion_api_key, &colmn_list_block.base.id).await?;
                markdown.push_str(&format!("\n{}\n\n", child_markdown));
            }

            Block::Divider(_) => {
                markdown.push_str(&format!("\n\n---\n\n"));
            }

            Block::Embed(_) => {
                println!("Embed is unsupported!");
            }

            Block::Equation(_) => {
                println!("Equation is unsupported!");
            }

            Block::File(_) => {
                println!("File is unsupported!");
            }

            Block::Heading1(heading_1) => {
                markdown.push_str(&format!("\n# {}\n\n", heading_1.heading_1.to_markdown()));
            }

            Block::Heading2(heading_2) => {
                markdown.push_str(&format!("\n## {}\n\n", heading_2.heading_2.to_markdown()));
            }

            Block::Heading3(heading_3) => {
                markdown.push_str(&format!("\n### {}\n\n", heading_3.heading_3.to_markdown()));
            }

            Block::Image(image_block) => match &image_block.image {
                FileObject::File { file } => {
                    markdown.push_str(&format!("\n![alt]({})\n\n", file.url));
                }

                FileObject::External { external } => {
                    markdown.push_str(&format!("\n![alt]({})\n\n", external.url));
                }
            },

            Block::LinkPreview(_) => {
                println!("LinkPreview is unsupported!");
            }

            Block::NumberedListItem(numberted_list_item_block) => {
                markdown.push_str(&format!(
                    "- {}\n",
                    numberted_list_item_block.numbered_list_item.to_markdown()
                ));
            }

            Block::Mention(_) => {
                println!("Mention is unsupported!");
            }

            Block::Paragraph(paragraph_block) => {
                markdown.push_str(&format!(
                    "\n{}\n\n",
                    paragraph_block.paragraph.to_markdown()
                ));
            }

            Block::Pdf(_) => {
                println!("Pdf is unsupported!");
            }

            Block::Quote(quote_block) => {
                markdown.push_str(&format!("\n> {}\n\n", quote_block.quote.to_markdown()));
            }

            Block::SyncedBlock(synced_block) => {
                let child_markdown =
                    convert_page_to_markdown(notion_api_key, &synced_block.base.id).await?;
                markdown.push_str(&child_markdown);
            }

            Block::Table(table_block) => {
                markdown.push_str("|");
                let child_markdown =
                    convert_page_to_markdown(notion_api_key, &table_block.base.id).await?;
                markdown.push_str(&child_markdown);
                markdown.push_str("\n");
            }

            Block::TableOfContents(_) => {
                println!("TableOfContents is unsupported!");
            }

            Block::TableRow(table_row_block) => {
                let mut is_head = false;
                if markdown.ends_with("\n|") {
                    is_head = true;
                    markdown.push_str("\n|");
                }
                for cell in &table_row_block.table_row.cells {
                    for rich_text in cell {
                        markdown.push_str(&format!(" {} |", rich_text.to_markdown()));
                    }
                }
                markdown.push_str("\n");
                if is_head {
                    markdown.push_str("|");
                    for _ in &table_row_block.table_row.cells {
                        markdown.push_str(" --- |");
                    }
                    markdown.push_str("\n");
                }
            }

            Block::Template(_) => {
                println!("Template is unsupported!");
            }

            Block::ToDo(todo_block) => {
                markdown.push_str(&format!("- [] {}\n", todo_block.to_do.to_markdown()));
            }

            Block::Toggle(_) => {
                println!("Toggle is unsupported!");
            }

            Block::Unsupported(_) => {
                println!("This kind of Block is unsupported!");
            }

            _ => {
                println!("Not Define!!!");
            }
        }
    }

    Ok(normalize_newlines(&markdown))
}

fn normalize_newlines(input: &str) -> String {
    let mut result = String::new();
    let mut newline_count = 0;
    let mut chars = input.chars().peekable();

    while chars.peek() == Some(&'\n') {
        chars.next();
    }

    for c in chars {
        if c == '\n' {
            newline_count += 1;
        } else {
            if newline_count >= 3 {
                result.push_str("\n\n");
            } else if newline_count > 0 {
                result.extend(std::iter::repeat('\n').take(newline_count));
            }
            newline_count = 0;
            result.push(c);
        }
    }

    if newline_count > 0 {
        result.push('\n');
    }

    result
}
