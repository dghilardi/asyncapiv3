use std::collections::HashMap;
use crate::spec::common::{Either, ExternalDocumentation, RefOr, Tag};

pub type Messages = HashMap<String, RefOr<Message>>;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes a message received on a given channel and operation.
pub struct Message {
    /// Schema definition of the application headers. Schema MUST be a map of key-value pairs. It MUST NOT define the protocol headers. If this is a Schema Object, then the schemaFormat will be assumed to be "application/vnd.aai.asyncapi+json;version=asyncapi" where the version is equal to the AsyncAPI Version String.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<RefOr<Either<schemars::schema::Schema, MultiFormatSchema>>>,
    /// Definition of the message payload. If this is a Schema Object, then the schemaFormat will be assumed to be "application/vnd.aai.asyncapi+json;version=asyncapi" where the version is equal to the AsyncAPI Version String.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<RefOr<Either<schemars::schema::Schema, MultiFormatSchema>>>,
    /// Definition of the correlation ID used for message tracing or matching.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<RefOr<CorrelationId>>,
    /// The content type to use when encoding/decoding a message's payload. The value MUST be a specific media type (e.g. application/json). When omitted, the value MUST be the one specified on the defaultContentType field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// A machine-friendly name for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A human-friendly title for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A short summary of what the message is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the message. CommonMark syntax can be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of tags for logical grouping and categorization of messages.
    #[serde(default)]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
    /// A map where the keys describe the name of the protocol and the values describe protocol-specific definitions for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<RefOr<MessageBindings>>,
    /// List of examples.
    #[serde(default)]
    pub examples: Vec<MessageExample>,
    /// A list of traits to apply to the message object. Traits MUST be merged using traits merge mechanism. The resulting object MUST be a valid Message Object.
    #[serde(default)]
    pub traits: Vec<RefOr<MessageTrait>>,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
/// The Multi Format Schema Object represents a schema definition. It differs from the Schema Object in that it supports multiple schema formats or languages (e.g., JSON Schema, Avro, etc.).
pub struct MultiFormatSchema {
    /// string	Required. A string containing the name of the schema format that is used to define the information. If schemaFormat is missing, it MUST default to application/vnd.aai.asyncapi+json;version={{asyncapi}} where {{asyncapi}} matches the AsyncAPI Version String. In such a case, this would make the Multi Format Schema Object equivalent to the Schema Object. When using Reference Object within the schema, the schemaFormat of the resource being referenced MUST match the schemaFormat of the schema that contains the initial reference. For example, if you reference Avro schema, then schemaFormat of referencing resource and the resource being reference MUST match.
    /// Check out the supported schema formats table for more information. Custom values are allowed but their implementation is OPTIONAL. A custom value MUST NOT refer to one of the schema formats listed in the table.
    /// When using Reference Objects within the schema, the schemaFormat of the referenced resource MUST match the schemaFormat of the schema containing the reference.
    pub schema_format: String,
    /// Required. Definition of the message payload. It can be of any type but defaults to Schema Object. It MUST match the schema format defined in schemaFormat, including the encoding type. E.g., Avro should be inlined as either a YAML or JSON object instead of as a string to be parsed as YAML or JSON. Non-JSON-based schemas (e.g., Protobuf or XSD) MUST be inlined as a string.
    pub schema: serde_json::Value,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrelationId {
    /// An optional description of the identifier. CommonMark syntax can be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A runtime expression that specifies the location of the correlation ID.
    pub location: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageBindings {
 // TODO: implement based on https://www.asyncapi.com/docs/reference/specification/v3.0.0#messageBindingsObject
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Message Example Object represents an example of a Message Object and MUST contain either headers and/or payload fields.
pub struct MessageExample {
    /// The value of this field MUST validate against the Message Object's headers field.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, serde_json::Value>,
    /// The value of this field MUST validate against the Message Object's payload field.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub payload: HashMap<String, serde_json::Value>,
    /// A machine-friendly name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A short summary of what the example is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes a trait that MAY be applied to a Message Object. This object MAY contain any property from the Message Object, except payload and traits.
/// If you're looking to apply traits to an operation, see the Operation Trait Object.
pub struct MessageTrait {
    /// Schema definition of the application headers. Schema MUST be a map of key-value pairs. It MUST NOT define the protocol headers. If this is a Schema Object, then the schemaFormat will be assumed to be "application/vnd.aai.asyncapi+json;version=asyncapi" where the version is equal to the AsyncAPI Version String.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<RefOr<Either<schemars::schema::Schema, MultiFormatSchema>>>,
    /// Definition of the correlation ID used for message tracing or matching.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<RefOr<CorrelationId>>,
    /// The content type to use when encoding/decoding a message's payload. The value MUST be a specific media type (e.g. application/json). When omitted, the value MUST be the one specified on the defaultContentType field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// A machine-friendly name for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A human-friendly title for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A short summary of what the message is about.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// A verbose explanation of the message. CommonMark syntax can be used for rich text representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of tags for logical grouping and categorization of messages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Additional external documentation for this message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<RefOr<ExternalDocumentation>>,
    /// A map where the keys describe the name of the protocol and the values describe protocol-specific definitions for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bindings: Option<RefOr<MessageBindings>>,
    /// List of examples.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<MessageExample>,
}