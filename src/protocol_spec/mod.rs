//! Protocol specifications and message field definitions.

pub mod message_field;
pub mod spec;

pub use spec::{Cipher, HandshakePattern, KeyPattern, ProtocolSpec};
