use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::common::Color;

/// ## RichText - Struct
///
/// for more details - https://developers.notion.com/reference/rich-text
///
/// Notion uses rich text to allow users to customize their content.
/// Rich text refers to a type of document where content can be styled
/// and formatted in a variety of customizable ways. This includes styling decisions,
/// such as the use of italics, font size, and font color, as well as formatting,
/// such as the use of hyperlinks or code blocks.
///
/// Notion includes rich text objects in block objects to indicate how blocks
/// in a page are represented. Blocks that support rich text will include a rich text object;
/// however, not all block types offer rich text.
///
/// When blocks are retrieved from a page using the Retrieve a block or Retrieve block children endpoints,
/// an array of rich text objects will be included in the block object (when available).
///  Developers can use this array to retrieve the plain text (plain_text)
/// for the block or get all the rich text styling and formatting options applied to the block.
#[derive(Deserialize, Serialize, Debug)]
// The type is determined based on the value of the type field.
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichTextElement {
    Text {
        text: Text,
        annotations: Annotations,
        plain_text: String,
        href: Option<String>,
    },
    Mention {
        mention: Mention,
        annotations: Annotations,
        plain_text: String,
        href: Option<String>,
    },
    Equation {
        equation: Equation,
        annotations: Annotations,
        plain_text: String,
        href: Option<String>,
    },
}

impl RichTextElement {
    pub fn to_html(&self) -> String {
        let (plain_text, annotations, href) = match self {
            RichTextElement::Text {
                plain_text,
                annotations,
                href,
                ..
            }
            | RichTextElement::Mention {
                plain_text,
                annotations,
                href,
                ..
            }
            | RichTextElement::Equation {
                plain_text,
                annotations,
                href,
                ..
            } => (plain_text, annotations, href),
        };

        let color_class = match annotations.color {
            Color::Blue => "blue",
            Color::BlueBackground => "blue-background",
            Color::Brown => "brown",
            Color::BrownBackground => "brown-background",
            Color::Default => "default",
            Color::Gray => "gray",
            Color::GrayBackground => "gray-background",
            Color::Green => "green",
            Color::GreenBackground => "green-background",
            Color::Orange => "orange",
            Color::OrangeBackground => "orange-background",
            Color::Pink => "pink",
            Color::PinkBackground => "pink-background",
            Color::Purple => "purple",
            Color::PurpleBackground => "purple-background",
            Color::Red => "red",
            Color::RedBackground => "red-background",
            Color::Yellow => "yellow",
            Color::YellowBackground => "yellow-background",
        };

        let mut html = format!("<span class='notion-rich-text {}'>", color_class);

        let escaped_plain_text = plain_text
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;");

        match href {
            Some(link) => html.push_str(&format!("<a href='{}'>", link)),
            None => {}
        };
        if annotations.code {
            html.push_str("<code>");
        }
        if annotations.bold {
            html.push_str("<strong>");
        }
        if annotations.italic {
            html.push_str("<em>");
        }
        if annotations.strikethrough {
            html.push_str("<del>");
        }
        if annotations.underline {
            html.push_str("<ins>");
        }

        // insert text
        html.push_str(&escaped_plain_text);

        // end tag (in reverse order)
        if annotations.underline {
            html.push_str("</ins>");
        }
        if annotations.strikethrough {
            html.push_str("</del>");
        }
        if annotations.italic {
            html.push_str("</em>");
        }
        if annotations.bold {
            html.push_str("</strong>");
        }
        if annotations.code {
            html.push_str("</code>");
        }
        match href {
            Some(_) => html.push_str("</a>"),
            None => {}
        };

        html.push_str("</span>");

        html
    }

    pub fn to_markdown(&self) -> String {
        let (plain_text, annotations, href) = match self {
            RichTextElement::Text {
                plain_text,
                annotations,
                href,
                ..
            }
            | RichTextElement::Mention {
                plain_text,
                annotations,
                href,
                ..
            }
            | RichTextElement::Equation {
                plain_text,
                annotations,
                href,
                ..
            } => (plain_text, annotations, href),
        };

        let mut markdown = String::new();

        match href {
            Some(link) => {
                markdown.push_str(&format!("[{}]({})", plain_text, link));
            }
            None => {
                if annotations.code {
                    markdown.push_str(&format!("`{}`", plain_text));
                } else if annotations.bold {
                    markdown.push_str(&format!("**{}**", plain_text));
                } else if annotations.italic {
                    markdown.push_str(&format!("*{}*", plain_text));
                } else if annotations.strikethrough {
                    markdown.push_str(&format!("~{}~", plain_text));
                }
            }
        };

        markdown
    }

    pub fn to_plaintext(&self) -> String {
        match self {
            RichTextElement::Text { plain_text, .. }
            | RichTextElement::Mention { plain_text, .. }
            | RichTextElement::Equation { plain_text, .. } => plain_text.clone(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RichText {
    pub rich_text: Vec<RichTextElement>,
    pub is_toggleable: Option<bool>,
    pub color: Color,
}

impl RichText {
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        for rich_text_element in &self.rich_text {
            html.push_str(&rich_text_element.to_html());
        }
        html
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        for rich_text_element in &self.rich_text {
            markdown.push_str(&rich_text_element.to_markdown());
        }
        markdown
    }

    #[allow(dead_code)]
    pub fn to_plaintext(&self) -> String {
        let mut text = String::new();
        for rich_text_element in &self.rich_text {
            text.push_str(&rich_text_element.to_markdown());
        }
        text
    }
}

/// --------------------------------------------------------------------------------
/// ## Equation - Struct
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct Equation {
    /// The LaTeX string representing the inline equation.
    pub expression: String,
}

/// --------------------------------------------------------------------------------
/// ## Mention - Struct
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Mention {
    Database { database: DatabaseMention },
    Date { date: DateMention },
    LinkPreview { link_preview: LinkPreviewMention },
    Page { page: PageMention },
    TemplateMention { template_mention: Value },
    User { user: UserMention },
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseMention {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DateMention {
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LinkPreviewMention {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PageMention {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserMention {
    pub id: String,
}

/// --------------------------------------------------------------------------------
/// ## Text - Struct
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct Text {
    /// The actual text content of the text.
    pub content: String,

    /// An object with information about any inline link in this text, if included.
    /// If the text contains an inline link, then the object key is url and the value is the URL’s string web address.
    /// If the text doesn’t have any inline links, then the value is null.
    pub link: Option<Link>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Link {
    /// The URL's string web address.
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Annotations {
    /// Whether the text is bolded.
    pub bold: bool,

    /// Whether the text is italicized.
    pub italic: bool,

    /// Whether the text is struck through.
    pub strikethrough: bool,

    /// Whether the text is underlined.
    pub underline: bool,

    /// Whether the text is code style.
    pub code: bool,

    /// Color of the text. Possible values include:
    /// - `blue`
    /// - `blue_background`
    /// - `brown`
    /// - `brown_background`
    /// - `default`
    /// - `gray`
    /// - `gray_background`
    /// - `green`
    /// - `green_background`
    /// - `orange`
    /// - `orange_background`
    /// - `pink`
    /// - `pink_background`
    /// - `purple`
    /// - `purple_background`
    /// - `red`
    /// - `red_background`
    /// - `yellow`
    /// - `yellow_background`
    pub color: Color,
}
