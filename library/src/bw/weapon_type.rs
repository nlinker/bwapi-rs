#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaponType {
    Gauss_Rifle = 0,
    Gauss_Rifle_Jim_Raynor = 1,
    C_10_Canister_Rifle = 2,
    C_10_Canister_Rifle_Sarah_Kerrigan = 3,
    Fragmentation_Grenade = 4,
    Fragmentation_Grenade_Jim_Raynor = 5,
    Spider_Mines = 6,
    Twin_Autocannons = 7,
    Hellfire_Missile_Pack = 8,
    Twin_Autocannons_Alan_Schezar = 9,
    Hellfire_Missile_Pack_Alan_Schezar = 10,
    Arclite_Cannon = 11,
    Arclite_Cannon_Edmund_Duke = 12,
    Fusion_Cutter = 13,

    Gemini_Missiles = 15,
    Burst_Lasers = 16,
    Gemini_Missiles_Tom_Kazansky = 17,
    Burst_Lasers_Tom_Kazansky = 18,
    ATS_Laser_Battery = 19,
    ATA_Laser_Battery = 20,
    ATS_Laser_Battery_Hero = 21,
    ATA_Laser_Battery_Hero = 22,
    ATS_Laser_Battery_Hyperion = 23,
    ATA_Laser_Battery_Hyperion = 24,
    Flame_Thrower = 25,
    Flame_Thrower_Gui_Montag = 26,
    Arclite_Shock_Cannon = 27,
    Arclite_Shock_Cannon_Edmund_Duke = 28,
    Longbolt_Missile = 29,
    Yamato_Gun = 30,
    Nuclear_Strike = 31,
    Lockdown = 32,
    EMP_Shockwave = 33,
    Irradiate = 34,
    Claws = 35,
    Claws_Devouring_One = 36,
    Claws_Infested_Kerrigan = 37,
    Needle_Spines = 38,
    Needle_Spines_Hunter_Killer = 39,
    Kaiser_Blades = 40,
    Kaiser_Blades_Torrasque = 41,
    Toxic_Spores = 42,
    Spines = 43,

    Acid_Spore = 46,
    Acid_Spore_Kukulza = 47,
    Glave_Wurm = 48,
    Glave_Wurm_Kukulza = 49,

    Seeker_Spores = 52,
    Subterranean_Tentacle = 53,
    Suicide_Infested_Terran = 54,
    Suicide_Scourge = 55,
    Parasite = 56,
    Spawn_Broodlings = 57,
    Ensnare = 58,
    Dark_Swarm = 59,
    Plague = 60,
    Consume = 61,
    Particle_Beam = 62,

    Psi_Blades = 64,
    Psi_Blades_Fenix = 65,
    Phase_Disruptor = 66,
    Phase_Disruptor_Fenix = 67,

    Psi_Assault = 69,
    Psionic_Shockwave = 70,
    Psionic_Shockwave_TZ_Archon = 71,

    Dual_Photon_Blasters = 73,
    Anti_Matter_Missiles = 74,
    Dual_Photon_Blasters_Mojo = 75,
    Anti_Matter_Missiles_Mojo = 76,
    Phase_Disruptor_Cannon = 77,
    Phase_Disruptor_Cannon_Danimoth = 78,
    Pulse_Cannon = 79,
    STS_Photon_Cannon = 80,
    STA_Photon_Cannon = 81,
    Scarab = 82,
    Stasis_Field = 83,
    Psionic_Storm = 84,
    Warp_Blades_Zeratul = 85,
    Warp_Blades_Hero = 86,

    Platform_Laser_Battery = 92,
    Independant_Laser_Battery = 93,

    Twin_Autocannons_Floor_Trap = 96,
    Hellfire_Missile_Pack_Wall_Trap = 97,
    Flame_Thrower_Wall_Trap = 98,
    Hellfire_Missile_Pack_Floor_Trap = 99,

    Neutron_Flare = 100,
    Disruption_Web = 101,
    Restoration = 102,
    Halo_Rockets = 103,
    Corrosive_Acid = 104,
    Mind_Control = 105,
    Feedback = 106,
    Optical_Flare = 107,
    Maelstrom = 108,
    Subterranean_Spines = 109,

    Warp_Blades = 111,
    C_10_Canister_Rifle_Samir_Duran = 112,
    C_10_Canister_Rifle_Infested_Duran = 113,
    Dual_Photon_Blasters_Artanis = 114,
    Anti_Matter_Missiles_Artanis = 115,
    C_10_Canister_Rifle_Alexei_Stukov = 116,

    None = 130,
    Unknown,
    MAX,
}

unsafe impl cxx::ExternType for WeaponType {
    type Id = cxx::type_id!("BWAPI::WeaponType");
    type Kind = cxx::kind::Trivial;
}

// const WeaponType::set & 	allWeaponTypes ()
// const WeaponType::set & 	normalWeaponTypes ()
// const WeaponType::set & 	specialWeaponTypes ()