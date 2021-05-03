#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Flag {
    CompleteMapInformation = 0,
    UserInput = 1,
    Max,
}

// required for ffi layer
unsafe impl cxx::ExternType for Flag {
    type Id = cxx::type_id!("BWAPI::Flag::Enum");
    type Kind = cxx::kind::Trivial;
}
