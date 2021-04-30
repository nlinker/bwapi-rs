#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Race {
    Zerg = 0,
    Terran,
    Protoss,
    Other,
    Unused,
    Select,
    Random,
    None,
    Unknown,
    MAX,
}

unsafe impl cxx::ExternType for Race {
    type Id = cxx::type_id!("BWAPI::Races::Enum::Enum");
    type Kind = cxx::kind::Trivial;
}
