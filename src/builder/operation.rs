use std::collections::HashMap;

use schemars::schema::Schema;

use crate::error::Error;
use crate::spec::channel::Channel;
use crate::spec::common::{Either, ReferenceObject, RefOr};
use crate::spec::message::Message;
use crate::spec::operation::{Operation, OperationAction, OperationReply, OperationReplyAddress};

use super::AsyncApiV3Builder;

pub struct MessageFullSpec {
    pub message: Message,
    pub definitions: HashMap<String, Schema>,
}

pub struct OperationInfo {
    pub name: String,
    pub address: String,
}

pub struct ReplyInfo {
    pub channel_name: String,
    pub reply_address_location: String,
}
impl AsyncApiV3Builder {
    pub fn register_simple_req_rep_operation(&mut self, operation: OperationInfo, reply_info: ReplyInfo, req: MessageFullSpec, res: MessageFullSpec) -> Result<(), Error> {
        if self.spec.operations.contains_key(&operation.name) {
            return Err(Error::DuplicateOperation { name: String::from(&operation.name) });
        }

        self.merge_schema_components(req.definitions)?;
        self.merge_schema_components(res.definitions)?;

        let req_name = if let Some(RefOr::Right(Either::Left(Schema::Object(ref schema)))) = req.message.payload {
            schema.metadata.as_ref()
                .and_then(|meta| meta.title.as_ref())
                .map(String::from)
        } else {
            None
        }.unwrap_or_else(|| format!("{}.req", operation.name));

        let res_name = if let Some(RefOr::Right(Either::Left(Schema::Object(ref schema)))) = res.message.payload {
            schema.metadata.as_ref()
                .and_then(|meta| meta.title.as_ref())
                .map(String::from)
        } else {
            None
        }.unwrap_or_else(|| format!("{}.res", operation.name));

        self.spec.channels.insert(String::from(&operation.name), RefOr::Right(Channel {
            address: Some(String::from(&operation.address)),
            messages: [(String::from(&req_name), RefOr::Right(req.message))]
                .into_iter()
                .collect(),
            title: None,
            summary: None,
            description: None,
            servers: vec![],
            parameters: Default::default(),
            tags: vec![],
            external_docs: None,
            bindings: None,
        }));

        self.register_response(&reply_info.channel_name, &res_name, res.message)?;

        self.spec.operations.insert(String::from(&operation.name), RefOr::Right(Operation {
            action: OperationAction::Send,
            channel: ReferenceObject::new_channel(&operation.name),
            title: None,
            summary: None,
            description: None,
            security: vec![],
            tags: vec![],
            external_docs: None,
            bindings: None,
            traits: vec![],
            messages: Some(vec![ReferenceObject::new_channel_message(&operation.name, &req_name)]),
            reply: Some(RefOr::Right(OperationReply {
                channel: Some(ReferenceObject::new_channel(&reply_info.channel_name)),
                address: Some(RefOr::Right(OperationReplyAddress {
                    description: None,
                    location: reply_info.reply_address_location,
                })),
                messages: vec![ReferenceObject::new_channel_message(&reply_info.channel_name, &res_name)],
            })),
        }));

        Ok(())
    }

    fn register_response(&mut self, reply_channel_name: &str, res_name: &str, message: Message) -> Result<(), Error> {
        let Some(RefOr::Right(channel)) = self.spec.channels.get_mut(reply_channel_name) else {
            return Err(Error::ChannelNotFound { name: String::from(reply_channel_name) });
        };
        channel.messages.insert(String::from(res_name), RefOr::Right(message));
        Ok(())
    }
    fn merge_schema_components(&mut self, partial: HashMap<String, Schema>) -> Result<(), Error> {
        for (key, schema) in partial {
            self.insert_schema_component(key, schema)?;
        }
        Ok(())
    }

    fn insert_schema_component(&mut self, key: String, schema: Schema) -> Result<(), Error> {
        let wrapped = RefOr::Right(Either::Left(schema));
        if let Some(current_schema) = self.spec.components.schemas.get(&key) {
            if !current_schema.eq(&wrapped) {
                log::warn!("Trying to add two schema components with the same name but different content {current_schema:?} - {wrapped:?}");
                return Err(Error::DuplicateDefinition { key });
            }
        } else {
            self.spec.components.schemas.insert(key, wrapped);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use schemars::JsonSchema;
    use crate::spec::info::Info;
    use super::*;

    #[derive(JsonSchema)]
    struct CommonData {
        number: u32,
        text: String,
    }

    #[derive(JsonSchema)]
    struct Req {
        data: CommonData,
    }

    #[derive(JsonSchema)]
    struct Res {
        data: CommonData,
    }

    #[test]
    fn build_spec() {
        let mut spec = AsyncApiV3Builder::new(Info {
            title: String::from("TEST"),
            version: String::from("1.0.0"),
            description: None,
            terms_of_service: None,
            contact: None,
            license: None,
            tags: vec![],
            external_docs: None,
        });

        let mut settings = schemars::gen::SchemaSettings::default();
        settings.definitions_path = String::from("#/components/schemas/");

        let req_schema = schemars::gen::SchemaGenerator::new(settings.clone()).into_root_schema_for::<Req>();
        let req_message = MessageFullSpec {
            message: Message {
                headers: None,
                payload: Some(RefOr::Right(Either::Left(Schema::Object(req_schema.schema)))),
                correlation_id: None,
                content_type: None,
                name: None,
                title: None,
                summary: None,
                description: None,
                tags: vec![],
                external_docs: None,
                bindings: None,
                examples: vec![],
                traits: vec![],
            },
            definitions: req_schema.definitions.into_iter().collect(),
        };


        let res_schema = schemars::gen::SchemaGenerator::new(settings).into_root_schema_for::<Res>();
        let res_message = MessageFullSpec {
            message: Message {
                headers: None,
                payload: Some(RefOr::Right(Either::Left(Schema::Object(res_schema.schema)))),
                correlation_id: None,
                content_type: None,
                name: None,
                title: None,
                summary: None,
                description: None,
                tags: vec![],
                external_docs: None,
                bindings: None,
                examples: vec![],
                traits: vec![],
            },
            definitions: res_schema.definitions.into_iter().collect(),
        };

        spec.register_channel("DemoRepl", Channel {
            address: None,
            messages: Default::default(),
            title: None,
            summary: None,
            description: None,
            servers: vec![],
            parameters: Default::default(),
            tags: vec![],
            external_docs: None,
            bindings: None,
        });
        spec.register_simple_req_rep_operation(
            OperationInfo { name: String::from("DemoOp"), address: String::from("demo.operation") },
            ReplyInfo { channel_name: String::from("DemoRepl"), reply_address_location: String::from("$message.payload#/reply") },
            req_message,
            res_message,
        ).unwrap();

        println!("{}", serde_json::to_string(&spec.build()).unwrap())
    }
}