use std::collections::HashMap;

use super::{FieldSemantic, HexLiteral, Identifier, ProtocolSpecification, SemanticBinding};

/// Identifies a field within a particular message format.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldReference {
    pub format_id: Identifier,
    pub field_id: Identifier,
}

/// Literal data supplied by an instance, distinct from runtime field values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralValue {
    String(String),
    Bytes(HexLiteral),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProtocolBindings {
    pub fields: HashMap<FieldReference, LiteralValue>,
    pub password: Option<String>,
}

/// An owned specification with a complete, compatible set of literal bindings.
///
/// Build with [`Self::builder`]. A completed instance can be rendered as PSF
/// using `Display`, `.to_string()`, or `write!`. Being binding-complete does not
/// guarantee that the protocol is semantically valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolInstance {
    specification: ProtocolSpecification,
    bindings: ProtocolBindings,
}

impl ProtocolInstance {
    pub fn builder(specification: ProtocolSpecification) -> ProtocolInstanceBuilder {
        ProtocolInstanceBuilder {
            specification,
            bindings: ProtocolBindings::default(),
        }
    }

    pub fn specification(&self) -> &ProtocolSpecification {
        &self.specification
    }

    pub fn bindings(&self) -> &ProtocolBindings {
        &self.bindings
    }

    pub(crate) fn literal(&self, binding: &SemanticBinding) -> &LiteralValue {
        // The private, immutable instance can only be created by the builder,
        // which checks that every literal slot has a compatible binding.
        // Compare borrowed identifiers to avoid allocating keys while rendering.
        self.bindings
            .fields
            .iter()
            .find(|(key, _)| key.format_id == binding.format_id && key.field_id == binding.field_id)
            .map(|(_, value)| value)
            .expect("the instance builder checked every literal slot")
    }
}

/// Accumulates inputs before creating a binding-complete instance.
#[derive(Debug)]
pub struct ProtocolInstanceBuilder {
    specification: ProtocolSpecification,
    bindings: ProtocolBindings,
}

impl ProtocolInstanceBuilder {
    pub fn bindings(mut self, bindings: ProtocolBindings) -> Self {
        self.bindings = bindings;
        self
    }

    /// Returns `None` for missing, extra, wrong-kind, or ambiguous bindings.
    pub fn build(self) -> Option<ProtocolInstance> {
        let specification = &self.specification;
        let has_password = specification
            .crypto
            .as_ref()
            .is_some_and(|crypto| crypto.has_password);
        if has_password != self.bindings.password.is_some() {
            return None;
        }

        let slots = specification.semantics.iter().filter(|binding| {
            matches!(
                binding.semantic,
                FieldSemantic::FixedString | FieldSemantic::FixedBytes
            )
        });
        if slots.clone().count() != self.bindings.fields.len() {
            return None;
        }

        for slot in slots {
            let key = FieldReference {
                format_id: slot.format_id.clone(),
                field_id: slot.field_id.clone(),
            };
            let value = self.bindings.fields.get(&key)?;
            if !matches!(
                (&slot.semantic, value),
                (FieldSemantic::FixedString, LiteralValue::String(_))
                    | (FieldSemantic::FixedBytes, LiteralValue::Bytes(_))
            ) || !unambiguous_target(specification, slot)
            {
                return None;
            }
        }

        Some(ProtocolInstance {
            specification: self.specification,
            bindings: self.bindings,
        })
    }
}

fn unambiguous_target(specification: &ProtocolSpecification, slot: &SemanticBinding) -> bool {
    let mut formats = specification
        .formats
        .iter()
        .filter(|format| format.name == slot.format_id);
    let Some(format) = formats.next() else {
        return false;
    };

    formats.next().is_none()
        && format
            .fields
            .iter()
            .filter(|field| field.name == slot.field_id)
            .count()
            == 1
        && specification
            .semantics
            .iter()
            .filter(|binding| {
                binding.format_id == slot.format_id && binding.field_id == slot.field_id
            })
            .count()
            == 1
}
