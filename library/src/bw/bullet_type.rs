#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BulletType {
    Melee = 0,

    Fusion_Cutter_Hit = 141,
    Gauss_Rifle_Hit,
    C_10_Canister_Rifle_Hit,
    Gemini_Missiles,
    Fragmentation_Grenade,
    Longbolt_Missile,
    Unused_Lockdown,
    ATS_ATA_Laser_Battery,
    Burst_Lasers,
    Arclite_Shock_Cannon_Hit,
    EMP_Missile,
    Dual_Photon_Blasters_Hit,
    Particle_Beam_Hit,
    Anti_Matter_Missile,
    Pulse_Cannon,
    Psionic_Shockwave_Hit,
    Psionic_Storm,
    Yamato_Gun,
    Phase_Disruptor,
    STA_STS_Cannon_Overlay,
    Sunken_Colony_Tentacle,
    Venom_Unused,
    Acid_Spore,
    Plasma_Drip_Unused,
    Glave_Wurm,
    Seeker_Spores,
    Queen_Spell_Carrier,
    Plague_Cloud,
    Consume,
    Ensnare,
    Needle_Spine_Hit,
    Invisible,

    Optical_Flare_Grenade = 201,
    Halo_Rockets,
    Subterranean_Spines,
    Corrosive_Acid_Shot,
    Corrosive_Acid_Hit,
    Neutron_Flare,

    None = 209,
    Unknown,
    MAX,
}

// required for ffi layer
unsafe impl cxx::ExternType for BulletType {
    type Id = cxx::type_id!("BWAPI::BulletType");
    type Kind = cxx::kind::Trivial;
}
