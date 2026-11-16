use proteus_lang::protocol_spec::*;

mod common;

#[test]
fn default_bindings_complete_a_specification_without_literal_slots() {
    let original = common::minimal();
    let instance = ProtocolInstance::builder(original.specification().clone())
        .build()
        .unwrap();
    assert_eq!(instance.bindings(), &ProtocolBindings::default());
}

#[test]
fn supplied_bindings_are_owned_and_available_read_only() {
    let (specification, bindings) = common::literal();
    let instance = ProtocolInstance::builder(specification.clone())
        .bindings(bindings.clone())
        .build()
        .unwrap();
    assert_eq!(instance.specification(), &specification);
    assert_eq!(instance.bindings(), &bindings);
}

#[test]
fn missing_required_field_binding_returns_none() {
    let (specification, _) = common::literal();
    assert!(ProtocolInstance::builder(specification).build().is_none());
}

#[test]
fn extra_field_binding_returns_none() {
    let (specification, mut bindings) = common::literal();
    bindings.fields.insert(
        FieldReference {
            format_id: "Message".into(),
            field_id: "extra".into(),
        },
        LiteralValue::String("unused".into()),
    );
    assert!(
        ProtocolInstance::builder(specification)
            .bindings(bindings)
            .build()
            .is_none()
    );
}

#[test]
fn an_unknown_binding_cannot_replace_a_required_binding() {
    let (specification, mut bindings) = common::literal();
    bindings.fields.clear();
    bindings.fields.insert(
        FieldReference {
            format_id: "Other".into(),
            field_id: "value".into(),
        },
        LiteralValue::String("unused".into()),
    );
    assert!(
        ProtocolInstance::builder(specification)
            .bindings(bindings)
            .build()
            .is_none()
    );
}

#[test]
fn literal_values_must_match_the_declared_kind() {
    for (semantic, wrong_value) in [
        (
            FieldSemantic::FixedString,
            LiteralValue::Bytes(HexLiteral("0x01".into())),
        ),
        (
            FieldSemantic::FixedBytes,
            LiteralValue::String("hello".into()),
        ),
    ] {
        let (mut specification, mut bindings) = common::literal();
        specification.semantics[0].semantic = semantic;
        *bindings.fields.values_mut().next().unwrap() = wrong_value;
        assert!(
            ProtocolInstance::builder(specification)
                .bindings(bindings)
                .build()
                .is_none()
        );
    }
}

#[test]
fn missing_declared_format_or_field_returns_none() {
    let (mut specification, bindings) = common::literal();
    specification.formats[0].fields.clear();
    assert!(
        ProtocolInstance::builder(specification.clone())
            .bindings(bindings.clone())
            .build()
            .is_none()
    );
    specification.formats.clear();
    assert!(
        ProtocolInstance::builder(specification)
            .bindings(bindings)
            .build()
            .is_none()
    );
}

#[test]
fn ambiguous_formats_fields_or_semantics_return_none() {
    let (specification, bindings) = common::literal();
    let mut duplicate_format = specification.clone();
    duplicate_format
        .formats
        .push(specification.formats[0].clone());
    let mut duplicate_field = specification.clone();
    duplicate_field.formats[0]
        .fields
        .push(specification.formats[0].fields[0].clone());
    let mut duplicate_semantic = specification.clone();
    duplicate_semantic.semantics.push(SemanticBinding {
        format_id: "Message".into(),
        field_id: "value".into(),
        semantic: FieldSemantic::Payload,
    });
    let mut duplicate_slot = specification.clone();
    duplicate_slot
        .semantics
        .push(specification.semantics[0].clone());

    for ambiguous in [
        duplicate_format,
        duplicate_field,
        duplicate_semantic,
        duplicate_slot,
    ] {
        assert!(
            ProtocolInstance::builder(ambiguous)
                .bindings(bindings.clone())
                .build()
                .is_none()
        );
    }
}

#[test]
fn identical_field_names_in_different_formats_have_distinct_bindings() {
    let (mut specification, mut bindings) = common::literal();
    let mut other = specification.formats[0].clone();
    other.name = "Other".into();
    specification.formats.push(other);
    specification.semantics.push(SemanticBinding {
        format_id: "Other".into(),
        field_id: "value".into(),
        semantic: FieldSemantic::FixedString,
    });
    bindings.fields.insert(
        FieldReference {
            format_id: "Other".into(),
            field_id: "value".into(),
        },
        LiteralValue::String("world".into()),
    );
    let instance = ProtocolInstance::builder(specification)
        .bindings(bindings)
        .build()
        .unwrap();
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
        let mut specification = original.specification().clone();
        specification.crypto.as_mut().unwrap().has_password = has_password;
        let mut bindings = original.bindings().clone();
        bindings.password = password.map(str::to_owned);
        assert_eq!(
            ProtocolInstance::builder(specification)
                .bindings(bindings)
                .build()
                .is_some(),
            succeeds
        );
    }
    let (specification, mut bindings) = common::literal();
    bindings.password = Some("undeclared".into());
    assert!(
        ProtocolInstance::builder(specification)
            .bindings(bindings)
            .build()
            .is_none()
    );
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
        let (mut specification, bindings) = common::literal();
        specification.semantics[0].semantic = semantic;
        assert!(
            ProtocolInstance::builder(specification.clone())
                .build()
                .is_some()
        );
        assert!(
            ProtocolInstance::builder(specification)
                .bindings(bindings)
                .build()
                .is_none()
        );
    }
}

#[test]
fn building_an_instance_does_not_apply_field_width_rules() {
    let (mut specification, bindings) = common::literal();
    specification.formats[0].fields[0].type_value =
        TypeValue::Array(Array::Primitive(PrimitiveArray {
            primitive_type: PrimitiveType::Char,
            nelems: 1,
        }));
    assert!(
        ProtocolInstance::builder(specification)
            .bindings(bindings)
            .build()
            .is_some()
    );
}

#[test]
fn one_specification_can_have_different_literal_values() {
    let original = common::complete();
    let mut bindings = original.bindings().clone();
    bindings.fields.insert(
        FieldReference {
            format_id: "Hello".into(),
            field_id: "greeting".into(),
        },
        LiteralValue::String("world".into()),
    );
    bindings.password = Some("another-password".into());
    let other = ProtocolInstance::builder(original.specification().clone())
        .bindings(bindings)
        .build()
        .unwrap();

    assert_eq!(original.specification(), other.specification());
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
