#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "writer")]
#[cfg_attr(docsrs, doc(cfg(feature = "writer")))]
pub mod builder;
#[cfg(feature = "writer")]
#[cfg_attr(docsrs, doc(cfg(feature = "writer")))]
pub mod error;
pub mod spec;
