/// Identifies a field without carrying its parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldKind {
    Type,
    Length,
    PaddingLength,
    Version,
    Nonce,
    Extra,
    Reserved,
}

/// Parameters for one of the protocol's header fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Type(TypeField),
    Length(LengthField),
    PaddingLength(PaddingLengthField),
    Version(VersionField),
    Nonce(NonceField),
    Extra(ExtraField),
    Reserved(ReservedField),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeField {
    pub encrypted: bool,
    pub num_type_nums: usize,
    pub start_num: u8,
    pub normal_types_first: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthField {
    pub encrypted: bool,
    /// Width of the length field in bytes.
    pub length: usize,
    pub mac_covered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaddingLengthField {
    pub encrypted: bool,
    /// Width of the padding length field in bytes.
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionField {
    /// Width of the version field in bytes.
    pub length: usize,
    pub in_handshake: bool,
    pub encrypted: bool,
    pub major_val: u8,
    /// Present when the version field is two bytes wide.
    pub minor_val: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonceField {
    pub encrypted: bool,
    pub exists: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraField {
    pub encrypted: bool,
    /// Number of extra bytes in handshake messages.
    pub handshake_length: usize,
    /// Number of extra bytes in data messages.
    pub data_length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservedField {
    pub encrypted: bool,
    /// Number of reserved bytes.
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldOrder {
    pub overall_structure: OverallStructure,
    pub unencrypted: Vec<FieldKind>,
    pub encrypted: Vec<FieldKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverallStructure {
    /// Authenticate encrypted headers separately from the payload (Python value 0).
    SeparateHeaderMac,
    /// Authenticate encrypted headers and payload together (Python value 1).
    CombinedMac,
}
