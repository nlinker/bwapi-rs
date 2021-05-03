#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExplosionType {
    None = 0,
    Normal,
    Radial_Splash,
    Enemy_Splash,
    Lockdown,
    Nuclear_Missile,
    Parasite,
    Broodlings,
    EMP_Shockwave,
    Irradiate,
    Ensnare,
    Plague,
    Stasis_Field,
    Dark_Swarm,
    Consume,
    Yamato_Gun,
    Restoration,
    Disruption_Web,
    Corrosive_Acid,
    Mind_Control,
    Feedback,
    Optical_Flare,
    Maelstrom,
    Unused,
    Air_Splash,
    Unknown,
    MAX
}

// required for ffi layer
unsafe impl cxx::ExternType for ExplosionType {
    type Id = cxx::type_id!("BWAPI::ExplosionType");
    type Kind = cxx::kind::Trivial;
}
