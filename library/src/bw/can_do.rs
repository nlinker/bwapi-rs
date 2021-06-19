use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct CommandFlags {
    #[builder(default = true)]
    pub check_can_use_tech_position_on_positions: bool,
    #[builder(default = true)]
    pub check_can_use_tech_unit_on_units: bool,
    #[builder(default = true)]
    pub check_can_build_unit_type: bool,
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}

#[derive(TypedBuilder)]
pub struct CommandGroupedFlags {
    #[builder(default = true)]
    pub check_can_use_tech_position_on_positions: bool,
    #[builder(default = true)]
    pub check_can_use_tech_unit_on_units: bool,
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility_grouped: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}

#[derive(TypedBuilder)]
pub struct Common3uFlags {
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}

#[derive(TypedBuilder)]
pub struct Common3pFlags {
    #[builder(default = true)]
    pub check_targets_positions: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}

#[derive(TypedBuilder)]
pub struct Common4igFlags {
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility_grouped: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}

#[derive(TypedBuilder)]
pub struct Common4piFlags {
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_position: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}

#[derive(TypedBuilder)]
pub struct Common4tiFlags {
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_targets_type: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}

#[derive(TypedBuilder)]
pub struct Common4uiFlags {
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_targets_units: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}

/// The implementations of the builders are synthesized, but IDE doesn't know of this.
/// We, therefore, use this artificial definitions to keep IDE's mouth shut.
#[cfg(ignore_this)]
mod ext {
    use super::*;

    pub trait Builder<T> {
        fn build(self) -> T { unreachable!("IDE stub") }
    }

    impl CommandFlags {
        pub fn builder() -> impl Builder<Self> { unreachable!("IDE stub") }
    }
    impl CommandGroupedFlags {
        pub fn builder() -> impl Builder<Self> { unreachable!("IDE stub") }
    }
    impl Common3uFlags {
        pub fn builder() -> impl Builder<Self> { unreachable!("IDE stub") }
    }
    impl Common3pFlags {
        pub fn builder() -> impl Builder<Self> { unreachable!("IDE stub") }
    }
    impl Common4igFlags {
        pub fn builder() -> impl Builder<Self> { unreachable!("IDE stub") }
    }
    impl Common4piFlags {
        pub fn builder() -> impl Builder<Self> { unreachable!("IDE stub") }
    }
    impl Common4tiFlags {
        pub fn builder() -> impl Builder<Self> { unreachable!("IDE stub") }
    }
    impl Common4uiFlags {
        pub fn builder() -> impl Builder<Self> { unreachable!("IDE stub") }
    }
}
