//! Construction of protocol instances from templates and assignments.

use super::{
    Assignments, FieldReference, FieldSemantic, LiteralValue, ProtocolSpecification, Template,
};

/// Binds assignments to a template, returning `None` if the binding is
/// not valid for this template.
///
/// Each fixed field needs an unambiguous assignment of the declared literal kind.
/// Every assignment must be used, and a password must be present exactly when
/// the template declares one. Pass `Assignments::default()` when no values are needed.
pub fn bind(template: Template, assignments: Assignments) -> Option<ProtocolSpecification> {
    if !assignments_match(&template, &assignments) {
        return None;
    }
    Some(ProtocolSpecification {
        template,
        assignments,
    })
}

fn assignments_match(template: &Template, assignments: &Assignments) -> bool {
    let expects_password = template
        .crypto
        .as_ref()
        .is_some_and(|crypto| crypto.has_password);
    if expects_password != assignments.password.is_some() {
        return false;
    }

    let mut slots = template.semantics.iter().filter(|binding| {
        matches!(
            binding.semantic,
            FieldSemantic::FixedString | FieldSemantic::FixedBytes
        )
    });
    if slots.clone().count() != assignments.fields.len() {
        return false;
    }
    slots.all(|slot| {
        matches!(
            (&slot.semantic, assignments.fields.get(&slot.target)),
            (FieldSemantic::FixedString, Some(LiteralValue::String(_)))
                | (FieldSemantic::FixedBytes, Some(LiteralValue::Bytes(_)))
        ) && unambiguous_target(template, &slot.target)
    })
}

fn unambiguous_target(template: &Template, target: &FieldReference) -> bool {
    let mut formats = template
        .formats
        .iter()
        .filter(|format| format.name == target.format_id);
    let Some(format) = formats.next() else {
        return false;
    };
    let fields = format
        .fields
        .iter()
        .filter(|field| field.name == target.field_id);
    let semantics = template
        .semantics
        .iter()
        .filter(|binding| binding.target == *target);
    formats.next().is_none() && fields.count() == 1 && semantics.count() == 1
}
