use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Duplicate definition entry '{key}'")]
    DuplicateDefinition { key: String },
    #[error("Duplicate operation entry '{name}'")]
    DuplicateOperation { name: String },
    #[error("Channel not found '{name}'")]
    ChannelNotFound { name: String },
}
