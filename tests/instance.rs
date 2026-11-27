use proteus_lang::protocol_specification::*;

mod common;

#[test]
fn default_assignments_complete_a_template_without_literal_slots() {
    assert_eq!(common::minimal().assignments(), &Assignments::default());
}

#[test]
fn supplied_assignments_are_owned_and_available_read_only() {
    let (template, assignments) = common::literal();
    let instance = binder::bind(template.clone(), assignments.clone()).unwrap();
    assert_eq!(instance.template(), &template);
    assert_eq!(instance.assignments(), &assignments);
}

#[test]
fn missing_required_field_assignment_returns_none() {
    let (template, _) = common::literal();
    assert!(binder::bind(template, Assignments::default()).is_none());
}

#[test]
fn extra_field_assignment_returns_none() {
    let (template, mut assignments) = common::literal();
    assignments.fields.insert(
        FieldReference::new("Message", "extra"),
        LiteralValue::String("unused".into()),
    );
    assert!(binder::bind(template, assignments).is_none());
}

#[test]
fn an_unknown_assignment_cannot_replace_a_required_assignment() {
    let (template, mut assignments) = common::literal();
    assignments.fields.clear();
    assignments.fields.insert(
        FieldReference::new("Other", "value"),
        LiteralValue::String("unused".into()),
    );
    assert!(binder::bind(template, assignments).is_none());
}

#[test]
fn literal_values_must_match_the_declared_kind() {
    for (semantic, wrong_value) in [
        (
            FieldSemantic::FixedString,
            LiteralValue::bytes("0x01").unwrap(),
        ),
        (
            FieldSemantic::FixedBytes,
            LiteralValue::String("hello".into()),
        ),
    ] {
        let (mut template, mut assignments) = common::literal();
        template.semantics[0].semantic = semantic;
        *assignments.fields.values_mut().next().unwrap() = wrong_value;
        assert!(binder::bind(template, assignments).is_none());
    }
}

#[test]
fn missing_declared_format_or_field_returns_none() {
    let (mut template, assignments) = common::literal();
    template.formats[0].fields.clear();
    assert!(binder::bind(template.clone(), assignments.clone()).is_none());
    template.formats.clear();
    assert!(binder::bind(template, assignments).is_none());
}

#[test]
fn ambiguous_formats_fields_or_semantics_return_none() {
    let (template, assignments) = common::literal();
    let mut duplicate_format = template.clone();
    duplicate_format.formats.push(template.formats[0].clone());
    let mut duplicate_field = template.clone();
    duplicate_field.formats[0]
        .fields
        .push(template.formats[0].fields[0].clone());
    let mut duplicate_semantic = template.clone();
    duplicate_semantic.semantics.push(SemanticBinding {
        target: FieldReference::new("Message", "value"),
        semantic: FieldSemantic::Payload,
    });
    let mut duplicate_slot = template.clone();
    duplicate_slot.semantics.push(template.semantics[0].clone());

    for ambiguous in [
        duplicate_format,
        duplicate_field,
        duplicate_semantic,
        duplicate_slot,
    ] {
        assert!(binder::bind(ambiguous, assignments.clone()).is_none());
    }
}

#[test]
fn identical_field_names_in_different_formats_have_distinct_assignments() {
    let (mut template, mut assignments) = common::literal();
    let mut other = template.formats[0].clone();
    other.name = "Other".into();
    template.formats.push(other);
    template.semantics.push(SemanticBinding {
        target: FieldReference::new("Other", "value"),
        semantic: FieldSemantic::FixedString,
    });
    assignments.fields.insert(
        FieldReference::new("Other", "value"),
        LiteralValue::String("world".into()),
    );
    let instance = binder::bind(template, assignments).unwrap();
    let text = instance.to_string();
    assert!(text.contains("FORMAT: Message; FIELD: value; SEMANTIC: FIXED_STRING(\"hello\")"));
    assert!(text.contains("FORMAT: Other; FIELD: value; SEMANTIC: FIXED_STRING(\"world\")"));
}

#[test]
fn password_presence_must_match_its_declaration() {
    let original = common::complete();
    for (has_password, password, succeeds) in [
        (true, Some("secret"), true),
        (true, Some(""), true),
        (true, None, false),
        (false, Some("secret"), false),
        (false, None, true),
    ] {
        let mut template = original.template().clone();
        template.crypto.as_mut().unwrap().has_password = has_password;
        let mut assignments = original.assignments().clone();
        assignments.password = password.map(str::to_owned);
        assert_eq!(binder::bind(template, assignments).is_some(), succeeds);
    }
    let (template, mut assignments) = common::literal();
    assignments.password = Some("undeclared".into());
    assert!(binder::bind(template, assignments).is_none());
}

#[test]
fn runtime_semantics_do_not_declare_literal_slots() {
    for semantic in [
        FieldSemantic::Random(5),
        FieldSemantic::Pubkey(PubkeyEncoding::Raw),
        FieldSemantic::Padding,
        FieldSemantic::Payload,
        FieldSemantic::Length,
        FieldSemantic::PaddingLength,
    ] {
        let (mut template, assignments) = common::literal();
        template.semantics[0].semantic = semantic;
        assert!(binder::bind(template.clone(), Assignments::default()).is_some());
        assert!(binder::bind(template, assignments).is_none());
    }
}

#[test]
fn binding_a_template_does_not_apply_field_width_rules() {
    let (mut template, assignments) = common::literal();
    template.formats[0].fields[0].type_value = TypeValue::Array(Array::Primitive(PrimitiveArray {
        primitive_type: PrimitiveType::Char,
        nelems: 1,
    }));
    assert!(binder::bind(template, assignments).is_some());
}

#[test]
fn one_template_can_have_different_literal_values() {
    let original = common::complete();
    let mut assignments = original.assignments().clone();
    assignments.fields.insert(
        FieldReference::new("Hello", "greeting"),
        LiteralValue::String("world".into()),
    );
    assignments.password = Some("another-password".into());
    let other = binder::bind(original.template().clone(), assignments).unwrap();

    assert_eq!(original.template(), other.template());
    assert_eq!(
        other.to_string(),
        original
            .to_string()
            .replace("FIXED_STRING(\"hello\")", "FIXED_STRING(\"world\")")
            .replace(
                "PASSWORD = \"example-password\"",
                "PASSWORD = \"another-password\""
            ),
    );
}
