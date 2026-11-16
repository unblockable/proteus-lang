use proteus_lang::protocol_spec::*;

pub fn minimal() -> ProtocolInstance {
    let specification = ProtocolSpecification {
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
                format_id: "Data".into(),
                field_id: "length".into(),
                semantic: FieldSemantic::Length,
            },
            SemanticBinding {
                format_id: "Data".into(),
                field_id: "payload".into(),
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
    ProtocolInstance::builder(specification).build().unwrap()
}

pub fn complete() -> ProtocolInstance {
    let specification = ProtocolSpecification {
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
                format_id: "Hello".into(),
                field_id: "key".into(),
                semantic: FieldSemantic::Pubkey(PubkeyEncoding::Raw),
            },
            SemanticBinding {
                format_id: "Hello".into(),
                field_id: "greeting".into(),
                semantic: FieldSemantic::FixedString,
            },
            SemanticBinding {
                format_id: "Data".into(),
                field_id: "length".into(),
                semantic: FieldSemantic::Length,
            },
            SemanticBinding {
                format_id: "Data".into(),
                field_id: "payload".into(),
                semantic: FieldSemantic::Payload,
            },
            SemanticBinding {
                format_id: "Data".into(),
                field_id: "padding_length".into(),
                semantic: FieldSemantic::PaddingLength,
            },
            SemanticBinding {
                format_id: "Data".into(),
                field_id: "padding".into(),
                semantic: FieldSemantic::Padding,
            },
            SemanticBinding {
                format_id: "Data".into(),
                field_id: "tag".into(),
                semantic: FieldSemantic::Random(16),
            },
            SemanticBinding {
                format_id: "Data".into(),
                field_id: "reserved".into(),
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
    let bindings = ProtocolBindings {
        fields: [
            (
                FieldReference {
                    format_id: "Hello".into(),
                    field_id: "greeting".into(),
                },
                LiteralValue::String("hello".into()),
            ),
            (
                FieldReference {
                    format_id: "Data".into(),
                    field_id: "reserved".into(),
                },
                LiteralValue::Bytes(HexLiteral("0x00aF".into())),
            ),
        ]
        .into_iter()
        .collect(),
        password: Some("example-password".into()),
    };
    ProtocolInstance::builder(specification)
        .bindings(bindings)
        .build()
        .unwrap()
}

/// One declared string slot and its bindings, ready for builder and escaping tests.
pub fn literal() -> (ProtocolSpecification, ProtocolBindings) {
    let specification = ProtocolSpecification {
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
            format_id: "Message".into(),
            field_id: "value".into(),
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
    let bindings = ProtocolBindings {
        fields: [(
            FieldReference {
                format_id: "Message".into(),
                field_id: "value".into(),
            },
            LiteralValue::String("hello".into()),
        )]
        .into_iter()
        .collect(),
        password: None,
    };
    (specification, bindings)
}
