/// Defines overall protocol structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Template {
    pub formats: Vec<Format>,
    pub semantics: Vec<SemanticBinding>,
    pub sequence: Vec<SequenceSpecifier>,
    pub crypto: Option<CryptoSegment>,
    pub options: Option<OptionsSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub String);

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}

/// Used to identify a field in a protocol. This struct is just used to
/// unambiguously assign values to fields through [`Assignments`](super::Assignments).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldReference {
    pub format_id: Identifier,
    pub field_id: Identifier,
}

impl FieldReference {
    pub fn new(format_id: impl Into<Identifier>, field_id: impl Into<Identifier>) -> Self {
        Self {
            format_id: format_id.into(),
            field_id: field_id.into(),
        }
    }
}

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
    /// A string literal supplied when binding the template.
    FixedString,
    /// A byte literal supplied when binding the template.
    FixedBytes,
    Random(usize),
    Pubkey(PubkeyEncoding),
    Padding,
    Payload,
    Length,
    PaddingLength,
}

/// Assigns a PSF semantic to a field in the template.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticBinding {
    pub target: FieldReference,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cipher {
    ChaCha20Poly1305,
    Aes128Gcm,
    Aes256Gcm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoSegment {
    /// Whether binding this template requires a password.
    pub has_password: bool,
    pub cipher: Cipher,
    pub encryption: Vec<EncryptionDirectives>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionsSegment {
    /// `None` omits the assignment. `Some(false)` emits an explicit false value.
    pub separate_length_field: Option<bool>,
}
