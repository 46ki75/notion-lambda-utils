use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::objects::{EmojiObject, FileObject, ParentObject, User};

/// --------------------------------------------------------------------------------
/// ## Page - Struct
///
/// for more details - https://developers.notion.com/reference/user
///
/// The Page object contains the page property values of a single Notion page.
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize)]
pub struct Page {
    /// Always "page".
    object: String,

    /// Unique identifier of the page.
    id: String,

    /// Date and time when this page was created. Formatted as an ISO 8601 date time string.
    created_time: String,

    /// Date and time when this page was updated. Formatted as an ISO 8601 date time string.
    last_edited_time: String,

    /// User who created the page.
    created_by: User,

    /// User who last edited the page.
    last_edited_by: User,

    /// Page cover image.
    cover: Option<FileObject>,

    /// Page icon.
    icon: Option<EmojiObject>,

    /// Information about the page's parent. See Parent object.
    parent: ParentObject,

    /// The archived status of the page.
    archived: bool,

    /// Property values of this page.
    ///
    /// As of version 2022-06-28, properties only contains the ID of the property;
    /// in prior versions properties contained the values as well.
    ///
    /// If parent.type is "page_id" or "workspace", then the only valid key is title.
    ///
    /// If parent.type is "database_id", then the keys and values of this field are
    /// determined by the properties of the database this page belongs to.
    ///
    /// - `key string`: Name of a property as it appears in Notion.
    /// - `value object` See Property value object.
    properties: Value,

    /// The URL of the Notion page.
    url: String,

    /// The public page URL if the page has been published to the web. Otherwise, null.
    public_url: Option<String>,
}
