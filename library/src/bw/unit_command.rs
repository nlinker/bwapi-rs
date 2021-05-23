use crate::bw::position::{Position, TilePosition, PositionLike};
use crate::bw::tech_type::TechType;
use crate::bw::unit::Unit;
use crate::bw::unit_type::UnitType;
use crate::bw::upgrade_type::UpgradeType;

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
// #[derive(TypedBuilder)]
pub struct UnitCommand {
    pub unit: Unit,
    pub ctype: UnitCommandType,
    // #[builder(default = None)]
    pub target: Option<Unit>,
    // #[builder(default = 0)]
    pub x: i32,
    // #[builder(default = 0)]
    pub y: i32,
    // #[builder(default = 0)]
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
            .target(target.clone().into())
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn build(unit: &Unit, target: TilePosition, unit_type: UnitType) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Build)
            .x(target.x)
            .y(target.y)
            .extra(unit_type as i32)
            .build()
    }
    pub fn build_addon(unit: &Unit, unit_type: UnitType) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Build_Addon)
            .extra(unit_type as i32)
            .build()
    }
    pub fn train(unit: &Unit, unit_type: UnitType) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Train)
            .extra(unit_type as i32)
            .build()
    }
    pub fn morph(unit: &Unit, unit_type: UnitType) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Morph)
            .extra(unit_type as i32)
            .build()
    }
    pub fn research(unit: &Unit, tech_type: TechType) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Research)
            .extra(tech_type as i32)
            .build()
    }
    pub fn upgrade(unit: &Unit, upgrade_type: UpgradeType) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Upgrade)
            .extra(upgrade_type as i32)
            .build()
    }
    pub fn set_rally_position(unit: &Unit, target: Position) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Set_Rally_Position)
            .x(target.x)
            .y(target.y)
            .build()
    }
    pub fn set_rally_unit(unit: &Unit, target: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Set_Rally_Unit)
            .target(target.clone().into())
            .build()
    }
    pub fn move_(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Move)
            .x(target.x)
            .y(target.y)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn patrol(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Patrol)
            .x(target.x)
            .y(target.y)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn hold_position(unit: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Hold_Position)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn stop(unit: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Stop)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn follow(unit: &Unit, target: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Follow)
            .target(target.clone().into())
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn gather(unit: &Unit, target: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Gather)
            .target(target.clone().into())
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn return_cargo(unit: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Return_Cargo)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn repair(unit: &Unit, target: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Repair)
            .target(target.clone().into())
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn burrow(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Burrow)
            .build()
    }
    pub fn unburrow(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Unburrow)
            .build()
    }
    pub fn cloak(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Cloak)
            .build()
    }
    pub fn decloak(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Decloak)
            .build()
    }
    pub fn siege(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Siege)
            .build()
    }
    pub fn unsiege(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Unsiege)
            .build()
    }
    pub fn lift(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Lift)
            .build()
    }
    pub fn land(unit: &Unit, target: TilePosition) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Land)
            .x(target.x)
            .y(target.y)
            .build()
    }
    pub fn load(unit: &Unit, target: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Load)
            .target(target.clone().into())
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn unload(unit: &Unit, target: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Unload)
            .target(target.clone().into())
            .build()
    }
    pub fn unload_all(unit: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Unload_All)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn unload_all_position(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Unload_All_Position)
            .x(target.x)
            .y(target.y)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn right_click_position(unit: &Unit, target: Position, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Right_Click_Position)
            .x(target.x)
            .y(target.y)
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn right_click_unit(unit: &Unit, target: &Unit, shift_queue_command: bool) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Right_Click_Unit)
            .target(target.clone().into())
            .extra(shift_queue_command as i32)
            .build()
    }
    pub fn halt_construction(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Halt_Construction)
            .build()
    }
    pub fn cancel_construction(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Cancel_Construction)
            .build()
    }
    pub fn cancel_addon(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Cancel_Addon)
            .build()
    }
    pub fn cancel_train(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Cancel_Train)
            .build()
    }
    pub fn cancel_train_slot(unit: &Unit, slot: i32) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Cancel_Train_Slot)
            .extra(slot)
            .build()
    }
    pub fn cancel_morph(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Cancel_Morph)
            .build()
    }
    pub fn cancel_research(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Cancel_Research)
            .build()
    }
    pub fn cancel_upgrade(unit: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Cancel_Upgrade)
            .build()
    }
    pub fn use_tech(unit: &Unit, tech_type: TechType) -> UnitCommand {
        let command_type = match tech_type {
            TechType::Burrowing => if unit.is_burrowed() { UnitCommandType::Unburrow } else { UnitCommandType::Burrow },
            TechType::Cloaking_Field | TechType::Personnel_Cloaking =>
                if unit.is_cloaked() { UnitCommandType::Decloak } else { UnitCommandType::Cloak },
            TechType::Tank_Siege_Mode => if unit.is_sieged() { UnitCommandType::Unsiege } else { UnitCommandType::Siege },
            _ => UnitCommandType::Use_Tech,
        };
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(command_type)
            .extra(tech_type as i32)
            .build()
    }
    pub fn use_tech_position(unit: &Unit, tech_type: TechType, target: Position) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Use_Tech_Position)
            .x(target.x)
            .y(target.y)
            .extra(tech_type as i32)
            .build()
    }
    pub fn use_tech_unit(unit: &Unit, tech_type: TechType, target: &Unit) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Use_Tech_Unit)
            .target(target.clone().into())
            .extra(tech_type as i32)
            .build()
    }
    pub fn place_cop(unit: &Unit, target: TilePosition) -> UnitCommand {
        UnitCommand::builder()
            .unit(unit.clone())
            .ctype(UnitCommandType::Place_COP)
            .x(target.x)
            .y(target.y)
            .build()
    }

    pub fn get_target_position(&self) -> Position {
        match self.ctype {
            UnitCommandType::Build |
            UnitCommandType::Land |
            UnitCommandType::Place_COP =>
                (self.x, self.y).to_tile_position().to_position(),
            _ =>
                (self.x, self.y).to_position(),
        }
    }
    pub fn get_target_tile_position(&self) -> TilePosition {
        match self.ctype {
            UnitCommandType::Build |
            UnitCommandType::Land |
            UnitCommandType::Place_COP =>
                (self.x, self.y).to_position().to_tile_position(),
            _ =>
                (self.x, self.y).to_tile_position(),
        }
    }
    pub fn get_unit_type(&self) -> Option<UnitType> {
        match self.ctype {
            UnitCommandType::Build |
            UnitCommandType::Build_Addon |
            UnitCommandType::Train |
            UnitCommandType::Morph => Some(UnitType::from(self.extra as u32)),
            _ => None
        }
    }
    pub fn get_tech_type(&self) -> Option<TechType> {
        match self.ctype {
            UnitCommandType::Research |
            UnitCommandType::Use_Tech |
            UnitCommandType::Use_Tech_Position |
            UnitCommandType::Use_Tech_Unit => Some(TechType::from(self.extra as u32)),
            _ => None
        }
    }
    pub fn get_upgrade_type(&self) -> Option<UpgradeType> {
        if self.ctype == UnitCommandType::Upgrade {
            Some(UpgradeType::from(self.extra as u32))
        } else {
            None
        }
    }
    pub fn get_slot(&self) -> Option<i32> {
        if self.ctype == UnitCommandType::Cancel_Train_Slot {
            Some(self.extra)
        } else {
            None
        }
    }
    pub fn is_queued(&self) -> bool {
        // extract shift_queue_command back
        match self.ctype {
            UnitCommandType::Attack_Move |
            UnitCommandType::Attack_Unit |
            UnitCommandType::Move |
            UnitCommandType::Patrol |
            UnitCommandType::Hold_Position |
            UnitCommandType::Stop |
            UnitCommandType::Follow |
            UnitCommandType::Gather |
            UnitCommandType::Return_Cargo |
            UnitCommandType::Repair |
            UnitCommandType::Load |
            UnitCommandType::Unload_All |
            UnitCommandType::Unload_All_Position |
            UnitCommandType::Right_Click_Position |
            UnitCommandType::Right_Click_Unit => self.extra != 0,
            _ => false,
        }
    }
}

// required for ffi layer
unsafe impl cxx::ExternType for UnitCommand {
    type Id = cxx::type_id!("BWAPI::UnitCommand");
    type Kind = cxx::kind::Trivial;
}

impl UnitCommand {
    ///
    /// Create a builder for building `UnitCommand`.
    /// On the builder, call `.unit(...)`, `.ctype(...)`, `.target(...)`(optional), `.x(...)`(optional), `.y(...)`(optional), `.extra(...)`(optional) to set the values of the fields.
    /// Finally, call `.build()` to create the instance of `UnitCommand`.
    ///
    #[allow(dead_code)]
    pub fn builder() -> ext::UnitCommandBuilder<((), (), (), (), (), ())> {
        ext::UnitCommandBuilder {
            fields: ((), (), (), (), (), ()),
            _phantom: core::default::Default::default(),
        }
    }
}

mod ext {
    use crate::bw::unit::Unit;
    use crate::bw::unit_command::{UnitCommand, UnitCommandType};

    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct UnitCommandBuilder<TypedBuilderFields> {
        pub(crate) fields: TypedBuilderFields,
        pub(crate) _phantom: (),
    }

    impl<TypedBuilderFields> Clone for UnitCommandBuilder<TypedBuilderFields>
        where
            TypedBuilderFields: Clone,
    {
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                _phantom: Default::default(),
            }
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub trait UnitCommandBuilder_Optional<T> {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
    }

    impl<T> UnitCommandBuilder_Optional<T> for () {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
            default()
        }
    }

    impl<T> UnitCommandBuilder_Optional<T> for (T, ) {
        fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
            self.0
        }
    }

    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__ctype, __target, __x, __y, __extra> UnitCommandBuilder<((), __ctype, __target, __x, __y, __extra)> {
        pub fn unit(self, unit: Unit) -> UnitCommandBuilder<((Unit, ), __ctype, __target, __x, __y, __extra)> {
            let unit = (unit, );
            let (_, ctype, target, x, y, extra) = self.fields;
            UnitCommandBuilder {
                fields: (unit, ctype, target, x, y, extra),
                _phantom: self._phantom,
            }
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum UnitCommandBuilder_Error_Repeated_field_unit {}

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__ctype, __target, __x, __y, __extra> UnitCommandBuilder<((Unit, ), __ctype, __target, __x, __y, __extra)> {
        #[deprecated(note = "Repeated field unit")]
        pub fn unit(
            self,
            _: UnitCommandBuilder_Error_Repeated_field_unit,
        ) -> UnitCommandBuilder<((Unit, ), __ctype, __target, __x, __y, __extra)> {
            self
        }
    }

    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __target, __x, __y, __extra> UnitCommandBuilder<(__unit, (), __target, __x, __y, __extra)> {
        pub fn ctype(
            self,
            ctype: UnitCommandType,
        ) -> UnitCommandBuilder<(__unit, (UnitCommandType, ), __target, __x, __y, __extra)> {
            let ctype = (ctype, );
            let (unit, _, target, x, y, extra) = self.fields;
            UnitCommandBuilder {
                fields: (unit, ctype, target, x, y, extra),
                _phantom: self._phantom,
            }
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum UnitCommandBuilder_Error_Repeated_field_ctype {}

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __target, __x, __y, __extra>
    UnitCommandBuilder<(__unit, (UnitCommandType, ), __target, __x, __y, __extra)>
    {
        #[deprecated(note = "Repeated field ctype")]
        pub fn ctype(
            self,
            _: UnitCommandBuilder_Error_Repeated_field_ctype,
        ) -> UnitCommandBuilder<(__unit, (UnitCommandType, ), __target, __x, __y, __extra)> {
            self
        }
    }

    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __ctype, __x, __y, __extra> UnitCommandBuilder<(__unit, __ctype, (), __x, __y, __extra)> {
        pub fn target(
            self,
            target: Option<Unit>,
        ) -> UnitCommandBuilder<(__unit, __ctype, (Option<Unit>, ), __x, __y, __extra)> {
            let target = (target, );
            let (unit, ctype, _, x, y, extra) = self.fields;
            UnitCommandBuilder {
                fields: (unit, ctype, target, x, y, extra),
                _phantom: self._phantom,
            }
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum UnitCommandBuilder_Error_Repeated_field_target {}

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __ctype, __x, __y, __extra> UnitCommandBuilder<(__unit, __ctype, (Option<Unit>, ), __x, __y, __extra)> {
        #[deprecated(note = "Repeated field target")]
        pub fn target(
            self,
            _: UnitCommandBuilder_Error_Repeated_field_target,
        ) -> UnitCommandBuilder<(__unit, __ctype, (Option<Unit>, ), __x, __y, __extra)> {
            self
        }
    }

    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __ctype, __target, __y, __extra> UnitCommandBuilder<(__unit, __ctype, __target, (), __y, __extra)> {
        pub fn x(self, x: i32) -> UnitCommandBuilder<(__unit, __ctype, __target, (i32, ), __y, __extra)> {
            let x = (x, );
            let (unit, ctype, target, _, y, extra) = self.fields;
            UnitCommandBuilder {
                fields: (unit, ctype, target, x, y, extra),
                _phantom: self._phantom,
            }
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum UnitCommandBuilder_Error_Repeated_field_x {}

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __ctype, __target, __y, __extra> UnitCommandBuilder<(__unit, __ctype, __target, (i32, ), __y, __extra)> {
        #[deprecated(note = "Repeated field x")]
        pub fn x(
            self,
            _: UnitCommandBuilder_Error_Repeated_field_x,
        ) -> UnitCommandBuilder<(__unit, __ctype, __target, (i32, ), __y, __extra)> {
            self
        }
    }

    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __ctype, __target, __x, __extra> UnitCommandBuilder<(__unit, __ctype, __target, __x, (), __extra)> {
        pub fn y(self, y: i32) -> UnitCommandBuilder<(__unit, __ctype, __target, __x, (i32, ), __extra)> {
            let y = (y, );
            let (unit, ctype, target, x, _, extra) = self.fields;
            UnitCommandBuilder {
                fields: (unit, ctype, target, x, y, extra),
                _phantom: self._phantom,
            }
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum UnitCommandBuilder_Error_Repeated_field_y {}

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __ctype, __target, __x, __extra> UnitCommandBuilder<(__unit, __ctype, __target, __x, (i32, ), __extra)> {
        #[deprecated(note = "Repeated field y")]
        pub fn y(
            self,
            _: UnitCommandBuilder_Error_Repeated_field_y,
        ) -> UnitCommandBuilder<(__unit, __ctype, __target, __x, (i32, ), __extra)> {
            self
        }
    }

    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __ctype, __target, __x, __y> UnitCommandBuilder<(__unit, __ctype, __target, __x, __y, ())> {
        pub fn extra(self, extra: i32) -> UnitCommandBuilder<(__unit, __ctype, __target, __x, __y, (i32, ))> {
            let extra = (extra, );
            let (unit, ctype, target, x, y, _) = self.fields;
            UnitCommandBuilder {
                fields: (unit, ctype, target, x, y, extra),
                _phantom: self._phantom,
            }
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum UnitCommandBuilder_Error_Repeated_field_extra {}

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__unit, __ctype, __target, __x, __y> UnitCommandBuilder<(__unit, __ctype, __target, __x, __y, (i32, ))> {
        #[deprecated(note = "Repeated field extra")]
        pub fn extra(
            self,
            _: UnitCommandBuilder_Error_Repeated_field_extra,
        ) -> UnitCommandBuilder<(__unit, __ctype, __target, __x, __y, (i32, ))> {
            self
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum UnitCommandBuilder_Error_Missing_required_field_unit {}

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl<__ctype, __target, __x, __y, __extra> UnitCommandBuilder<((), __ctype, __target, __x, __y, __extra)> {
        #[deprecated(note = "Missing required field unit")]
        pub fn build(self, _: UnitCommandBuilder_Error_Missing_required_field_unit) -> UnitCommand {
            {
                panic!("explicit panic")
            };
        }
    }

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum UnitCommandBuilder_Error_Missing_required_field_ctype {}

    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl<__target, __x, __y, __extra> UnitCommandBuilder<((Unit, ), (), __target, __x, __y, __extra)> {
        #[deprecated(note = "Missing required field ctype")]
        pub fn build(self, _: UnitCommandBuilder_Error_Missing_required_field_ctype) -> UnitCommand {
            {
                panic!("explicit panic")
            };
        }
    }

    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<
        __target: UnitCommandBuilder_Optional<Option<Unit>>,
        __x: UnitCommandBuilder_Optional<i32>,
        __y: UnitCommandBuilder_Optional<i32>,
        __extra: UnitCommandBuilder_Optional<i32>,
    > UnitCommandBuilder<((Unit, ), (UnitCommandType, ), __target, __x, __y, __extra)>
    {
        pub fn build(self) -> UnitCommand {
            let (unit, ctype, target, x, y, extra) = self.fields;
            let unit = unit.0;
            let ctype = ctype.0;
            let target = UnitCommandBuilder_Optional::into_value(target, || None);
            let x = UnitCommandBuilder_Optional::into_value(x, || 0);
            let y = UnitCommandBuilder_Optional::into_value(y, || 0);
            let extra = UnitCommandBuilder_Optional::into_value(extra, || 0);
            UnitCommand {
                unit,
                ctype,
                target,
                x,
                y,
                extra,
            }
        }
    }
}
