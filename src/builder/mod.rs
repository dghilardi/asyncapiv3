use crate::spec::{AsyncApiSpec, AsyncApiV3Spec};
use crate::spec::channel::{Channel, Channels};
use crate::spec::common::{ReferenceObject, RefOr};
use crate::spec::component::Components;
use crate::spec::info::Info;
use crate::spec::operation::{Operation, Operations};
use crate::spec::server::{Server, Servers};

pub struct AsyncApiV3Builder {
    spec: AsyncApiV3Spec,
}

impl AsyncApiV3Builder {
    pub fn build(self) -> AsyncApiSpec {
        AsyncApiSpec::V3_0_0(self.spec)
    }

    pub fn new(info: Info) -> Self {
        Self {
            spec: AsyncApiV3Spec {
                id: None,
                info,
                servers: Default::default(),
                default_content_type: None,
                channels: Default::default(),
                operations: Default::default(),
                components: Default::default(),
            },
        }
    }

    pub fn id(mut self, id: String) -> Self {
        self.spec.id = Some(id);
        self
    }

    pub fn default_content_type(mut self, default_content_type: String) -> Self {
        self.spec.default_content_type = Some(default_content_type);
        self
    }

    pub fn servers(mut self, servers: Servers) -> Self {
        self.spec.servers = servers;
        self
    }

    pub fn register_server(&mut self, name: &str, server: Server) {
        self.spec.servers.insert(String::from(name), RefOr::Right(server));
    }

    pub fn register_server_ref(&mut self, name: &str, reference: ReferenceObject) {
        self.spec.servers.insert(String::from(name), RefOr::Left(reference));
    }
    pub fn channels(mut self, channels: Channels) -> Self {
        self.spec.channels = channels;
        self
    }

    pub fn register_channel(&mut self, name: &str, channel: Channel) {
        self.spec.channels.insert(String::from(name), RefOr::Right(channel));
    }

    pub fn register_channel_ref(&mut self, name: &str, reference: ReferenceObject) {
        self.spec.channels.insert(String::from(name), RefOr::Left(reference));
    }
    pub fn operations(mut self, operations: Operations) -> Self {
        self.spec.operations = operations;
        self
    }

    pub fn register_operation(&mut self, name: &str, operation: Operation) {
        self.spec.operations.insert(String::from(name), RefOr::Right(operation));
    }

    pub fn register_operation_ref(&mut self, name: &str, reference: ReferenceObject) {
        self.spec.operations.insert(String::from(name), RefOr::Left(reference));
    }
    pub fn components(mut self, components: Components) -> Self {
        self.spec.components = components;
        self
    }
}