use proteus_lang::protocol_specification::*;

pub fn minimal() -> ProtocolSpecification {
    let template = Template {
        formats: vec![Format {
            name: "Data".into(),
            fields: vec![
                Field {
                    name: "length".into(),
                    type_value: TypeValue::Primitive(PrimitiveType::Numeric(NumericType::U16)),
                },
                Field {
                    name: "payload".into(),
                    type_value: TypeValue::Array(Array::Dynamic(DynamicArray {
                        id: "length".into(),
                    })),
                },
            ],
        }],
        semantics: vec![
            SemanticBinding {
                target: FieldReference::new("Data", "length"),
                semantic: FieldSemantic::Length,
            },
            SemanticBinding {
                target: FieldReference::new("Data", "payload"),
                semantic: FieldSemantic::Payload,
            },
        ],
        sequence: vec![
            SequenceSpecifier {
                role: Role::Client,
                phase: Phase::Data,
                format_id: "Data".into(),
            },
            SequenceSpecifier {
                role: Role::Server,
                phase: Phase::Data,
                format_id: "Data".into(),
            },
        ],
        crypto: None,
        options: None,
    };
    binder::bind(template, Assignments::default()).unwrap()
}

pub fn complete() -> ProtocolSpecification {
    let template = Template {
        formats: vec![
            Format {
                name: "Hello".into(),
                fields: vec![
                    Field {
                        name: "key".into(),
                        type_value: TypeValue::Array(Array::Primitive(PrimitiveArray {
                            primitive_type: PrimitiveType::Numeric(NumericType::U8),
                            nelems: 32,
                        })),
                    },
                    Field {
                        name: "greeting".into(),
                        type_value: TypeValue::Array(Array::Primitive(PrimitiveArray {
                            primitive_type: PrimitiveType::Char,
                            nelems: 5,
                        })),
                    },
                ],
            },
            Format {
                name: "Data".into(),
                fields: vec![
                    Field {
                        name: "length".into(),
                        type_value: TypeValue::Primitive(PrimitiveType::Numeric(NumericType::U16)),
                    },
                    Field {
                        name: "payload".into(),
                        type_value: TypeValue::Array(Array::Dynamic(DynamicArray {
                            id: "length".into(),
                        })),
                    },
                    Field {
                        name: "padding_length".into(),
                        type_value: TypeValue::Primitive(PrimitiveType::Numeric(NumericType::U16)),
                    },
                    Field {
                        name: "padding".into(),
                        type_value: TypeValue::Array(Array::Dynamic(DynamicArray {
                            id: "padding_length".into(),
                        })),
                    },
                    Field {
                        name: "tag".into(),
                        type_value: TypeValue::Array(Array::Primitive(PrimitiveArray {
                            primitive_type: PrimitiveType::Numeric(NumericType::U8),
                            nelems: 16,
                        })),
                    },
                    Field {
                        name: "reserved".into(),
                        type_value: TypeValue::Array(Array::Primitive(PrimitiveArray {
                            primitive_type: PrimitiveType::Numeric(NumericType::U8),
                            nelems: 2,
                        })),
                    },
                ],
            },
        ],
        semantics: vec![
            SemanticBinding {
                target: FieldReference::new("Hello", "key"),
                semantic: FieldSemantic::Pubkey(PubkeyEncoding::Raw),
            },
            SemanticBinding {
                target: FieldReference::new("Hello", "greeting"),
                semantic: FieldSemantic::FixedString,
            },
            SemanticBinding {
                target: FieldReference::new("Data", "length"),
                semantic: FieldSemantic::Length,
            },
            SemanticBinding {
                target: FieldReference::new("Data", "payload"),
                semantic: FieldSemantic::Payload,
            },
            SemanticBinding {
                target: FieldReference::new("Data", "padding_length"),
                semantic: FieldSemantic::PaddingLength,
            },
            SemanticBinding {
                target: FieldReference::new("Data", "padding"),
                semantic: FieldSemantic::Padding,
            },
            SemanticBinding {
                target: FieldReference::new("Data", "tag"),
                semantic: FieldSemantic::Random(16),
            },
            SemanticBinding {
                target: FieldReference::new("Data", "reserved"),
                semantic: FieldSemantic::FixedBytes,
            },
        ],
        sequence: vec![
            SequenceSpecifier {
                role: Role::Client,
                phase: Phase::Handshake,
                format_id: "Hello".into(),
            },
            SequenceSpecifier {
                role: Role::Server,
                phase: Phase::Handshake,
                format_id: "Hello".into(),
            },
            SequenceSpecifier {
                role: Role::Client,
                phase: Phase::Data,
                format_id: "Data".into(),
            },
            SequenceSpecifier {
                role: Role::Server,
                phase: Phase::Data,
                format_id: "Data".into(),
            },
        ],
        crypto: Some(CryptoSegment {
            has_password: true,
            cipher: Cipher::Aes256Gcm,
            encryption: vec![
                EncryptionDirectives {
                    binding: EncryptionFormatBinding {
                        encrypt_id: "Data".into(),
                        from_id: "Data".into(),
                    },
                    fields: vec![
                        EncryptionFieldDirective {
                            plaintext: "length".into(),
                            ciphertext: "length".into(),
                            mac: None,
                        },
                        EncryptionFieldDirective {
                            plaintext: "payload".into(),
                            ciphertext: "payload".into(),
                            mac: Some("tag".into()),
                        },
                    ],
                },
                EncryptionDirectives {
                    binding: EncryptionFormatBinding {
                        encrypt_id: "Hello".into(),
                        from_id: "Hello".into(),
                    },
                    fields: vec![EncryptionFieldDirective {
                        plaintext: "key".into(),
                        ciphertext: "key".into(),
                        mac: None,
                    }],
                },
            ],
        }),
        options: Some(OptionsSegment {
            separate_length_field: Some(true),
        }),
    };
    let assignments = Assignments {
        fields: [
            (
                FieldReference::new("Hello", "greeting"),
                LiteralValue::String("hello".into()),
            ),
            (
                FieldReference::new("Data", "reserved"),
                LiteralValue::bytes("0x00aF").unwrap(),
            ),
        ]
        .into_iter()
        .collect(),
        password: Some("example-password".into()),
    };
    binder::bind(template, assignments).unwrap()
}

/// A template with one fixed string and its matching assignment.
pub fn literal() -> (Template, Assignments) {
    let template = Template {
        formats: vec![Format {
            name: "Message".into(),
            fields: vec![Field {
                name: "value".into(),
                type_value: TypeValue::Array(Array::Primitive(PrimitiveArray {
                    primitive_type: PrimitiveType::Char,
                    nelems: 5,
                })),
            }],
        }],
        semantics: vec![SemanticBinding {
            target: FieldReference::new("Message", "value"),
            semantic: FieldSemantic::FixedString,
        }],
        sequence: vec![SequenceSpecifier {
            role: Role::Client,
            phase: Phase::Data,
            format_id: "Message".into(),
        }],
        crypto: None,
        options: None,
    };
    let assignments = Assignments {
        fields: [(
            FieldReference::new("Message", "value"),
            LiteralValue::String("hello".into()),
        )]
        .into_iter()
        .collect(),
        password: None,
    };
    (template, assignments)
}
