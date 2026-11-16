use pest::Parser;
use pest_derive::Parser;
use proteus_lang::protocol_spec::*;

mod common;

#[derive(Parser)]
#[grammar = "tests/fixtures/proteus_lite.pest"]
struct PsfSyntax;

fn assert_psf_syntax(instance: &ProtocolInstance) {
    let text = instance.to_string();
    let pairs = PsfSyntax::parse(Rule::psf, &text).unwrap_or_else(|error| {
        panic!("Rendered PSF does not match Proteus grammar: {error}\n{text}")
    });
    assert_eq!(pairs.as_str(), text);
}

#[test]
fn representative_instances_match_the_pinned_grammar() {
    assert_psf_syntax(&common::minimal());
    assert_psf_syntax(&common::complete());
}

#[test]
fn escaped_bindings_match_the_pinned_grammar() {
    let (mut specification, mut bindings) = common::literal();
    specification.crypto = Some(CryptoSegment {
        has_password: true,
        cipher: Cipher::Aes128Gcm,
        encryption: vec![],
    });
    let value = "quotes: \" backslash: \\ controls: \n\r\t\0\u{7f} Unicode: café 🦀";
    *bindings.fields.values_mut().next().unwrap() = LiteralValue::String(value.into());
    bindings.password = Some(value.into());
    let instance = ProtocolInstance::builder(specification)
        .bindings(bindings)
        .build()
        .unwrap();
    assert_psf_syntax(&instance);
}

#[test]
fn optional_empty_sections_match_the_pinned_grammar() {
    let mut specification = common::minimal().specification().clone();
    specification.semantics.clear();
    specification.crypto = Some(CryptoSegment {
        has_password: false,
        cipher: Cipher::ChaCha20Poly1305,
        encryption: vec![],
    });
    specification.options = Some(OptionsSegment {
        separate_length_field: None,
    });
    let instance = ProtocolInstance::builder(specification).build().unwrap();
    assert_psf_syntax(&instance);
}
