use pest::Parser;
use pest_derive::Parser;
use proteus_lang::protocol_specification::*;

mod common;

#[derive(Parser)]
#[grammar = "tests/fixtures/proteus_lite.pest"]
struct PsfSyntax;

fn assert_psf_syntax(instance: &ProtocolSpecification) {
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
fn byte_literal_construction_matches_the_hex_grammar() {
    for text in [
        "0xF", "0x00aF", "", "ff", "0Xff", "0x", "0x-2", "0x1g", "0x０", "0x00 ",
    ] {
        let accepted =
            PsfSyntax::parse(Rule::hex_literal, text).is_ok_and(|pairs| pairs.as_str() == text);
        assert_eq!(LiteralValue::bytes(text).is_some(), accepted, "{text:?}");
    }
}

#[test]
fn escaped_assignments_match_the_pinned_grammar() {
    let (mut template, mut assignments) = common::literal();
    template.crypto = Some(CryptoSegment {
        has_password: true,
        cipher: Cipher::Aes128Gcm,
        encryption: vec![],
    });
    let value = "quotes: \" backslash: \\ controls: \n\r\t\0\u{7f} Unicode: café 🦀";
    *assignments.fields.values_mut().next().unwrap() = LiteralValue::String(value.into());
    assignments.password = Some(value.into());
    let instance = binder::bind(template, assignments).unwrap();
    assert_psf_syntax(&instance);
}

#[test]
fn optional_empty_sections_match_the_pinned_grammar() {
    let mut template = common::minimal().template().clone();
    template.semantics.clear();
    template.crypto = Some(CryptoSegment {
        has_password: false,
        cipher: Cipher::ChaCha20Poly1305,
        encryption: vec![],
    });
    template.options = Some(OptionsSegment {
        separate_length_field: None,
    });
    let instance = binder::bind(template, Assignments::default()).unwrap();
    assert_psf_syntax(&instance);
}
