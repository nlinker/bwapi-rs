#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerType {
    None = 0,
    Computer,
    Player,
    RescuePassive,
    RescueActive,
    EitherPreferComputer,
    EitherPreferHuman,
    Neutral,
    Closed,
    Observer,
    PlayerLeft,
    ComputerLeft,
    Unknown,
    MAX,
}

unsafe impl cxx::ExternType for PlayerType {
    type Id = cxx::type_id!("BWAPI::PlayerType");
    type Kind = cxx::kind::Trivial;
}
