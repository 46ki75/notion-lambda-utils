use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::common::Color;

use crate::models::objects::{ParentObject, User};

use crate::models::objects::{EmojiObject, FileObject};

use crate::models::rich_text::{RichText, RichTextElement};

/// ## Block - Struct
///
/// for more details - https://developers.notion.com/reference/block
///
/// A block object represents a piece of content within Notion.
/// The API translates the headings, toggles, paragraphs, lists, media,
/// and more that you can interact with in the Notion UI as different block type objects.
#[derive(Deserialize, Serialize, Debug)]
pub struct BaseBlock {
    pub object: String,
    pub id: String,
    pub parent: ParentObject,
    pub created_time: String,
    pub last_edited_time: String,
    pub created_by: User,
    pub last_edited_by: User,
    pub has_children: bool,
    pub archived: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockChildren {
    // always "list"
    pub object: String,
    pub results: Vec<Block>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    // always "block"
    pub r#type: String,
    pub block: Value,
    pub request_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Bookmark(BookmarkBlock),
    Breadcrumb(BreadcrumbBlock),
    BulletedListItem(BulletedListItemBlock),
    Callout(CalloutBlock),
    ChildDatabase(ChildDatabaseBlock),
    ChildPage(ChildPageBlock),
    Code(CodeBlock),
    Column(ColumnBlock),
    ColumnList(ColumnListBlock),
    Divider(DividerBlock),
    Embed(EmbedBlock),
    Equation(EquationBlock),
    File(FileBlock),
    #[serde(rename = "heading_1")]
    Heading1(Heading1Block),
    #[serde(rename = "heading_2")]
    Heading2(Heading2Block),
    #[serde(rename = "heading_3")]
    Heading3(Heading3Block),
    Image(ImageBlock),
    LinkPreview(LinkPreviewBlock),
    // LinkToPage -> Unsupported,
    NumberedListItem(NumberedListItemBlock),
    Mention(MentionBlock),
    Paragraph(ParagraphBlock),
    Pdf(PdfBlock),
    Quote(QuoteBlock),
    SyncedBlock(SyncedBlock),
    Table(TableBlock),
    TableOfContents(TableOfContentsBlock),
    TableRow(TableRowBlock),
    Template(TemplateBlock),
    ToDo(ToDoBlock),
    Toggle(ToggleBlock),
    Unsupported(UnsupportedBlock),
    Video(VideoBlock),
}

/// --------------------------------------------------------------------------------
/// Bookmark
/// https://developers.notion.com/reference/block#bookmark
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct BookmarkBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub bookmark: BookmarkField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BookmarkField {
    pub caption: Vec<RichTextElement>,
    pub url: String,
}

/// --------------------------------------------------------------------------------
/// Breadcrumb
/// https://developers.notion.com/reference/block#breadcrumb
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct BreadcrumbBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub breadcrumb: Value,
}

/// --------------------------------------------------------------------------------
/// Bulleted list item
/// https://developers.notion.com/reference/block#bulleted-list-item
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct BulletedListItemBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub bulleted_list_item: RichText,
}

/// --------------------------------------------------------------------------------
/// Callout
/// https://developers.notion.com/reference/block#callout
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct CalloutBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub callout: CalloutField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CalloutField {
    pub rich_text: Vec<RichTextElement>,
    pub icon: EmojiObject,
    pub color: Color,
}

/// --------------------------------------------------------------------------------
/// Child database
/// https://developers.notion.com/reference/block#child-database
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ChildDatabaseBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub title: String,
}

/// --------------------------------------------------------------------------------
/// Child page
/// https://developers.notion.com/reference/block#child-page
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ChildPageBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub title: String,
}

/// --------------------------------------------------------------------------------
/// Code
/// https://developers.notion.com/reference/block#code
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct CodeBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub code: CodeField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CodeField {
    pub caption: Vec<RichTextElement>,
    pub rich_text: Vec<RichTextElement>,
    pub language: ProgrammingLanguage,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgrammingLanguage {
    #[serde(rename = "abap")]
    Abap,
    #[serde(rename = "arduino")]
    Arduino,
    #[serde(rename = "bash")]
    Bash,
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "clojure")]
    Clojure,
    #[serde(rename = "coffeescript")]
    Coffeescript,
    #[serde(rename = "c++")]
    Cpp,
    #[serde(rename = "c#")]
    CSharp,
    #[serde(rename = "css")]
    Css,
    #[serde(rename = "dart")]
    Dart,
    #[serde(rename = "diff")]
    Diff,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "elixir")]
    Elixir,
    #[serde(rename = "elm")]
    Elm,
    #[serde(rename = "erlang")]
    Erlang,
    #[serde(rename = "flow")]
    Flow,
    #[serde(rename = "fortran")]
    Fortran,
    #[serde(rename = "f#")]
    FSharp,
    #[serde(rename = "gherkin")]
    Gherkin,
    #[serde(rename = "glsl")]
    Glsl,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "graphql")]
    Graphql,
    #[serde(rename = "groovy")]
    Groovy,
    #[serde(rename = "haskell")]
    Haskell,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "javascript")]
    Javascript,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "julia")]
    Julia,
    #[serde(rename = "kotlin")]
    Kotlin,
    #[serde(rename = "latex")]
    Latex,
    #[serde(rename = "less")]
    Less,
    #[serde(rename = "lisp")]
    Lisp,
    #[serde(rename = "livescript")]
    Livescript,
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "makefile")]
    Makefile,
    #[serde(rename = "markdown")]
    Markdown,
    #[serde(rename = "markup")]
    Markup,
    #[serde(rename = "matlab")]
    Matlab,
    #[serde(rename = "mermaid")]
    Mermaid,
    #[serde(rename = "nix")]
    Nix,
    #[serde(rename = "objective-c")]
    ObjectiveC,
    #[serde(rename = "ocaml")]
    Ocaml,
    #[serde(rename = "pascal")]
    Pascal,
    #[serde(rename = "perl")]
    Perl,
    #[serde(rename = "php")]
    Php,
    #[serde(rename = "plain text")]
    PlainText,
    #[serde(rename = "powershell")]
    Powershell,
    #[serde(rename = "prolog")]
    Prolog,
    #[serde(rename = "protobuf")]
    Protobuf,
    #[serde(rename = "python")]
    Python,
    #[serde(rename = "r")]
    R,
    #[serde(rename = "reason")]
    Reason,
    #[serde(rename = "ruby")]
    Ruby,
    #[serde(rename = "rust")]
    RustLang,
    #[serde(rename = "sass")]
    Sass,
    #[serde(rename = "scala")]
    Scala,
    #[serde(rename = "scheme")]
    Scheme,
    #[serde(rename = "scss")]
    Scss,
    #[serde(rename = "shell")]
    Shell,
    #[serde(rename = "sql")]
    Sql,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "typescript")]
    Typescript,
    #[serde(rename = "vb.net")]
    VbNet,
    #[serde(rename = "verilog")]
    Verilog,
    #[serde(rename = "vhdl")]
    Vhdl,
    #[serde(rename = "visual basic")]
    VisualBasic,
    #[serde(rename = "webassembly")]
    WebAssembly,
    #[serde(rename = "xml")]
    Xml,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "java/c/c++/c#")]
    JavaCCppCSharp,
}

impl ProgrammingLanguage {
    pub fn to_class_name(&self) -> &str {
        match self {
            ProgrammingLanguage::Abap => "abap",
            ProgrammingLanguage::Arduino => "arduino",
            ProgrammingLanguage::Bash => "bash",
            ProgrammingLanguage::Basic => "basic",
            ProgrammingLanguage::C => "c",
            ProgrammingLanguage::Clojure => "clojure",
            ProgrammingLanguage::Coffeescript => "coffeescript",
            ProgrammingLanguage::Cpp => "cpp",
            ProgrammingLanguage::CSharp => "csharp",
            ProgrammingLanguage::Css => "css",
            ProgrammingLanguage::Dart => "dart",
            ProgrammingLanguage::Diff => "diff",
            ProgrammingLanguage::Docker => "docker",
            ProgrammingLanguage::Elixir => "elixir",
            ProgrammingLanguage::Elm => "elm",
            ProgrammingLanguage::Erlang => "erlang",
            ProgrammingLanguage::Flow => "flow",
            ProgrammingLanguage::Fortran => "fortran",
            ProgrammingLanguage::FSharp => "fsharp",
            ProgrammingLanguage::Gherkin => "gherkin",
            ProgrammingLanguage::Glsl => "glsl",
            ProgrammingLanguage::Go => "go",
            ProgrammingLanguage::Graphql => "graphql",
            ProgrammingLanguage::Groovy => "groovy",
            ProgrammingLanguage::Haskell => "haskell",
            ProgrammingLanguage::Html => "html",
            ProgrammingLanguage::Java => "java",
            ProgrammingLanguage::Javascript => "javascript",
            ProgrammingLanguage::Json => "json",
            ProgrammingLanguage::Julia => "julia",
            ProgrammingLanguage::Kotlin => "kotlin",
            ProgrammingLanguage::Latex => "latex",
            ProgrammingLanguage::Less => "less",
            ProgrammingLanguage::Lisp => "lisp",
            ProgrammingLanguage::Livescript => "livescript",
            ProgrammingLanguage::Lua => "lua",
            ProgrammingLanguage::Makefile => "makefile",
            ProgrammingLanguage::Markdown => "markdown",
            ProgrammingLanguage::Markup => "markup",
            ProgrammingLanguage::Matlab => "matlab",
            ProgrammingLanguage::Mermaid => "mermaid",
            ProgrammingLanguage::Nix => "nix",
            ProgrammingLanguage::ObjectiveC => "objectivec",
            ProgrammingLanguage::Ocaml => "ocaml",
            ProgrammingLanguage::Pascal => "pascal",
            ProgrammingLanguage::Perl => "perl",
            ProgrammingLanguage::Php => "php",
            ProgrammingLanguage::PlainText => "plaintext",
            ProgrammingLanguage::Powershell => "powershell",
            ProgrammingLanguage::Prolog => "prolog",
            ProgrammingLanguage::Protobuf => "protobuf",
            ProgrammingLanguage::Python => "python",
            ProgrammingLanguage::R => "r",
            ProgrammingLanguage::Reason => "reason",
            ProgrammingLanguage::Ruby => "ruby",
            ProgrammingLanguage::RustLang => "rust",
            ProgrammingLanguage::Sass => "sass",
            ProgrammingLanguage::Scala => "scala",
            ProgrammingLanguage::Scheme => "scheme",
            ProgrammingLanguage::Scss => "scss",
            ProgrammingLanguage::Shell => "shell",
            ProgrammingLanguage::Sql => "sql",
            ProgrammingLanguage::Swift => "swift",
            ProgrammingLanguage::Typescript => "typescript",
            ProgrammingLanguage::VbNet => "vbnet",
            ProgrammingLanguage::Verilog => "verilog",
            ProgrammingLanguage::Vhdl => "vhdl",
            ProgrammingLanguage::VisualBasic => "visualbasic",
            ProgrammingLanguage::WebAssembly => "webassembly",
            ProgrammingLanguage::Xml => "xml",
            ProgrammingLanguage::Yaml => "yaml",
            ProgrammingLanguage::JavaCCppCSharp => "javac-cpp-csharp",
        }
    }
}

/// --------------------------------------------------------------------------------
/// Column
/// https://developers.notion.com/reference/block#column-list-and-column
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ColumnBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub column: Value,
}

/// --------------------------------------------------------------------------------
/// Column list
/// https://developers.notion.com/reference/block#column-list-and-column
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ColumnListBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub column_list: Value,
}

/// --------------------------------------------------------------------------------
/// Divider
/// https://developers.notion.com/reference/block#divider
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct DividerBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub divider: Value,
}

/// --------------------------------------------------------------------------------
/// Embed
/// https://developers.notion.com/reference/block#embed
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct EmbedBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub embed: EmbedField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EmbedField {
    pub url: String,
}

/// --------------------------------------------------------------------------------
/// Equation
/// https://developers.notion.com/reference/block#equation
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct EquationBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub equation: EquationField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EquationField {
    pub expression: String,
}

/// --------------------------------------------------------------------------------
/// File
/// https://developers.notion.com/reference/block#file
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct FileBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub file: FileObject,
}

/// --------------------------------------------------------------------------------
/// Heading
/// https://developers.notion.com/reference/block#headings
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct Heading1Block {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub heading_1: RichText,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Heading2Block {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub heading_2: RichText,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Heading3Block {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub heading_3: RichText,
}

/// --------------------------------------------------------------------------------
/// Image
/// https://developers.notion.com/reference/block#image
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ImageBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub image: FileObject,
}

/// --------------------------------------------------------------------------------
/// Link Preview
/// https://developers.notion.com/reference/block#link-preview
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct LinkPreviewBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub link_preview: LinkPreviewField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LinkPreviewField {
    pub url: String,
}

/// --------------------------------------------------------------------------------
/// Numbered list item
/// https://developers.notion.com/reference/block#numbered-list-item
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct NumberedListItemBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub numbered_list_item: RichText,
}

/// --------------------------------------------------------------------------------
/// Mention
/// https://developers.notion.com/reference/block#mention
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct MentionBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub page: MentionField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MentionField {
    pub id: String,
}

/// --------------------------------------------------------------------------------
/// Paragraph
/// https://developers.notion.com/reference/block#paragraph
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ParagraphBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub paragraph: RichText,
}

/// --------------------------------------------------------------------------------
/// PDF
/// https://developers.notion.com/reference/block#pdf
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct PdfBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub pdf: PDFField,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PDFField {
    External { external: Value },
    File { file: Value },
}

/// --------------------------------------------------------------------------------
/// Quote
/// https://developers.notion.com/reference/block#quote
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct QuoteBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub quote: RichText,
}

/// --------------------------------------------------------------------------------
/// Synced Block
/// https://developers.notion.com/reference/block#synced-block
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct SyncedBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub synced_block: Value,
}

/// --------------------------------------------------------------------------------
/// Table
/// https://developers.notion.com/reference/block#table
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct TableBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub table: TableField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TableField {
    pub table_width: u32,
    pub has_column_header: bool,
    pub has_row_header: bool,
}

/// --------------------------------------------------------------------------------
/// Table of contents
/// https://developers.notion.com/reference/block#table-of-contents
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct TableOfContentsBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub table_of_contents: TableOfContentsField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TableOfContentsField {
    pub color: Color,
}

/// --------------------------------------------------------------------------------
/// Table rows
/// https://developers.notion.com/reference/block#table-rows
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct TableRowBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub table_row: TableRowField,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TableRowField {
    pub cells: Vec<Vec<RichTextElement>>,
}

/// --------------------------------------------------------------------------------
/// Template
///
/// @depricated
///
/// Deprecation Notice
/// As of March 27, 2023 creation of template blocks will no longer be supported.
///
/// https://developers.notion.com/reference/block#template
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct TemplateBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub template: RichText,
}

/// --------------------------------------------------------------------------------
/// To do
/// https://developers.notion.com/reference/block#to-do
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ToDoBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub to_do: RichText,
}

/// --------------------------------------------------------------------------------
/// Toggle blocks
/// https://developers.notion.com/reference/block#toggle-blocks
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ToggleBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub toggle: RichText,
}

/// --------------------------------------------------------------------------------
/// Unsupported
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct UnsupportedBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
}

/// --------------------------------------------------------------------------------
/// Video
/// https://developers.notion.com/reference/block#video
/// --------------------------------------------------------------------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct VideoBlock {
    #[serde(flatten)]
    pub base: BaseBlock,
    pub video: FileObject,
}
