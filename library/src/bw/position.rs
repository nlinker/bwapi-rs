#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct WalkPosition {
    pub x: i32,
    pub y: i32,
}

// required for ffi layer
unsafe impl cxx::ExternType for Position {
    type Id = cxx::type_id!("BWAPI::Position");
    type Kind = cxx::kind::Trivial;
}

// required for ffi layer
unsafe impl cxx::ExternType for TilePosition {
    type Id = cxx::type_id!("BWAPI::TilePosition");
    type Kind = cxx::kind::Trivial;
}

// required for ffi layer
unsafe impl cxx::ExternType for WalkPosition {
    type Id = cxx::type_id!("BWAPI::WalkPosition");
    type Kind = cxx::kind::Trivial;
}
