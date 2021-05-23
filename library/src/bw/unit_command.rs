use crate::bw::unit::Unit;
use crate::bw::position::{Position, TilePosition};
use crate::bw::unit_type::UnitType;
use crate::bw::tech_type::TechType;
use crate::bw::upgrade_type::UpgradeType;
use typed_builder::TypedBuilder;

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    MAX,
}

impl Default for UnitCommandType {
    fn default() -> Self {
        Self::Unknown
    }
}

// required for ffi layer
unsafe impl cxx::ExternType for UnitCommandType {
    type Id = cxx::type_id!("BWAPI::UnitCommandType");
    type Kind = cxx::kind::Trivial;
}

#[derive(Debug, Clone)]
#[derive(TypedBuilder)]
pub struct UnitCommand {
    pub unit: Unit,
    pub ctype: UnitCommandType,
    #[builder(default = None)]
    pub target: Option<Unit>,
    #[builder(default = 0)]
    pub x: i32,
    #[builder(default = 0)]
    pub y: i32,
    #[builder(default = 0)]
    pub extra: i32,
}

impl UnitCommand {
    pub fn attack_move(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand {
        // `shift_queue_command as i32` works https://stackoverflow.com/a/55461702/5066426
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Attack_Move)
            .x(target.x)
            .y(target.y)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn attack_unit(unit: &Unit, target: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Attack_Unit)
            .target(Some(target.clone()))
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn build(unit: &Unit, target: TilePosition, utype: UnitType) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Build)
            .x(target.x)
            .y(target.y)
            .extra(utype as i32)
            .build()
    }
    pub fn build_addon(unit: &Unit, utype: UnitType) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Build_Addon)
            .extra(utype as i32)
            .build()
    }
    pub fn train(unit: &Unit, utype: UnitType) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Train).build() }
    pub fn morph(unit: &Unit, utype: UnitType) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Morph).build() }
    pub fn research(unit: &Unit, tech: TechType) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Research).build() }
    pub fn upgrade(unit: &Unit, upgrade: UpgradeType) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Upgrade).build() }
    pub fn set_rally_position(unit: &Unit, target: Position) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Set_Rally_Position).build() }
    pub fn set_rally_unit(unit: &Unit, target: Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Set_Rally_Unit).build() }
    pub fn move_(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Move).build() }
    pub fn patrol(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Patrol).build() }
    pub fn hold_position(unit: &Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Hold_Position).build() }
    pub fn stop(unit: &Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Stop).build() }
    pub fn follow(unit: &Unit, target: Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Follow).build() }
    pub fn gather(unit: &Unit, target: Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Gather).build() }
    pub fn return_cargo(unit: &Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Return_Cargo).build() }
    pub fn repair(unit: &Unit, target: Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Repair).build() }
    pub fn burrow(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Burrow).build() }
    pub fn unburrow(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Unburrow).build() }
    pub fn cloak(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Cloak).build() }
    pub fn decloak(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Decloak).build() }
    pub fn siege(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Siege).build() }
    pub fn unsiege(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Unsiege).build() }
    pub fn lift(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Lift).build() }
    pub fn land(unit: &Unit, target: TilePosition) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Land).build() }
    pub fn load(unit: &Unit, target: Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Load).build() }
    pub fn unload(unit: &Unit, target: Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Unload).build() }
    pub fn unload_all(unit: &Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Unload_All).build() }
    pub fn unload_all_position(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Unload_All_Position).build() }
    pub fn right_click_position(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Right_Click_Position).build() }
    pub fn right_click_unit(unit: &Unit, target: Unit, shift_queue_command: bool) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Right_Click_Unit).build() }
    pub fn halt_construction(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Halt_Construction).build() }
    pub fn cancel_construction(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Cancel_Construction).build() }
    pub fn cancel_addon(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Cancel_Addon).build() }
    pub fn cancel_train(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Cancel_Train).build() }
    pub fn cancel_train_slot(unit: &Unit, slot: i32) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Cancel_Train_Slot).build() }
    pub fn cancel_morph(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Cancel_Morph).build() }
    pub fn cancel_research(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Cancel_Research).build() }
    pub fn cancel_upgrade(unit: &Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Cancel_Upgrade).build() }
    pub fn use_tech(unit: &Unit, tech: TechType) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Use_Tech).build() }
    pub fn use_tech_position(unit: &Unit, tech: TechType, target: Position) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Use_Tech_Position).build() }
    pub fn use_tech_unit(unit: &Unit, tech: TechType, target: Unit) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Use_Tech_Unit).build() }
    pub fn place_cop(unit: &Unit, target: TilePosition) -> UnitCommand { UnitCommand::builder().unit(unit.clone()).ctype(UnitCommandType::Place_COP).build() }
}

// required for ffi layer
unsafe impl cxx::ExternType for UnitCommand {
    type Id = cxx::type_id!("BWAPI::UnitCommand");
    type Kind = cxx::kind::Trivial;
}

