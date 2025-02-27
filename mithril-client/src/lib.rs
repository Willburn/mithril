#![warn(missing_docs)]

//! Define everything necessary to list, download, and validate snapshots from a
//! [Mithril Aggregator](https://mithril.network/mithril-aggregator/doc/mithril_aggregator/index.html).
//!
//! To query an aggregator for snapshots & certificate use the [AggregatorHTTPClient] that implement
//! the [AggregatorHandler] trait.
//!
//! To list, download, and validate snapshots use the [Runtime].
//! You must initialize it by giving it a CertificateVerifier, a ProtocolGenesisVerifier and a [AggregatorHandler], and a
//! [Digester](https://mithril.network/mithril-common/doc/mithril_common/digesters/trait.Digester.html)
//! implementations using the `with_xxx` methods.

mod aggregator;
pub mod commands;
mod entities;
mod runtime;

pub use crate::aggregator::{AggregatorHTTPClient, AggregatorHandler, AggregatorHandlerError};
pub use crate::entities::Config;
pub use crate::runtime::{Runtime, RuntimeError};

pub use crate::runtime::convert_to_field_items;
