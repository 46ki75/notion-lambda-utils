use serde::{Deserialize, Serialize};

/// for more details - https://developers.notion.com/reference/user
#[derive(Deserialize, Serialize)]
pub struct User {
    pub object: String, // always "user"
    pub id: String,
    pub r#type: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

/// for more details - https://developers.notion.com/reference/emoji-object
#[derive(Deserialize, Serialize)]
pub struct EmojiObject {
    /// The constant string "emoji" that represents the object type.
    pub r#type: String,

    /// The emoji character.
    pub emoji: String,
}

/// ## ParentObject - Struct
///
/// for more details - https://developers.notion.com/reference/parent-object
///
/// Pages, databases, and blocks are either located inside other pages, databases, and blocks,
/// or are located at the top level of a workspace. This location is known as the "parent".
/// Parent information is represented by a consistent parent object throughout the API.
#[derive(Deserialize, Serialize)]
pub struct ParentObject {
    pub r#type: String,
    pub database_id: Option<String>,
    pub page_id: Option<String>,
    pub workspace: Option<bool>,
    pub block_id: Option<String>,
}

/// ## File - Struct
/// for more details - https://developers.notion.com/reference/file-object
#[derive(Deserialize, Serialize)]
pub struct File {
    /// An authenticated S3 URL to the file.
    /// The URL is valid for **one hour**.
    /// If the link expires, then you can send an API request to get an updated URL.
    pub url: String,

    /// The date and time when the link expires,
    /// formatted as an ISO 8601 date time string.
    pub expiry_time: String,
}

/// ## External - Struct
///
/// for more details - https://developers.notion.com/reference/file-object
///
/// An external file is any URL linked to in Notion that isn’t hosted by Notion.
/// All external files have a type of "external".
#[derive(Deserialize, Serialize)]
pub struct External {
    /// A link to the externally hosted content.
    pub url: String,
}

/// ## FileObject - Struct
///
/// for more details - https://developers.notion.com/reference/file-object
///
/// File objects contain data about a file that is uploaded to Notion,
/// or data about an external file that is linked to in Notion.
#[derive(Deserialize, Serialize)]
pub struct FileObject {
    /// The type of the file object. Possible type values are: "external", "file".
    pub r#type: String,

    /// An object containing type-specific configuration. The key of the object is
    /// external for external files, and file for Notion-hosted files.
    /// Refer to the type sections below for details on type-specific values.
    pub external: Option<External>,
    pub file: Option<File>,
}
