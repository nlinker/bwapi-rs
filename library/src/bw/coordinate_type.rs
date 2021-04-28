// todo

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub enum CoordinateType {
    None = 0,
    Screen = 1,
    Map = 2,
    Mouse = 3,
}

// required for ffi layer
unsafe impl cxx::ExternType for CoordinateType {
    type Id = cxx::type_id!("BWAPI::CoordinateType");
    type Kind = cxx::kind::Trivial;
}
