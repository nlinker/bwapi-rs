#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DamageType {
    Independent,
    Explosive,
    Concussive,
    Normal,
    Ignore_Armor,
    None,
    Unknown,
    MAX
}

// required for ffi layer
unsafe impl cxx::ExternType for DamageType {
    type Id = cxx::type_id!("BWAPI::DamageType");
    type Kind = cxx::kind::Trivial;
}
