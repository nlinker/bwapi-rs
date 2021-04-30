use crate::bw::unit::Unit;
// use crate::bw::position::{Position, TilePosition};
// use crate::bw::unit_type::UnitType;
// use crate::bw::tech_type::TechType;
// use crate::bw::upgrade_type::UpgradeType;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnitCommandType {
    Attack_Move = 0,
    Attack_Unit,
    Build,
    Build_Addon,
    Train,
    Morph,
    Research,
    Upgrade,
    Set_Rally_Position,
    Set_Rally_Unit,
    Move,
    Patrol,
    Hold_Position,
    Stop,
    Follow,
    Gather,
    Return_Cargo,
    Repair,
    Burrow,
    Unburrow,
    Cloak,
    Decloak,
    Siege,
    Unsiege,
    Lift,
    Land,
    Load,
    Unload,
    Unload_All,
    Unload_All_Position,
    Right_Click_Position,
    Right_Click_Unit,
    Halt_Construction,
    Cancel_Construction,
    Cancel_Addon,
    Cancel_Train,
    Cancel_Train_Slot,
    Cancel_Morph,
    Cancel_Research,
    Cancel_Upgrade,
    Use_Tech,
    Use_Tech_Position,
    Use_Tech_Unit,
    Place_COP,
    None,
    Unknown,
    MAX
}

impl Default for UnitCommandType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone)]
pub struct UnitCommand {
    unit: Unit,
    uc_type: UnitCommandType,
    target: Unit,
    x: i32,
    y: i32,
    extra: i32,
}

// #[derive(Debug)]
// pub enum UnitCommand {
//     AttackUnit { unit: Unit, target: Unit, shift_queue_command: bool },
//     AttackMove { unit: Unit, target: Position, shift_queue_command: bool },
//     Build { unit: Unit, target: TilePosition, utype: UnitType },
//     BuildAddon { unit: Unit, utype: UnitType },
//     Train { unit: Unit, utype: UnitType },
//     Morph { unit: Unit, utype: UnitType },
//     Research { unit: Unit, tech: TechType },
//     Upgrade { unit: Unit, upgrade: UpgradeType },
//     SetRallyPosition { unit: Unit, target: Position },
//     SetRallyUnit { unit: Unit, target: Unit },
//     Move { unit: Unit, target: Position, shift_queue_command: bool },
//     Patrol { unit: Unit, target: Position, shift_queue_command: bool },
//     HoldPosition { unit: Unit, shift_queue_command: bool },
//     Stop { unit: Unit, shift_queue_command: bool },
//     Follow { unit: Unit, target: Unit, shift_queue_command: bool },
//     Gather { unit: Unit, target: Unit, shift_queue_command: bool },
//     ReturnCargo { unit: Unit, shift_queue_command: bool },
//     Repair { unit: Unit, target: Unit, shift_queue_command: bool },
//     Burrow { unit: Unit },
//     Unburrow { unit: Unit },
//     Cloak { unit: Unit },
//     Decloak { unit: Unit },
//     Siege { unit: Unit },
//     Unsiege { unit: Unit },
//     Lift { unit: Unit },
//     Land { unit: Unit, target: TilePosition },
//     Load { unit: Unit, target: Unit, shift_queue_command: bool },
//     Unload { unit: Unit, target: Unit },
//     UnloadAll { unit: Unit, shift_queue_command: bool },
//     UnloadAllPosition { unit: Unit, target: Position, shift_queue_command: bool },
//     RightClickUnit { unit: Unit, target: Unit, shift_queue_command: bool },
//     RightClickPosition { unit: Unit, target: Position, shift_queue_command: bool },
//     HaltConstruction { unit: Unit },
//     CancelConstruction { unit: Unit },
//     CancelAddon { unit: Unit },
//     CancelTrain { unit: Unit, slot: i32 },
//     CancelMorph { unit: Unit },
//     CancelResearch { unit: Unit },
//     CancelUpgrade { unit: Unit },
//     UseTech { unit: Unit, tech: TechType },
//     UseTechUnit { unit: Unit, tech: TechType, target: Unit },
//     UseTechPosition { unit: Unit, tech: TechType, target: Position },
//     PlaceCOP { unit: Unit, target: TilePosition },
//     None,
//     Unknown,
// }

// required for ffi layer
unsafe impl cxx::ExternType for UnitCommand {
    type Id = cxx::type_id!("BWAPI::UnitCommand");
    type Kind = cxx::kind::Trivial;
}
