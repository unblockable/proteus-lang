use std::fmt::{self, Write};

use crate::protocol_spec::*;

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for HexLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl fmt::Display for NumericType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
        })
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeric(numeric) => write!(f, "{numeric}"),
            Self::Bool => f.write_str("bool"),
            Self::Char => f.write_str("char"),
        }
    }
}

impl fmt::Display for PrimitiveArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", self.primitive_type, self.nelems)
    }
}

impl fmt::Display for DynamicArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[u8; {}.size_of]", self.id)
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(array) => write!(f, "{array}"),
            Self::Dynamic(array) => write!(f, "{array}"),
        }
    }
}

impl fmt::Display for TypeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(primitive) => write!(f, "{primitive}"),
            Self::Array(array) => write!(f, "{array}"),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ NAME: {}; TYPE: {} }}", self.name, self.type_value)
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DEFINE {}", self.name)?;
        for (index, field) in self.fields.iter().enumerate() {
            if index > 0 {
                f.write_str(",\n")?;
            }
            write!(f, "  {field}")?;
        }
        f.write_str(";")
    }
}

/// Formats a string literal without allocating an intermediate escaped string.
struct Quoted<'a>(&'a str);

impl fmt::Display for Quoted<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('"')?;
        for character in self.0.chars() {
            match character {
                '"' => f.write_str("\\\"")?,
                '\\' => f.write_str("\\\\")?,
                '\n' => f.write_str("\\n")?,
                '\r' => f.write_str("\\r")?,
                '\t' => f.write_str("\\t")?,
                '\u{08}' => f.write_str("\\b")?,
                '\u{0c}' => f.write_str("\\f")?,
                c if c.is_ascii_control() => write!(f, "\\u{:04x}", u32::from(c))?,
                c => f.write_char(c)?,
            }
        }
        f.write_char('"')
    }
}

impl fmt::Display for PubkeyEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Raw => "RAW",
            Self::Der => "DER",
            Self::Pem => "PEM",
        })
    }
}

fn write_semantic(
    f: &mut fmt::Formatter<'_>,
    binding: &SemanticBinding,
    instance: &ProtocolInstance,
) -> fmt::Result {
    match &binding.semantic {
        FieldSemantic::FixedString => {
            let LiteralValue::String(value) = instance.literal(binding) else {
                unreachable!("the instance builder checked the literal kind");
            };
            write!(f, "FIXED_STRING({})", Quoted(value))
        }
        FieldSemantic::FixedBytes => {
            let LiteralValue::Bytes(value) = instance.literal(binding) else {
                unreachable!("the instance builder checked the literal kind");
            };
            write!(f, "FIXED_BYTES({value})")
        }
        FieldSemantic::Random(value) => write!(f, "RANDOM({value})"),
        FieldSemantic::Pubkey(value) => write!(f, "PUBKEY({value})"),
        FieldSemantic::Padding => f.write_str("PADDING"),
        FieldSemantic::Payload => f.write_str("PAYLOAD"),
        FieldSemantic::Length => f.write_str("LENGTH"),
        FieldSemantic::PaddingLength => f.write_str("PADDING_LENGTH"),
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Client => "CLIENT",
            Self::Server => "SERVER",
        })
    }
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Handshake => "HANDSHAKE",
            Self::Data => "DATA",
        })
    }
}

impl fmt::Display for SequenceSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ ROLE: {}; PHASE: {}; FORMAT: {} }};",
            self.role, self.phase, self.format_id
        )
    }
}

impl fmt::Display for Cipher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ChaCha20Poly1305 => "CHACHA20-POLY1305",
            Self::Aes128Gcm => "AES128GCM",
            Self::Aes256Gcm => "AES256GCM",
        })
    }
}

impl fmt::Display for EncryptionFormatBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ENCRYPT {} FROM {}", self.encrypt_id, self.from_id)
    }
}

impl fmt::Display for EncryptionFieldDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ PTEXT: {}; CTEXT: {}; MAC: ",
            self.plaintext, self.ciphertext
        )?;
        match &self.mac {
            Some(identifier) => write!(f, "{identifier}")?,
            None => f.write_str("NULL")?,
        }
        f.write_str(" }")
    }
}

impl fmt::Display for EncryptionDirectives {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.binding)?;
        for (index, field) in self.fields.iter().enumerate() {
            if index > 0 {
                f.write_str(",\n")?;
            }
            write!(f, "  {field}")?;
        }
        f.write_str(";")
    }
}

fn write_crypto(
    f: &mut fmt::Formatter<'_>,
    crypto: &CryptoSegment,
    password: Option<&str>,
) -> fmt::Result {
    f.write_str("@SEGMENT.CRYPTO\n")?;
    if let Some(password) = password {
        writeln!(f, "PASSWORD = {};", Quoted(password))?;
    }
    write!(f, "CIPHER = {};", crypto.cipher)?;
    for directives in &crypto.encryption {
        write!(f, "\n\n{directives}")?;
    }
    Ok(())
}

impl fmt::Display for OptionsSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("@SEGMENT.OPTIONS")?;
        if let Some(value) = self.separate_length_field {
            write!(f, "\nSEPARATE_LENGTH_FIELD = {value};")?;
        }
        Ok(())
    }
}

impl fmt::Display for ProtocolInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let specification = self.specification();
        f.write_str("@SEGMENT.FORMATS")?;
        for (index, format) in specification.formats.iter().enumerate() {
            f.write_str(if index == 0 { "\n" } else { "\n\n" })?;
            write!(f, "{format}")?;
        }
        f.write_str("\n\n@SEGMENT.SEMANTICS")?;
        for binding in &specification.semantics {
            write!(
                f,
                "\n{{ FORMAT: {}; FIELD: {}; SEMANTIC: ",
                binding.format_id, binding.field_id
            )?;
            write_semantic(f, binding, self)?;
            f.write_str(" };")?;
        }
        f.write_str("\n\n@SEGMENT.SEQUENCE")?;
        for entry in &specification.sequence {
            write!(f, "\n{entry}")?;
        }
        if let Some(crypto) = &specification.crypto {
            f.write_str("\n\n")?;
            write_crypto(f, crypto, self.bindings().password.as_deref())?;
        }
        if let Some(options) = &specification.options {
            write!(f, "\n\n{options}")?;
        }
        f.write_str("\n")
    }
}
