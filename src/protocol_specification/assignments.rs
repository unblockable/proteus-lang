use std::collections::HashMap;

use super::template::FieldReference;

/// A value supplied for a fixed string or fixed byte field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralValue {
    String(String),
    Bytes(HexLiteral),
}

impl LiteralValue {
    /// Accepts only `0x` followed by one or more ASCII hex digits.
    pub fn bytes(text: impl Into<String>) -> Option<Self> {
        let text = text.into();
        let digits = text.strip_prefix("0x")?;
        if digits.is_empty() || !digits.bytes().all(|digit| digit.is_ascii_hexdigit()) {
            return None;
        }
        Some(Self::Bytes(HexLiteral(text)))
    }
}

/// A PSF hex literal checked by [`LiteralValue::bytes`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexLiteral(String);

impl HexLiteral {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Fixed field values and the optional password for one protocol instance.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Assignments {
    pub fields: HashMap<FieldReference, LiteralValue>,
    pub password: Option<String>,
}
