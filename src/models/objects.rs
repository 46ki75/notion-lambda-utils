use serde::{Deserialize, Serialize};

/// for more details - https://developers.notion.com/reference/user
#[derive(Deserialize, Serialize)]
pub struct User {
    object: String, // always "user"
    id: String,
    r#type: Option<String>,
    name: Option<String>,
    avatar_url: Option<String>,
}

/// for more details - https://developers.notion.com/reference/emoji-object
#[derive(Deserialize, Serialize)]
pub struct EmojiObject {
    /// The constant string "emoji" that represents the object type.
    r#type: String,

    /// The emoji character.
    emoji: String,
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
    r#type: String,
    database_id: Option<String>,
    page_id: Option<String>,
    workspace: Option<bool>,
    block_id: Option<String>,
}

/// ## File - Struct
/// for more details - https://developers.notion.com/reference/file-object
#[derive(Deserialize, Serialize)]
pub struct File {
    /// An authenticated S3 URL to the file.
    /// The URL is valid for **one hour**.
    /// If the link expires, then you can send an API request to get an updated URL.
    url: String,

    /// The date and time when the link expires,
    /// formatted as an ISO 8601 date time string.
    expiry_time: String,
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
    url: String,
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
    r#type: String,

    /// An object containing type-specific configuration. The key of the object is
    /// external for external files, and file for Notion-hosted files.
    /// Refer to the type sections below for details on type-specific values.
    external: Option<External>,
    file: Option<File>,
}
