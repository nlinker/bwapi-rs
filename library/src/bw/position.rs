#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct WalkPosition {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TilePosition {
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

pub trait PositionLike {
    fn to_position(&self) -> Position;
    fn to_walk_position(&self) -> WalkPosition;
    fn to_tile_position(&self) -> TilePosition;
}

impl PositionLike for (i32, i32) {
    fn to_position(&self) -> Position {
        Position { x: self.0, y: self.1 }
    }
    fn to_walk_position(&self) -> WalkPosition {
        WalkPosition { x: self.0, y: self.1 }
    }
    fn to_tile_position(&self) -> TilePosition {
        TilePosition { x: self.0, y: self.1 }
    }
}

impl PositionLike for Position {
    fn to_position(&self) -> Position {
        (*self).into()
    }
    fn to_walk_position(&self) -> WalkPosition {
        (*self).into()
    }
    fn to_tile_position(&self) -> TilePosition {
        (*self).into()
    }
}

impl PositionLike for WalkPosition {
    fn to_position(&self) -> Position {
        (*self).into()
    }
    fn to_walk_position(&self) -> WalkPosition {
        (*self).into()
    }
    fn to_tile_position(&self) -> TilePosition {
        (*self).into()
    }
}

impl PositionLike for TilePosition {
    fn to_position(&self) -> Position {
        (*self).into()
    }
    fn to_walk_position(&self) -> WalkPosition {
        (*self).into()
    }
    fn to_tile_position(&self) -> TilePosition {
        (*self).into()
    }
}

impl From<TilePosition> for Position {
    fn from(p: TilePosition) -> Self {
        Self {
            x: p.x * 32,
            y: p.y * 32,
        }
    }
}

impl From<Position> for TilePosition {
    fn from(p: Position) -> Self {
        Self {
            x: p.x / 32,
            y: p.y / 32,
        }
    }
}

impl From<WalkPosition> for Position {
    fn from(p: WalkPosition) -> Self {
        Self { x: p.x * 8, y: p.y * 8 }
    }
}

impl From<Position> for WalkPosition {
    fn from(p: Position) -> Self {
        Self { x: p.x / 8, y: p.y / 8 }
    }
}

impl From<TilePosition> for WalkPosition {
    fn from(p: TilePosition) -> Self {
        Self { x: p.x * 4, y: p.y * 4 }
    }
}

impl From<WalkPosition> for TilePosition {
    fn from(p: WalkPosition) -> Self {
        Self { x: p.x / 4, y: p.y / 4 }
    }
}

// namespace Positions
// {
//     const Position Invalid{32000 / POSITION_SCALE, 32000 / POSITION_SCALE};
//     const Position None{32000 / POSITION_SCALE, 32032 / POSITION_SCALE};
//     const Position Unknown{32000 / POSITION_SCALE, 32064 / POSITION_SCALE};
//     const Position Origin{0, 0};
// }
