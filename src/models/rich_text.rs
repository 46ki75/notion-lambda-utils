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
#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct RichText {
    pub rich_text: Vec<RichTextElement>,
    pub is_toggleable: Option<bool>,
    pub color: Color,
}

/// --------------------------------------------------------------------------------
/// ## Equation - Struct
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize)]
pub struct Equation {
    /// The LaTeX string representing the inline equation.
    pub expression: String,
}

/// --------------------------------------------------------------------------------
/// ## Mention - Struct
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Mention {
    Database { database: DatabaseMention },
    Date { date: DateMention },
    LinkPreview { link_preview: LinkPreviewMention },
    Page { page: PageMention },
    TemplateMention { template_mention: Value },
    User { user: UserMention },
}

#[derive(Deserialize, Serialize)]
pub struct DatabaseMention {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct DateMention {
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LinkPreviewMention {
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct PageMention {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserMention {
    pub id: String,
}

/// --------------------------------------------------------------------------------
/// ## Text - Struct
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize)]
pub struct Text {
    /// The actual text content of the text.
    pub content: String,

    /// An object with information about any inline link in this text, if included.
    /// If the text contains an inline link, then the object key is url and the value is the URL’s string web address.
    /// If the text doesn’t have any inline links, then the value is null.
    pub link: Option<Link>,
}

#[derive(Deserialize, Serialize)]
pub struct Link {
    /// The URL's string web address.
    pub url: String,
}

#[derive(Deserialize, Serialize)]
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
