pub struct TechType;

// required for ffi layer
unsafe impl cxx::ExternType for TechType {
    type Id = cxx::type_id!("BWAPI::TechType");
    type Kind = cxx::kind::Trivial;
}
