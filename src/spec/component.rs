use crate::spec::channel::{Channel, ChannelBindings, Parameter};
use crate::spec::common::{Either, ExternalDocumentation, RefOr, Tag};
use crate::spec::message::{
    CorrelationId, Message, MessageBindings, MessageTrait, MultiFormatSchema,
};
use crate::spec::operation::{
    Operation, OperationBindings, OperationReply, OperationReplyAddress, OperationTrait,
};
use crate::spec::security::SecurityScheme;
use crate::spec::server::{Server, ServerBindings, Variable};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Holds a set of reusable objects for different aspects of the AsyncAPI specification. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
pub struct Components {
    /// An object to hold reusable Schema Object. If this is a Schema Object, then the schemaFormat will be assumed to be "application/vnd.aai.asyncapi+json;version=asyncapi" where the version is equal to the AsyncAPI Version String.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub schemas: HashMap<String, RefOr<Either<schemars::Schema, MultiFormatSchema>>>,
    /// An object to hold reusable [Server Objects](Server).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub servers: HashMap<String, RefOr<Server>>,
    /// An object to hold reusable [Channel Objects](Channel).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub channels: HashMap<String, RefOr<Channel>>,
    /// An object to hold reusable [Operation Objects](Operation).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub operations: HashMap<String, RefOr<Operation>>,
    /// An object to hold reusable [Message Objects](Message).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub messages: HashMap<String, RefOr<Message>>,
    /// An object to hold reusable [Security Scheme Objects](SecurityScheme).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub security_schemes: HashMap<String, RefOr<SecurityScheme>>,
    /// An object to hold reusable [Server Variable Objects](Variable).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub server_variables: HashMap<String, RefOr<Variable>>,
    /// An object to hold reusable [Parameter Objects](Parameter).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, RefOr<Parameter>>,
    /// An object to hold reusable Correlation [ID Objects](CorrelationId).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub correlation_ids: HashMap<String, RefOr<CorrelationId>>,
    /// An object to hold reusable [Operation Reply Objects](OperationReply).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub replies: HashMap<String, RefOr<OperationReply>>,
    /// An object to hold reusable Operation [Reply Address Objects](OperationReplyAddress).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub reply_addresses: HashMap<String, RefOr<OperationReplyAddress>>,
    /// An object to hold reusable [External Documentation Objects](ExternalDocumentation).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub external_docs: HashMap<String, RefOr<ExternalDocumentation>>,
    /// An object to hold reusable [Tag Objects](Tag).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, RefOr<Tag>>,
    /// An object to hold reusable [Operation Trait Objects](OperationTrait).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub operation_traits: HashMap<String, RefOr<OperationTrait>>,
    /// An object to hold reusable [Message Trait Objects](MessageTrait).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub message_traits: HashMap<String, RefOr<MessageTrait>>,
    /// An object to hold reusable [Server Bindings Objects](ServerBindings).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub server_bindings: HashMap<String, RefOr<ServerBindings>>,
    /// An object to hold reusable [Channel Bindings Objects](ChannelBindings).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub channel_bindings: HashMap<String, RefOr<ChannelBindings>>,
    /// An object to hold reusable [Operation Bindings Objects](OperationBindings).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub operation_bindings: HashMap<String, RefOr<OperationBindings>>,
    /// An object to hold reusable [Message Bindings Objects](MessageBindings).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub message_bindings: HashMap<String, RefOr<MessageBindings>>,
}
