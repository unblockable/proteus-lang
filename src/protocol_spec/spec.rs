use std::collections::HashMap;

use super::message_field::{Field, FieldKind, FieldOrder};

/// An editable protocol description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolSpec {
    pub secparam: usize,
    pub fields: HashMap<FieldKind, Field>,
    pub field_order: FieldOrder,
    pub handshake_pattern: HandshakePattern,
    pub key_encoding: String,
    pub cipher: Cipher,
    pub greeting: bool,
    pub subprotocol_nrounds: usize,
    pub subprotocol_sizes: Vec<usize>,
    pub send_encrypted_cert: bool,
    pub encrypted_cert_nbytes: usize,
    pub separate_length_fields: bool,
    pub reversed_greeting: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakePattern {
    pub key_patterns: Vec<Vec<KeyPattern>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPattern {
    Ephemeral,
    EphemeralWithOptionalData,
    Static,
    EncryptedStatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cipher {
    ChaCha20Poly1305,
    Aes128Gcm,
    Aes256Gcm,
}
