#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnitSizeType {
    Independent = 0,
    Small,
    Medium,
    Large,
    None,
    Unknown,
    MAX,
}

// required for ffi layer
unsafe impl cxx::ExternType for UnitSizeType {
    type Id = cxx::type_id!("BWAPI::UnitSizeType");
    type Kind = cxx::kind::Trivial;
}
