#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameType {
    None = 0,
    Custom,
    Melee,
    Free_For_All,
    One_on_One,
    Capture_The_Flag,
    Greed,
    Slaughter,
    Sudden_Death,
    Ladder,
    Use_Map_Settings,
    Team_Melee,
    Team_Free_For_All,
    Team_Capture_The_Flag,
    Unknown_0x0E,
    Top_vs_Bottom,
    Iron_Man_Ladder,
    Pro_Gamer_League = 32,
    Unknown,
    MAX,
}

// required for ffi layer
unsafe impl cxx::ExternType for GameType {
    type Id = cxx::type_id!("BWAPI::GameType");
    type Kind = cxx::kind::Trivial;
}
