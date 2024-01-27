use std::collections::HashMap;
use crate::spec::common::{ExternalDocumentation, ReferenceObject, RefOr, Tag};
use crate::spec::message::Messages;

pub type Channels = HashMap<String, RefOr<Channel>>;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    /// An optional string representation of this channel's address. The address is typically the "topic name", "routing key", "event type", or "path". When null or absent, it MUST be interpreted as unknown. This is useful when the address is generated dynamically at runtime or can't be known upfront. It MAY contain Channel Address Expressions. Query parameters and fragments SHALL NOT be used, instead use bindings to define them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// A map of the messages that will be sent to this channel by any application at any time. Every message sent to this channel MUST be valid against one, and only one, of the message objects defined in this map.
    pub messages: Messages,
    /// A human-friendly title for the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A short summary of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// An optional description of this channel. CommonMark syntax can be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An array of $ref pointers to the definition of the servers in which this channel is available. If the channel is located in the root Channels Object, it MUST point to a subset of server definitions located in the root Servers Object, and MUST NOT point to a subset of server definitions located in the Components Object or anywhere else. If the channel is located in the Components Object, it MAY point to a Server Objects in any location. If servers is absent or empty, this channel MUST be available on all the servers defined in the Servers Object. Please note the servers property value MUST be an array of Reference Objects and, therefore, MUST NOT contain an array of Server Objects. However, it is RECOMMENDED that parsers (or other software) dereference this property for a better development experience.
    #[serde(default)]
    pub servers: Vec<ReferenceObject>,
    /// A map of the parameters included in the channel address. It MUST be present only when the address contains Channel Address Expressions.
    #[serde(default)]
    pub parameters: Parameters,
    /// A list of tags for logical grouping of channels.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
    /// A map where the keys describe the name of the protocol and the values describe protocol-specific definitions for the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<RefOr<ChannelBindings>>,
}

pub type Parameters = HashMap<String, RefOr<Parameter>>;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// An enumeration of string values to be used if the substitution options are from a limited set.
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    /// The default value to use for substitution, and to send, if an alternate value is not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// An optional description for the parameter. CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An array of examples of the parameter value.
    #[serde(default)]
    pub examples: Vec<String>,
    /// A runtime expression that specifies the location of the parameter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelBindings {
    // TODO: implement based on https://www.asyncapi.com/docs/reference/specification/v3.0.0#channelBindingsObject
}