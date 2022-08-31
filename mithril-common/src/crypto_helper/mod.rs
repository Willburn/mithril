//! Tools and types to abstract the use of the [Mithril Core library](https://mithril.network/mithril-core/doc/mithril/index.html)

mod codec;
mod conversions;
#[cfg(feature = "test-utils")]
pub mod tests_setup;
mod types;

pub use codec::*;
pub use types::*;

/// The current protocol version
pub const PROTOCOL_VERSION: ProtocolVersion = "0.1.0";
