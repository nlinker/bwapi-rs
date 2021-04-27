// todo
pub struct GameType;

// required for ffi layer
unsafe impl cxx::ExternType for GameType {
    type Id = cxx::type_id!("BWAPI::GameType");
    type Kind = cxx::kind::Trivial;
}
