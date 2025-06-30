#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod spec;
#[cfg(feature = "builder_unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "builder_unstable")))]
pub mod builder;
#[cfg(feature = "builder_unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "builder_unstable")))]
pub mod error;
