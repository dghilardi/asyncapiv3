use crate::spec::channel::Channels;
use crate::spec::component::Components;
use crate::spec::info::Info;
use crate::spec::operation::Operations;
use crate::spec::server::Servers;

pub mod channel;
pub mod common;
pub mod component;
pub mod info;
pub mod message;
pub mod operation;
pub mod security;
pub mod server;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "asyncapi")]
pub enum AsyncApiSpec {
    #[serde(rename = "3.0.0")]
    V3_0_0(AsyncApiV3Spec),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AsyncApiV3Spec {
    /// Identifier of the application the AsyncAPI document is defining.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Provides metadata about the API. The metadata can be used by the clients if needed.
    pub info: Info,
    /// Provides connection details of servers.
    #[serde(default)]
    pub servers: Servers,
    /// Default content type to use when encoding/decoding a message's payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_content_type: Option<String>,
    /// The channels used by this application.
    #[serde(default)]
    pub channels: Channels,
    /// The operations this application MUST implement.
    #[serde(default)]
    pub operations: Operations,
    /// An element to hold various reusable objects for the specification. Everything that is defined inside this object represents a resource that MAY or MAY NOT be used in the rest of the document and MAY or MAY NOT be used by the implemented Application.
    #[serde(default)]
    pub components: Components,
}
