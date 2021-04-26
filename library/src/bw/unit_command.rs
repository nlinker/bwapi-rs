pub struct UnitCommand;

// required for ffi layer
unsafe impl cxx::ExternType for UnitCommand {
    type Id = cxx::type_id!("BWAPI::UnitCommand");
    type Kind = cxx::kind::Trivial;
}
