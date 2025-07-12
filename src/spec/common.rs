#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Clone for Either<L, R>
where
    L: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::Left(l) => Self::Left(l.clone()),
            Self::Right(r) => Self::Right(r.clone()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferenceObject {
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl ReferenceObject {
    pub fn new_channel(channel_name: &str) -> Self {
        Self {
            reference: format!("#/channels/{channel_name}"),
        }
    }

    pub fn new_message(message_name: &str) -> Self {
        Self {
            reference: format!("#/components/messages/{message_name}"),
        }
    }

    pub fn new_channel_message(channel_name: &str, message_name: &str) -> Self {
        Self {
            reference: format!("#/channels/{channel_name}/messages/{message_name}"),
        }
    }
}

pub type RefOr<T> = Either<ReferenceObject, T>;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The name of the tag.
    pub name: String,
    /// A short description for the tag. CommonMark syntax can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocumentation {
    /// A short description of the target documentation. CommonMark syntax can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL for the target documentation. This MUST be in the form of an absolute URL.
    pub url: String,
}
