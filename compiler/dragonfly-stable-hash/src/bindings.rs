

// A type used to work around `Span` not being visible in this crate. It is the same layout as
// `Span`.
pub struct RawSpan(pub u32,pub u16,pub u16);

// A type used to work around `DefId` not being visible in this crate. It is the same size as
// `DefId`.
pub struct RawDefId(pub u32,pub u32);

// A type used to work around `DefPathHash` not being visible in this crate. It is the same size as
// `DefPathHash`.
pub struct RawDefPathHash(pub [u8;16]);



