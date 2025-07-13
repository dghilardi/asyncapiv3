//! Module for common types or utilities used throughout the specification.

/// Either type used to store either one type or another.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Either<L, R> {
    /// The first potentially stored type
    Left(L),
    /// The second potentially stored type
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

/// To prevent excessive repetitions of shared elements AsyncAPI allows you to refer to already
/// defined objects via references. This type is used for serializing and deserializing said
/// references.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferenceObject {
    /// The reference for the referred to object
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl ReferenceObject {
    /// Creates a reference to a channel.
    pub fn new_channel(channel_name: &str) -> Self {
        Self {
            reference: format!("#/channels/{channel_name}"),
        }
    }

    /// Creates a reference to a message.
    pub fn new_message(message_name: &str) -> Self {
        Self {
            reference: format!("#/components/messages/{message_name}"),
        }
    }

    /// Creates a reference to a channel message.
    pub fn new_channel_message(channel_name: &str, message_name: &str) -> Self {
        Self {
            reference: format!("#/channels/{channel_name}/messages/{message_name}"),
        }
    }
}

pub type RefOr<T> = Either<ReferenceObject, T>;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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
