use crate::bw::unit_type::UnitType::{*, self};
use crate::bw::upgrade_type::UpgradeType;

// TODO Here goes the `bwapilib` implementation in Rust
// the aim of the lib is to back bw module with static data so that as much as possible
// can be calculated without access BWAPI runtime, e.g. Position::get_approx_distance,
// TechType::mineral_price/gas_price/what_uses/research_time/energy_cost etc
//
// Then Client API can be made on top of it.

const _DEFAULT_ORE_COST_BASE: [i32; UpgradeType::MAX as usize] =    // same as default gas cost base
    [
        100, 100, 150, 150, 150, 100, 150, 100, 100, 100, 100, 100, 100, 100, 100, 200, 150, 100, 200, 150, 100, 150, 200, 150, 200, 150, 150, 100, 200,
        150, 150, 150, 150, 150, 150, 200, 200, 200, 150, 150, 150, 100, 200, 100, 150, 0, 0, 100, 100, 150, 150, 150, 150, 200, 100, 0, 0, 0, 0, 0, 0, 0, 0
    ];

const _DEFAULT_TIME_COST_BASE: [i32; UpgradeType::MAX as usize] =
    [
        4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 4000, 1500, 1500, 0, 2500,
        2500, 2500, 2500, 2500, 2400, 2000, 2000, 1500, 1500, 1500, 1500, 2500, 2500, 2500, 2000, 2500, 2500, 2500, 2000,
        2000, 2500, 2500, 2500, 1500, 2500, 0, 0, 2500, 2500, 2500, 2500, 2500, 2000, 2000, 2000, 0, 0, 0, 0, 0, 0, 0, 0
    ];

pub(crate) const _REQUIREMENTS: [[UnitType; UpgradeType::MAX as usize]; 3] =
    [
        // Level 1
        [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, Zerg_Hive, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, Terran_Armory, None, None, None, None, None, None, None, None],
        // Level 2
        [Terran_Science_Facility, Terran_Science_Facility, Terran_Science_Facility, Zerg_Lair, Zerg_Lair, Protoss_Templar_Archives, Protoss_Fleet_Beacon,
            Terran_Science_Facility, Terran_Science_Facility, Terran_Science_Facility, Zerg_Lair, Zerg_Lair, Zerg_Lair, Protoss_Templar_Archives,
            Protoss_Fleet_Beacon, Protoss_Cybernetics_Core, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None],
        // Level 3
        [Terran_Science_Facility, Terran_Science_Facility, Terran_Science_Facility, Zerg_Hive, Zerg_Hive, Protoss_Templar_Archives, Protoss_Fleet_Beacon,
            Terran_Science_Facility, Terran_Science_Facility, Terran_Science_Facility, Zerg_Hive, Zerg_Hive, Zerg_Hive, Protoss_Templar_Archives,
            Protoss_Fleet_Beacon, Protoss_Cybernetics_Core, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None],
    ];

