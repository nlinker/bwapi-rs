use crate::ffi::PlayerInterface;

#[repr(i32)]
#[allow(non_camel_case_types)]
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
    type Id = cxx::type_id!("BWAPI::PlayerTypes::Enum::Enum");
    type Kind = cxx::kind::Trivial;
}
