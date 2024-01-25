use async_recursion::async_recursion;
use lambda_runtime::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::helpers::get_all_blocks::get_all_blocks;

use crate::models::block::Block;

use crate::models::objects::FileObject;

#[derive(Deserialize, Serialize, Debug)]
pub struct HTMLResponse {
    pub html: String,
}

pub async fn convert_page_to_html_command(
    notion_api_key: &str,
    block_id: &str,
) -> Result<Value, Error> {
    let html = convert_page_to_html(notion_api_key, block_id).await;
    let response = HTMLResponse {
        html: html.unwrap(),
    };
    Ok(json!(response))
}

#[async_recursion]
pub async fn convert_page_to_html(notion_api_key: &str, block_id: &str) -> Result<String, Error> {
    let mut html: Vec<String> = Vec::new();
    let blocks = get_all_blocks(notion_api_key, block_id).await.unwrap();

    for block in &blocks {
        match block {
            Block::Bookmark(bookmark_block) => {
                html.push(format!(
                    "<a href='{}' class='notion-bookmark'></a>",
                    bookmark_block.bookmark.url
                ));
            }

            Block::Breadcrumb(_) => {
                println!("Breadcrumb is unsupported!");
            }

            Block::BulletedListItem(bulleted_list_item_block) => {
                html.push(format!(
                    "<li class='notion-bulleted-list-item'>{}</li>",
                    bulleted_list_item_block.bulleted_list_item.to_html()
                ));
            }

            Block::Callout(callout_block) => {
                let mut inner_html = String::new();
                for rich_text in &callout_block.callout.rich_text {
                    inner_html.push_str(&rich_text.to_html())
                }
                html.push(format!("<div class='notion-callout'>{}</div>", inner_html));
            }

            Block::ChildDatabase(_) => {
                println!("ChildDatabase is unsupported!");
            }

            Block::ChildPage(_) => {
                println!("ChildPage is unsupported!");
            }

            Block::Code(code_block) => {
                html.push(format!(
                    "<div class='notion-code'><pre class='{}'><code class='language-{}'></code></pre></div>",
                    code_block.code.language.to_class_name(),
                    code_block.code.language.to_class_name(),
                ));
            }

            Block::Column(column_block) => {
                html.push(String::from("<div class='notion-column'>"));
                let child_html =
                    convert_page_to_html(notion_api_key, &column_block.base.id).await?;
                html.push(child_html);
                html.push(String::from("</div>"));
            }

            Block::ColumnList(colmn_list_block) => {
                html.push(String::from("<div class='notion-column-list'>"));
                let child_html =
                    convert_page_to_html(notion_api_key, &colmn_list_block.base.id).await?;
                html.push(child_html);
                html.push(String::from("</div>"));
            }

            Block::Divider(_) => {
                html.push(format!("<hr class='notion-divider' />"));
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
                html.push(format!(
                    "<h1 class='notion-heading-1'>{}</h1>",
                    heading_1.heading_1.to_html()
                ));
            }

            Block::Heading2(heading_2) => {
                html.push(format!(
                    "<h2 class='notion-heading-2'>{}</h2>",
                    heading_2.heading_2.to_html()
                ));
            }

            Block::Heading3(heading_3) => {
                html.push(format!(
                    "<h3 class='notion-heading-3'>{}</h3>",
                    heading_3.heading_3.to_html()
                ));
            }

            Block::Image(image_block) => match &image_block.image {
                FileObject::File { file } => {
                    html.push(format!(
                        "<img src='{}' alt='' class='notion-image' />",
                        file.url
                    ));
                }

                FileObject::External { external } => {
                    html.push(format!(
                        "<img src='{}' alt='' class='notion-image' />",
                        external.url
                    ));
                }
            },

            Block::LinkPreview(_) => {
                println!("LinkPreview is unsupported!");
            }

            Block::NumberedListItem(bulleted_list_item_block) => {
                html.push(format!(
                    "<li class='notion-numbered-list-item'>{}</li>",
                    bulleted_list_item_block.numbered_list_item.to_html()
                ));
            }

            Block::Mention(_) => {
                println!("Mention is unsupported!");
            }

            Block::Paragraph(paragraph_block) => {
                html.push(format!(
                    "<p class='notion-paragraph'>{}</p>",
                    paragraph_block.paragraph.to_html()
                ));
            }

            Block::Pdf(_) => {
                println!("Pdf is unsupported!");
            }

            Block::Quote(quote_block) => {
                html.push(format!(
                    "<blockquote class='notion-quote'>{}</blockquote>",
                    quote_block.quote.to_html()
                ));
            }

            Block::SyncedBlock(synced_block) => {
                let child_html =
                    convert_page_to_html(notion_api_key, &synced_block.base.id).await?;
                html.push(child_html);
            }

            Block::Table(table_block) => {
                html.push(String::from("<table class='notion-table'>"));
                let child_html = convert_page_to_html(notion_api_key, &table_block.base.id).await?;
                html.push(child_html);
                html.push(String::from("</tbody>"));
                html.push(String::from("</table>"));
            }

            Block::TableOfContents(_) => {
                println!("TableOfContents is unsupported!");
            }

            Block::TableRow(table_row_block) => {
                let mut is_head = false;
                if html.last() == None {
                    is_head = true;
                    html.push(String::from("<thead>"));
                }
                html.push(String::from("<tr>"));
                for cell in &table_row_block.table_row.cells {
                    for rich_text in cell {
                        if is_head {
                            html.push(String::from("<th>"));
                        } else {
                            html.push(String::from("<td>"));
                        }
                        html.push(format!("{}", rich_text.to_html()));
                        if is_head {
                            html.push(String::from("</th>"));
                        } else {
                            html.push(String::from("</td>"));
                        }
                    }
                }
                html.push(String::from("</tr>"));
                if is_head {
                    html.push(String::from("</thead>"));
                    html.push(String::from("<tbody>"));
                }
            }

            Block::Template(_) => {
                println!("Template is unsupported!");
            }

            Block::ToDo(todo_block) => {
                html.push(format!(
                    "<div class='notion-todo'>{}</div>",
                    todo_block.to_do.to_html()
                ));
            }

            Block::Toggle(toggle_block) => {
                html.push(String::from("<details class='notion-toggle-block'>"));
                for rich_text in &toggle_block.toggle.rich_text {
                    html.push(String::from("<summary class='notion-toggle-block-header'>"));
                    html.push(rich_text.to_html());
                    html.push(String::from("</summary>"));
                }
                let child_html =
                    convert_page_to_html(notion_api_key, &toggle_block.base.id).await?;
                html.push(child_html);
                html.push(String::from("</details>"));
            }

            Block::Unsupported(_) => {
                println!("This kind of Block is unsupported!");
            }

            _ => {
                println!("Not Define!!!");
            }
        }
    }

    // Ok(html.join(""))
    Ok(wrap_list_items(html))
}

fn wrap_list_items(html_lines: Vec<String>) -> String {
    let mut result = String::new();
    let mut in_bulleted_list = false;
    let mut in_numbered_list = false;

    for line in &html_lines {
        if line.contains("class='notion-bulleted-list-item'") {
            if !in_bulleted_list {
                if in_numbered_list {
                    result.push_str("</ol>");
                    in_numbered_list = false;
                }
                result.push_str("<ul class='notion-bulleted-list'>");
                in_bulleted_list = true;
            }
            result.push_str(line);
        } else if line.contains("class='notion-numbered-list-item'") {
            if !in_numbered_list {
                if in_bulleted_list {
                    result.push_str("</ul>");
                    in_bulleted_list = false;
                }
                result.push_str("<ol class='notion-numbered-list'>");
                in_numbered_list = true;
            }
            result.push_str(line);
        } else {
            if in_bulleted_list {
                result.push_str("</ul>");
                in_bulleted_list = false;
            }
            if in_numbered_list {
                result.push_str("</ol>");
                in_numbered_list = false;
            }
            result.push_str(line);
        }
    }

    if in_bulleted_list {
        result.push_str("</ul>");
    }
    if in_numbered_list {
        result.push_str("</ol>");
    }

    result
}
