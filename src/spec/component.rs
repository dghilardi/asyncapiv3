use std::collections::HashMap;
use crate::spec::channel::{Channel, ChannelBindings, Parameter};
use crate::spec::common::{Either, ExternalDocumentation, RefOr, Tag};
use crate::spec::message::{CorrelationId, Message, MessageBindings, MessageTrait, MultiFormatSchema};
use crate::spec::operation::{Operation, OperationBindings, OperationReply, OperationReplyAddress, OperationTrait};
use crate::spec::security::SecurityScheme;
use crate::spec::server::{Server, ServerBindings, Variable};

#[derive(serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// Holds a set of reusable objects for different aspects of the AsyncAPI specification. All objects defined within the components object will have no effect on the API unless they are explicitly referenced from properties outside the components object.
pub struct Components {
    /// An object to hold reusable Schema Object. If this is a Schema Object, then the schemaFormat will be assumed to be "application/vnd.aai.asyncapi+json;version=asyncapi" where the version is equal to the AsyncAPI Version String.
    pub schemas: HashMap<String, RefOr<Either<schemars::schema::Schema, MultiFormatSchema>>>,
    /// An object to hold reusable [Server Objects](Server).
    pub servers: HashMap<String, RefOr<Server>>,
    /// An object to hold reusable [Channel Objects](Channel).
    pub channels: HashMap<String, RefOr<Channel>>,
    /// An object to hold reusable [Operation Objects](Operation).
    pub operations: HashMap<String, RefOr<Operation>>,
    /// An object to hold reusable [Message Objects](Message).
    pub messages: HashMap<String, RefOr<Message>>,
    /// An object to hold reusable [Security Scheme Objects](SecurityScheme).
    pub security_schemes: HashMap<String, RefOr<SecurityScheme>>,
    /// An object to hold reusable [Server Variable Objects](Variable).
    pub server_variables: HashMap<String, RefOr<Variable>>,
    /// An object to hold reusable [Parameter Objects](Parameter).
    pub parameters: HashMap<String, RefOr<Parameter>>,
    /// An object to hold reusable Correlation [ID Objects](CorrelationId).
    pub correlation_ids: HashMap<String, RefOr<CorrelationId>>,
    /// An object to hold reusable [Operation Reply Objects](OperationReply).
    pub replies: HashMap<String, RefOr<OperationReply>>,
    /// An object to hold reusable Operation [Reply Address Objects](OperationReplyAddress).
    pub reply_addresses: HashMap<String, RefOr<OperationReplyAddress>>,
    /// An object to hold reusable [External Documentation Objects](ExternalDocumentation).
    pub external_docs: HashMap<String, RefOr<ExternalDocumentation>>,
    /// An object to hold reusable [Tag Objects](Tag).
    pub tags: HashMap<String, RefOr<Tag>>,
    /// An object to hold reusable [Operation Trait Objects](OperationTrait).
    pub operation_traits: HashMap<String, RefOr<OperationTrait>>,
    /// An object to hold reusable [Message Trait Objects](MessageTrait).
    pub message_traits: HashMap<String, RefOr<MessageTrait>>,
    /// An object to hold reusable [Server Bindings Objects](ServerBindings).
    pub server_bindings: HashMap<String, RefOr<ServerBindings>>,
    /// An object to hold reusable [Channel Bindings Objects](ChannelBindings).
    pub channel_bindings: HashMap<String, RefOr<ChannelBindings>>,
    /// An object to hold reusable [Operation Bindings Objects](OperationBindings).
    pub operation_bindings: HashMap<String, RefOr<OperationBindings>>,
    /// An object to hold reusable [Message Bindings Objects](MessageBindings).
    pub message_bindings: HashMap<String, RefOr<MessageBindings>>,
}