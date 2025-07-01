#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "builder_unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "builder_unstable")))]
pub mod builder;
#[cfg(feature = "builder_unstable")]
#[cfg_attr(docsrs, doc(cfg(feature = "builder_unstable")))]
pub mod error;
pub mod spec;
