use std::fmt::{self, Write};

use proteus_lang::protocol_spec::*;

mod common;

#[test]
fn minimal_instance_matches_canonical_fixture() {
    assert_eq!(
        common::minimal().to_string(),
        include_str!("fixtures/minimal.psf")
    );
}

#[test]
fn complete_instance_matches_canonical_fixture() {
    assert_eq!(
        common::complete().to_string(),
        include_str!("fixtures/complete.psf")
    );
}

#[test]
fn numeric_types_and_fixed_arrays_use_psf_spelling() {
    for (kind, expected) in [
        (NumericType::U8, "u8"),
        (NumericType::U16, "u16"),
        (NumericType::U32, "u32"),
        (NumericType::U64, "u64"),
        (NumericType::I8, "i8"),
        (NumericType::I16, "i16"),
        (NumericType::I32, "i32"),
        (NumericType::I64, "i64"),
    ] {
        assert_eq!(kind.to_string(), expected);
        let array = PrimitiveArray {
            primitive_type: PrimitiveType::Numeric(kind),
            nelems: 3,
        };
        assert_eq!(array.to_string(), format!("[{expected}; 3]"));
    }
    assert_eq!(PrimitiveType::Bool.to_string(), "bool");
    assert_eq!(PrimitiveType::Char.to_string(), "char");
}

#[test]
fn runtime_semantics_render_without_literal_bindings() {
    for (semantic, expected) in [
        (FieldSemantic::Random(12), "RANDOM(12)"),
        (FieldSemantic::Pubkey(PubkeyEncoding::Raw), "PUBKEY(RAW)"),
        (FieldSemantic::Pubkey(PubkeyEncoding::Der), "PUBKEY(DER)"),
        (FieldSemantic::Pubkey(PubkeyEncoding::Pem), "PUBKEY(PEM)"),
        (FieldSemantic::Padding, "PADDING"),
        (FieldSemantic::Payload, "PAYLOAD"),
        (FieldSemantic::Length, "LENGTH"),
        (FieldSemantic::PaddingLength, "PADDING_LENGTH"),
    ] {
        let (mut specification, _) = common::literal();
        specification.semantics[0].semantic = semantic;
        let instance = ProtocolInstance::builder(specification).build().unwrap();
        assert!(
            instance
                .to_string()
                .contains(&format!("SEMANTIC: {expected} }};"))
        );
    }
}

#[test]
fn cipher_names_and_password_presence_render_as_declared() {
    for (cipher, expected) in [
        (Cipher::Aes128Gcm, "AES128GCM"),
        (Cipher::Aes256Gcm, "AES256GCM"),
        (Cipher::ChaCha20Poly1305, "CHACHA20-POLY1305"),
    ] {
        assert_eq!(cipher.to_string(), expected);
    }
    let base = common::minimal();
    for password in [None, Some(""), Some("secret")] {
        let mut specification = base.specification().clone();
        specification.crypto = Some(CryptoSegment {
            has_password: password.is_some(),
            cipher: Cipher::Aes128Gcm,
            encryption: vec![],
        });
        let bindings = ProtocolBindings {
            password: password.map(str::to_owned),
            ..ProtocolBindings::default()
        };
        let instance = ProtocolInstance::builder(specification)
            .bindings(bindings)
            .build()
            .unwrap();
        let expected = match password {
            None => "@SEGMENT.CRYPTO\nCIPHER = AES128GCM;\n",
            Some("") => "@SEGMENT.CRYPTO\nPASSWORD = \"\";\nCIPHER = AES128GCM;\n",
            Some(_) => "@SEGMENT.CRYPTO\nPASSWORD = \"secret\";\nCIPHER = AES128GCM;\n",
        };
        assert!(instance.to_string().ends_with(expected));
    }
}

#[test]
fn options_preserve_omitted_settings_and_explicit_booleans() {
    for (setting, expected) in [
        (None, "@SEGMENT.OPTIONS"),
        (
            Some(false),
            "@SEGMENT.OPTIONS\nSEPARATE_LENGTH_FIELD = false;",
        ),
        (
            Some(true),
            "@SEGMENT.OPTIONS\nSEPARATE_LENGTH_FIELD = true;",
        ),
    ] {
        assert_eq!(
            OptionsSegment {
                separate_length_field: setting
            }
            .to_string(),
            expected
        );
    }
}

#[test]
fn field_literals_and_passwords_share_correct_escaping() {
    for (value, quoted) in [
        ("", "\"\""),
        ("\"\\", "\"\\\"\\\\\""),
        ("\n\r\t\u{08}\u{0c}", "\"\\n\\r\\t\\b\\f\""),
        (
            "\0\u{01}\u{0b}\u{1f}\u{7f}",
            "\"\\u0000\\u0001\\u000b\\u001f\\u007f\"",
        ),
        ("café / 🦀", "\"café / 🦀\""),
    ] {
        let (mut specification, mut bindings) = common::literal();
        specification.crypto = Some(CryptoSegment {
            has_password: true,
            cipher: Cipher::Aes128Gcm,
            encryption: vec![],
        });
        *bindings.fields.values_mut().next().unwrap() = LiteralValue::String(value.into());
        bindings.password = Some(value.into());
        let instance = ProtocolInstance::builder(specification)
            .bindings(bindings)
            .build()
            .unwrap();
        let text = instance.to_string();
        assert!(text.contains(&format!("FIXED_STRING({quoted})")));
        assert!(text.contains(&format!("PASSWORD = {quoted};")));
    }
}

#[test]
fn formatting_propagates_writer_errors() {
    struct FailingWriter;
    impl fmt::Write for FailingWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }
    assert!(write!(FailingWriter, "{}", common::complete()).is_err());
}
