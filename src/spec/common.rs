#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReferenceObject {
    #[serde(rename = "$ref")]
    pub reference: String,
}

pub type RefOr<T> = Either<ReferenceObject, T>;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// A short description for the tag. CommonMark syntax can be used for rich text representation.
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocumentation {
    /// A short description of the target documentation. CommonMark syntax can be used for rich text representation.
    pub description: Option<String>,
    /// The URL for the target documentation. This MUST be in the form of an absolute URL.
    pub url: String,
}