use crate::spec::common::{ExternalDocumentation, RefOr, ReferenceObject, Tag};
use crate::spec::security::SecurityScheme;
use std::collections::HashMap;

pub type Operations = HashMap<String, RefOr<Operation>>;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes a specific operation.
pub struct Operation {
    /// Use send when it's expected that the application will send a message to the given channel, and receive when the application should expect receiving messages from the given channel.
    pub action: OperationAction,
    /// A $ref pointer to the definition of the channel in which this operation is performed. If the operation is located in the root Operations Object, it MUST point to a channel definition located in the root Channels Object, and MUST NOT point to a channel definition located in the Components Object or anywhere else. If the operation is located in the Components Object, it MAY point to a Channel Object in any location. Please note the channel property value MUST be a Reference Object and, therefore, MUST NOT contain a Channel Object. However, it is RECOMMENDED that parsers (or other software) dereference this property for a better development experience.
    pub channel: ReferenceObject,
    /// A human-friendly title for the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A short summary of what the operation is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation. CommonMark syntax can be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A declaration of which security schemes are associated with this operation. Only one of the security scheme objects MUST be satisfied to authorize an operation. In cases where Server Security also applies, it MUST also be satisfied.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<RefOr<SecurityScheme>>,
    /// A list of tags for logical grouping and categorization of operations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
    /// A map where the keys describe the name of the protocol and the values describe protocol-specific definitions for the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<RefOr<OperationBindings>>,
    /// A list of traits to apply to the operation object. Traits MUST be merged using traits merge mechanism. The resulting object MUST be a valid Operation Object.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub traits: Vec<RefOr<OperationTrait>>,
    /// A list of $ref pointers pointing to the supported Message Objects that can be processed by this operation. It MUST contain a subset of the messages defined in the channel referenced in this operation, and MUST NOT point to a subset of message definitions located in the Messages Object in the Components Object or anywhere else. Every message processed by this operation MUST be valid against one, and only one, of the message objects referenced in this list. Please note the messages property value MUST be a list of Reference Objects and, therefore, MUST NOT contain Message Objects. However, it is RECOMMENDED that parsers (or other software) dereference this property for a better development experience.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ReferenceObject>>,
    /// The definition of the reply in a request-reply operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply: Option<RefOr<OperationReply>>,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OperationAction {
    Send,
    Receive,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationBindings {
    //TODO: implement operation-binding object https://www.asyncapi.com/docs/reference/specification/v3.0.0#operationBindingsObject
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WebSocketOperationBinding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nats: Option<NatsOperationBinding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpOperationBinding>,
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpOperationMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Connect,
    Trace,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HttpOperationBinding {
    /// The HTTP method for the request.
    pub method: HttpOperationMethod,
    /// A Schema object containing the definitions for each query parameter.
    /// This schema MUST be of type object and have a properties key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<RefOr<schemars::Schema>>,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WebSocketOperationBinding;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NatsOperationBinding {
    /// Defines the name of the queue to use.
    /// It MUST NOT exceed 255 characters.
    pub queue: String,
    /// The version of this binding. If omitted, "latest" MUST be assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_version: Option<String>,
}

impl NatsOperationBinding {
    pub fn binding_version(&self) -> &str {
        self.binding_version.as_deref().unwrap_or("latest")
    }
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes a trait that MAY be applied to an [Operation Object](Operation). This object MAY contain any property from the [Operation Object](Operation), except the action, channel and traits ones.
/// If you're looking to apply traits to a message, see the [Message Trait Object](super::message::MessageTrait).
pub struct OperationTrait {
    /// A human-friendly title for the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A short summary of what the operation is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the operation. CommonMark syntax can be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A declaration of which security schemes are associated with this operation. Only one of the security scheme objects MUST be satisfied to authorize an operation. In cases where Server Security also applies, it MUST also be satisfied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<RefOr<SecurityScheme>>,
    /// A list of tags for logical grouping and categorization of operations.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
    /// A map where the keys describe the name of the protocol and the values describe protocol-specific definitions for the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<RefOr<OperationBindings>>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes the reply part that MAY be applied to an [Operation Object](Operation). If an operation implements the request/reply pattern, the reply object represents the response message.
pub struct OperationReply {
    /// Definition of the address that implementations MUST use for the reply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<RefOr<OperationReplyAddress>>,
    /// A $ref pointer to the definition of the channel in which this operation is performed. When address is specified, the address property of the channel referenced by this property MUST be either null or not defined. If the operation reply is located inside a root Operation Object, it MUST point to a channel definition located in the root Channels Object, and MUST NOT point to a channel definition located in the Components Object or anywhere else. If the operation reply is located inside an [Operation Object] in the Components Object or in the Replies Object in the Components Object, it MAY point to a Channel Object in any location. Please note the channel property value MUST be a Reference Object and, therefore, MUST NOT contain a Channel Object. However, it is RECOMMENDED that parsers (or other software) dereference this property for a better development experience.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<ReferenceObject>,
    /// A list of $ref pointers pointing to the supported Message Objects that can be processed by this operation as reply. It MUST contain a subset of the messages defined in the channel referenced in this operation reply, and MUST NOT point to a subset of message definitions located in the Components Object or anywhere else. Every message processed by this operation MUST be valid against one, and only one, of the message objects referenced in this list. Please note the messages property value MUST be a list of Reference Objects and, therefore, MUST NOT contain Message Objects. However, it is RECOMMENDED that parsers (or other software) dereference this property for a better development experience.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<ReferenceObject>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// An object that specifies where an operation has to send the reply.
/// For specifying and computing the location of a reply address, a [runtime expression](https://www.asyncapi.com/docs/reference/specification/v3.0.0#runtimeExpression) is used.
pub struct OperationReplyAddress {
    /// An optional description of the address. CommonMark syntax can be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A runtime expression that specifies the location of the reply address.
    pub location: String,
}
