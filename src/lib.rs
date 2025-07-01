#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod spec;
#[cfg(feature = "writer")]
#[cfg_attr(docsrs, doc(cfg(feature = "writer")))]
pub mod builder;
#[cfg(feature = "writer")]
#[cfg_attr(docsrs, doc(cfg(feature = "writer")))]
pub mod error;
