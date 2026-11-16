/// A PSF identifier, stored without quoting or validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

/// A PSF hexadecimal literal including its `0x` prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexLiteral(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Numeric(NumericType),
    Bool,
    Char,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveArray {
    pub primitive_type: PrimitiveType,
    pub nelems: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicArray {
    /// The field whose value supplies the byte-array length.
    pub id: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Array {
    Primitive(PrimitiveArray),
    Dynamic(DynamicArray),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeValue {
    Primitive(PrimitiveType),
    Array(Array),
}

/// A field declaration in a message format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    pub name: Identifier,
    pub type_value: TypeValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Format {
    pub name: Identifier,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PubkeyEncoding {
    Raw,
    Der,
    Pem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldSemantic {
    /// Declares a string literal supplied by the instance's field bindings.
    FixedString,
    /// Declares a byte literal supplied by the instance's field bindings.
    FixedBytes,
    Random(usize),
    Pubkey(PubkeyEncoding),
    Padding,
    Payload,
    Length,
    PaddingLength,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBinding {
    pub format_id: Identifier,
    pub field_id: Identifier,
    pub semantic: FieldSemantic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Client,
    Server,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Handshake,
    Data,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SequenceSpecifier {
    pub role: Role,
    pub phase: Phase,
    pub format_id: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptionFormatBinding {
    pub encrypt_id: Identifier,
    pub from_id: Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptionFieldDirective {
    pub plaintext: Identifier,
    pub ciphertext: Identifier,
    pub mac: Option<Identifier>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncryptionDirectives {
    pub binding: EncryptionFormatBinding,
    pub fields: Vec<EncryptionFieldDirective>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoSegment {
    /// Whether an instance must supply a password binding.
    pub has_password: bool,
    pub cipher: Cipher,
    pub encryption: Vec<EncryptionDirectives>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionsSegment {
    /// `None` omits the assignment; `Some(false)` emits an explicit false value.
    pub separate_length_field: Option<bool>,
}

/// The structure of a protocol, with literal values supplied by an instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolSpecification {
    pub formats: Vec<Format>,
    pub semantics: Vec<SemanticBinding>,
    pub sequence: Vec<SequenceSpecifier>,
    pub crypto: Option<CryptoSegment>,
    pub options: Option<OptionsSegment>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cipher {
    ChaCha20Poly1305,
    Aes128Gcm,
    Aes256Gcm,
}
