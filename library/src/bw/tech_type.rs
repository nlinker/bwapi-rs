
#[allow(non_camel_case_types)]
pub enum TechType {
    Stim_Packs = 0,
    Lockdown,
    EMP_Shockwave,
    Spider_Mines,
    Scanner_Sweep,
    Tank_Siege_Mode,
    Defensive_Matrix,
    Irradiate,
    Yamato_Gun,
    Cloaking_Field,
    Personnel_Cloaking,
    Burrowing,
    Infestation,
    Spawn_Broodlings,
    Dark_Swarm,
    Plague,
    Consume,
    Ensnare,
    Parasite,
    Psionic_Storm,
    Hallucination,
    Recall,
    Stasis_Field,
    Archon_Warp,
    Restoration,
    Disruption_Web,
    Unused_26,
    Mind_Control,
    Dark_Archon_Meld,
    Feedback,
    Optical_Flare,
    Maelstrom,
    Lurker_Aspect,
    Unused_33,
    Healing,

    None = 44,
    Nuclear_Strike,
    Unknown,
    MAX
}

// required for ffi layer
unsafe impl cxx::ExternType for TechType {
    type Id = cxx::type_id!("BWAPI::TechType");
    type Kind = cxx::kind::Trivial;
}

// TechType functions
// fn getRace() -> Race
// fn mineralPrice() -> int
// fn gasPrice() -> int
// fn researchTime() -> int
// fn energyCost() -> int
// fn whatResearches() -> UnitType
// fn getWeapon() -> WeaponType
// fn targetsUnit() -> bool
// fn targetsPosition() -> bool
// fn whatUses() -> UnitType::set&
// fn getOrder() -> Order
// fn requiredUnit() -> UnitType