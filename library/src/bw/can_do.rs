use typed_builder::TypedBuilder;

// region CanIssueCommandArg::builder
#[derive(TypedBuilder)]
pub struct CanIssueCommandFlags {
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
// endregion

// region CanIssueCommandGroupedArg::builder
#[derive(TypedBuilder)]
pub struct CanIssueCommandGroupedFlags {
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
// endregion

// region CanCheck3Arg::builder
#[derive(TypedBuilder)]
pub struct CanCheck3Flags {
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}
// endregion

#[derive(TypedBuilder)]
pub struct CanCheck4Flags {
    #[builder(default = true)]
    pub check_can_target_unit: bool,
    #[builder(default = true)]
    pub check_can_issue_command_type: bool,
    #[builder(default = true)]
    pub check_commandibility_grouped: bool,
    #[builder(default = true)]
    pub check_commandibility: bool,
}
