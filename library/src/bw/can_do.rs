use typed_builder::TypedBuilder;

// #[derive(TypedBuilder)]
// pub struct CanIssueCommandArg {
//     pub unit_command: UnitCommand,
//     #[builder(default = true)]
//     pub check_can_use_tech_position_on_positions: bool,
//     #[builder(default = true)]
//     pub check_can_use_tech_unit_on_units: bool,
//     #[builder(default = true)]
//     pub check_can_build_unit_type: bool,
//     #[builder(default = true)]
//     pub check_can_target_unit: bool,
//     #[builder(default = true)]
//     pub check_can_issue_command_type: bool,
//     #[builder(default = true)]
//     pub check_commandibility: bool,
// }

use crate::bw::unit_command::UnitCommand;

// region CanIssueCommandArg::builder
pub struct CanIssueCommandArg {
    pub unit_command: UnitCommand,
    pub check_can_use_tech_position_on_positions: bool,
    pub check_can_use_tech_unit_on_units: bool,
    pub check_can_build_unit_type: bool,
    pub check_can_target_unit: bool,
    pub check_can_issue_command_type: bool,
    pub check_commandibility: bool,
}
impl CanIssueCommandArg {
    ///
    ///                    Create a builder for building `CanIssueCommandArg`.
    ///                    On the builder, call `.unit_command(...)`, `.check_can_use_tech_position_on_positions(...)`(optional), `.check_can_use_tech_unit_on_units(...)`(optional), `.check_can_build_unit_type(...)`(optional), `.check_can_target_unit(...)`(optional), `.check_can_issue_command_type(...)`(optional), `.check_commandibility(...)`(optional) to set the values of the fields.
    ///                    Finally, call `.build()` to create the instance of `CanIssueCommandArg`.
    ///
    #[allow(dead_code)]
    pub fn builder() -> CanIssueCommandArgBuilder<((), (), (), (), (), (), ())> {
        CanIssueCommandArgBuilder {
            fields: ((), (), (), (), (), (), ()),
            _phantom: core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub struct CanIssueCommandArgBuilder<TypedBuilderFields> {
    fields: TypedBuilderFields,
    _phantom: (),
}
impl<TypedBuilderFields> Clone for CanIssueCommandArgBuilder<TypedBuilderFields>
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
pub trait CanIssueCommandArgBuilder_Optional<T> {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
}
impl<T> CanIssueCommandArgBuilder_Optional<T> for () {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
        default()
    }
}
impl<T> CanIssueCommandArgBuilder_Optional<T> for (T,) {
    fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
        self.0
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    (),
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    pub fn unit_command(
        self,
        unit_command: UnitCommand,
    ) -> CanIssueCommandArgBuilder<(
        (UnitCommand,),
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        let unit_command = (unit_command,);
        let (
            _,
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_build_unit_type,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandArgBuilder {
            fields: (
                unit_command,
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_build_unit_type,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandArgBuilder_Error_Repeated_field_unit_command {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    (UnitCommand,),
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field unit_command")]
    pub fn unit_command(
        self,
        _: CanIssueCommandArgBuilder_Error_Repeated_field_unit_command,
    ) -> CanIssueCommandArgBuilder<(
        (UnitCommand,),
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    (),
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    pub fn check_can_use_tech_position_on_positions(
        self,
        check_can_use_tech_position_on_positions: bool,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        (bool,),
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        let check_can_use_tech_position_on_positions =
            (check_can_use_tech_position_on_positions,);
        let (
            unit_command,
            _,
            check_can_use_tech_unit_on_units,
            check_can_build_unit_type,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandArgBuilder {
            fields: (
                unit_command,
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_build_unit_type,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandArgBuilder_Error_Repeated_field_check_can_use_tech_position_on_positions {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    (bool,),
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_use_tech_position_on_positions")]
    pub fn check_can_use_tech_position_on_positions(
        self,
        _ : CanIssueCommandArgBuilder_Error_Repeated_field_check_can_use_tech_position_on_positions,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        (bool,),
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    (),
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    pub fn check_can_use_tech_unit_on_units(
        self,
        check_can_use_tech_unit_on_units: bool,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        (bool,),
        __check_can_build_unit_type,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        let check_can_use_tech_unit_on_units = (check_can_use_tech_unit_on_units,);
        let (
            unit_command,
            check_can_use_tech_position_on_positions,
            _,
            check_can_build_unit_type,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandArgBuilder {
            fields: (
                unit_command,
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_build_unit_type,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandArgBuilder_Error_Repeated_field_check_can_use_tech_unit_on_units {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    (bool,),
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_use_tech_unit_on_units")]
    pub fn check_can_use_tech_unit_on_units(
        self,
        _: CanIssueCommandArgBuilder_Error_Repeated_field_check_can_use_tech_unit_on_units,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        (bool,),
        __check_can_build_unit_type,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    (),
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    pub fn check_can_build_unit_type(
        self,
        check_can_build_unit_type: bool,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        (bool,),
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        let check_can_build_unit_type = (check_can_build_unit_type,);
        let (
            unit_command,
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            _,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandArgBuilder {
            fields: (
                unit_command,
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_build_unit_type,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandArgBuilder_Error_Repeated_field_check_can_build_unit_type {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    (bool,),
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_build_unit_type")]
    pub fn check_can_build_unit_type(
        self,
        _: CanIssueCommandArgBuilder_Error_Repeated_field_check_can_build_unit_type,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        (bool,),
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    (),
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    pub fn check_can_target_unit(
        self,
        check_can_target_unit: bool,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        (bool,),
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        let check_can_target_unit = (check_can_target_unit,);
        let (
            unit_command,
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_build_unit_type,
            _,
            check_can_issue_command_type,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandArgBuilder {
            fields: (
                unit_command,
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_build_unit_type,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandArgBuilder_Error_Repeated_field_check_can_target_unit {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    (bool,),
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_target_unit")]
    pub fn check_can_target_unit(
        self,
        _: CanIssueCommandArgBuilder_Error_Repeated_field_check_can_target_unit,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        (bool,),
        __check_can_issue_command_type,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    (),
    __check_commandibility,
)>
{
    pub fn check_can_issue_command_type(
        self,
        check_can_issue_command_type: bool,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        __check_can_target_unit,
        (bool,),
        __check_commandibility,
    )> {
        let check_can_issue_command_type = (check_can_issue_command_type,);
        let (
            unit_command,
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_build_unit_type,
            check_can_target_unit,
            _,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandArgBuilder {
            fields: (
                unit_command,
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_build_unit_type,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandArgBuilder_Error_Repeated_field_check_can_issue_command_type {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    (bool,),
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_issue_command_type")]
    pub fn check_can_issue_command_type(
        self,
        _: CanIssueCommandArgBuilder_Error_Repeated_field_check_can_issue_command_type,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        __check_can_target_unit,
        (bool,),
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    (),
)>
{
    pub fn check_commandibility(
        self,
        check_commandibility: bool,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        __check_can_target_unit,
        __check_can_issue_command_type,
        (bool,),
    )> {
        let check_commandibility = (check_commandibility,);
        let (
            unit_command,
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_build_unit_type,
            check_can_target_unit,
            check_can_issue_command_type,
            _,
        ) = self.fields;
        CanIssueCommandArgBuilder {
            fields: (
                unit_command,
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_build_unit_type,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandArgBuilder_Error_Repeated_field_check_commandibility {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
>
CanIssueCommandArgBuilder<(
    __unit_command,
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    (bool,),
)>
{
    #[deprecated(note = "Repeated field check_commandibility")]
    pub fn check_commandibility(
        self,
        _: CanIssueCommandArgBuilder_Error_Repeated_field_check_commandibility,
    ) -> CanIssueCommandArgBuilder<(
        __unit_command,
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_build_unit_type,
        __check_can_target_unit,
        __check_can_issue_command_type,
        (bool,),
    )> {
        self
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandArgBuilder_Error_Missing_required_field_unit_command {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandArgBuilder<(
    (),
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    #[deprecated(note = "Missing required field unit_command")]
    pub fn build(
        self,
        _: CanIssueCommandArgBuilder_Error_Missing_required_field_unit_command,
    ) -> CanIssueCommandArg {
        {
            // ::std::rt::begin_panic("explicit panic")
            panic!("explicit panic")
        };
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions: CanIssueCommandArgBuilder_Optional<bool>,
    __check_can_use_tech_unit_on_units: CanIssueCommandArgBuilder_Optional<bool>,
    __check_can_build_unit_type: CanIssueCommandArgBuilder_Optional<bool>,
    __check_can_target_unit: CanIssueCommandArgBuilder_Optional<bool>,
    __check_can_issue_command_type: CanIssueCommandArgBuilder_Optional<bool>,
    __check_commandibility: CanIssueCommandArgBuilder_Optional<bool>,
>
CanIssueCommandArgBuilder<(
    (UnitCommand,),
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_build_unit_type,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
)>
{
    pub fn build(self) -> CanIssueCommandArg {
        let (
            unit_command,
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_build_unit_type,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility,
        ) = self.fields;
        let unit_command = unit_command.0;
        let check_can_use_tech_position_on_positions =
            CanIssueCommandArgBuilder_Optional::into_value(
                check_can_use_tech_position_on_positions,
                || true,
            );
        let check_can_use_tech_unit_on_units =
            CanIssueCommandArgBuilder_Optional::into_value(
                check_can_use_tech_unit_on_units,
                || true,
            );
        let check_can_build_unit_type = CanIssueCommandArgBuilder_Optional::into_value(
            check_can_build_unit_type,
            || true,
        );
        let check_can_target_unit =
            CanIssueCommandArgBuilder_Optional::into_value(check_can_target_unit, || true);
        let check_can_issue_command_type = CanIssueCommandArgBuilder_Optional::into_value(
            check_can_issue_command_type,
            || true,
        );
        let check_commandibility =
            CanIssueCommandArgBuilder_Optional::into_value(check_commandibility, || true);
        CanIssueCommandArg {
            unit_command,
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_build_unit_type,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility,
        }
    }
}
// endregion

// region CanIssueCommandGroupedArg::builder
pub struct CanIssueCommandGroupedArg {
    pub check_can_use_tech_position_on_positions: bool,
    pub check_can_use_tech_unit_on_units: bool,
    pub check_can_target_unit: bool,
    pub check_can_issue_command_type: bool,
    pub check_commandibility_grouped: bool,
    pub check_commandibility: bool,
}
impl CanIssueCommandGroupedArg {
    ///
    ///                    Create a builder for building `CanIssueCommandGroupedArg`.
    ///                    On the builder, call `.check_can_use_tech_position_on_positions(...)`(optional), `.check_can_use_tech_unit_on_units(...)`(optional), `.check_can_target_unit(...)`(optional), `.check_can_issue_command_type(...)`(optional), `.check_commandibility_grouped(...)`(optional), `.check_commandibility(...)`(optional) to set the values of the fields.
    ///                    Finally, call `.build()` to create the instance of `CanIssueCommandGroupedArg`.
    ///
    #[allow(dead_code)]
    pub fn builder() -> CanIssueCommandGroupedArgBuilder<((), (), (), (), (), ())> {
        CanIssueCommandGroupedArgBuilder {
            fields: ((), (), (), (), (), ()),
            _phantom: core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub struct CanIssueCommandGroupedArgBuilder<TypedBuilderFields> {
    fields: TypedBuilderFields,
    _phantom: (),
}
impl<TypedBuilderFields> Clone for CanIssueCommandGroupedArgBuilder<TypedBuilderFields>
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
pub trait CanIssueCommandGroupedArgBuilder_Optional<T> {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
}
impl<T> CanIssueCommandGroupedArgBuilder_Optional<T> for () {
    fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
        default()
    }
}
impl<T> CanIssueCommandGroupedArgBuilder_Optional<T> for (T,) {
    fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
        self.0
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    (),
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    pub fn check_can_use_tech_position_on_positions(
        self,
        check_can_use_tech_position_on_positions: bool,
    ) -> CanIssueCommandGroupedArgBuilder<(
        (bool,),
        __check_can_use_tech_unit_on_units,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility_grouped,
        __check_commandibility,
    )> {
        let check_can_use_tech_position_on_positions =
            (check_can_use_tech_position_on_positions,);
        let (
            _,
            check_can_use_tech_unit_on_units,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility_grouped,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandGroupedArgBuilder {
            fields: (
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility_grouped,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_can_use_tech_position_on_positions
{}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    (bool,),
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_use_tech_position_on_positions")]
    pub fn check_can_use_tech_position_on_positions(
        self,
        _ : CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_can_use_tech_position_on_positions,
    ) -> CanIssueCommandGroupedArgBuilder<(
        (bool,),
        __check_can_use_tech_unit_on_units,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility_grouped,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    (),
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    pub fn check_can_use_tech_unit_on_units(
        self,
        check_can_use_tech_unit_on_units: bool,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        (bool,),
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility_grouped,
        __check_commandibility,
    )> {
        let check_can_use_tech_unit_on_units = (check_can_use_tech_unit_on_units,);
        let (
            check_can_use_tech_position_on_positions,
            _,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility_grouped,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandGroupedArgBuilder {
            fields: (
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility_grouped,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_can_use_tech_unit_on_units {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    (bool,),
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_use_tech_unit_on_units")]
    pub fn check_can_use_tech_unit_on_units(
        self,
        _ : CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_can_use_tech_unit_on_units,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        (bool,),
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility_grouped,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    (),
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    pub fn check_can_target_unit(
        self,
        check_can_target_unit: bool,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        (bool,),
        __check_can_issue_command_type,
        __check_commandibility_grouped,
        __check_commandibility,
    )> {
        let check_can_target_unit = (check_can_target_unit,);
        let (
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            _,
            check_can_issue_command_type,
            check_commandibility_grouped,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandGroupedArgBuilder {
            fields: (
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility_grouped,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_can_target_unit {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    (bool,),
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_target_unit")]
    pub fn check_can_target_unit(
        self,
        _: CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_can_target_unit,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        (bool,),
        __check_can_issue_command_type,
        __check_commandibility_grouped,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_commandibility_grouped,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    (),
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    pub fn check_can_issue_command_type(
        self,
        check_can_issue_command_type: bool,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_target_unit,
        (bool,),
        __check_commandibility_grouped,
        __check_commandibility,
    )> {
        let check_can_issue_command_type = (check_can_issue_command_type,);
        let (
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_target_unit,
            _,
            check_commandibility_grouped,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandGroupedArgBuilder {
            fields: (
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility_grouped,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_can_issue_command_type {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_commandibility_grouped,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    (bool,),
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_can_issue_command_type")]
    pub fn check_can_issue_command_type(
        self,
        _ : CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_can_issue_command_type,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_target_unit,
        (bool,),
        __check_commandibility_grouped,
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    (),
    __check_commandibility,
)>
{
    pub fn check_commandibility_grouped(
        self,
        check_commandibility_grouped: bool,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_target_unit,
        __check_can_issue_command_type,
        (bool,),
        __check_commandibility,
    )> {
        let check_commandibility_grouped = (check_commandibility_grouped,);
        let (
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_target_unit,
            check_can_issue_command_type,
            _,
            check_commandibility,
        ) = self.fields;
        CanIssueCommandGroupedArgBuilder {
            fields: (
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility_grouped,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_commandibility_grouped {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    (bool,),
    __check_commandibility,
)>
{
    #[deprecated(note = "Repeated field check_commandibility_grouped")]
    pub fn check_commandibility_grouped(
        self,
        _ : CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_commandibility_grouped,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_target_unit,
        __check_can_issue_command_type,
        (bool,),
        __check_commandibility,
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    (),
)>
{
    pub fn check_commandibility(
        self,
        check_commandibility: bool,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility_grouped,
        (bool,),
    )> {
        let check_commandibility = (check_commandibility,);
        let (
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility_grouped,
            _,
        ) = self.fields;
        CanIssueCommandGroupedArgBuilder {
            fields: (
                check_can_use_tech_position_on_positions,
                check_can_use_tech_unit_on_units,
                check_can_target_unit,
                check_can_issue_command_type,
                check_commandibility_grouped,
                check_commandibility,
            ),
            _phantom: self._phantom,
        }
    }
}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub enum CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_commandibility {}
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    (bool,),
)>
{
    #[deprecated(note = "Repeated field check_commandibility")]
    pub fn check_commandibility(
        self,
        _: CanIssueCommandGroupedArgBuilder_Error_Repeated_field_check_commandibility,
    ) -> CanIssueCommandGroupedArgBuilder<(
        __check_can_use_tech_position_on_positions,
        __check_can_use_tech_unit_on_units,
        __check_can_target_unit,
        __check_can_issue_command_type,
        __check_commandibility_grouped,
        (bool,),
    )> {
        self
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
impl<
    __check_can_use_tech_position_on_positions: CanIssueCommandGroupedArgBuilder_Optional<bool>,
    __check_can_use_tech_unit_on_units: CanIssueCommandGroupedArgBuilder_Optional<bool>,
    __check_can_target_unit: CanIssueCommandGroupedArgBuilder_Optional<bool>,
    __check_can_issue_command_type: CanIssueCommandGroupedArgBuilder_Optional<bool>,
    __check_commandibility_grouped: CanIssueCommandGroupedArgBuilder_Optional<bool>,
    __check_commandibility: CanIssueCommandGroupedArgBuilder_Optional<bool>,
>
CanIssueCommandGroupedArgBuilder<(
    __check_can_use_tech_position_on_positions,
    __check_can_use_tech_unit_on_units,
    __check_can_target_unit,
    __check_can_issue_command_type,
    __check_commandibility_grouped,
    __check_commandibility,
)>
{
    pub fn build(self) -> CanIssueCommandGroupedArg {
        let (
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility_grouped,
            check_commandibility,
        ) = self.fields;
        let check_can_use_tech_position_on_positions =
            CanIssueCommandGroupedArgBuilder_Optional::into_value(
                check_can_use_tech_position_on_positions,
                || true,
            );
        let check_can_use_tech_unit_on_units =
            CanIssueCommandGroupedArgBuilder_Optional::into_value(
                check_can_use_tech_unit_on_units,
                || true,
            );
        let check_can_target_unit = CanIssueCommandGroupedArgBuilder_Optional::into_value(
            check_can_target_unit,
            || true,
        );
        let check_can_issue_command_type =
            CanIssueCommandGroupedArgBuilder_Optional::into_value(
                check_can_issue_command_type,
                || true,
            );
        let check_commandibility_grouped =
            CanIssueCommandGroupedArgBuilder_Optional::into_value(
                check_commandibility_grouped,
                || true,
            );
        let check_commandibility = CanIssueCommandGroupedArgBuilder_Optional::into_value(
            check_commandibility,
            || true,
        );
        CanIssueCommandGroupedArg {
            check_can_use_tech_position_on_positions,
            check_can_use_tech_unit_on_units,
            check_can_target_unit,
            check_can_issue_command_type,
            check_commandibility_grouped,
            check_commandibility,
        }
    }
}
// endregion

#[derive(TypedBuilder)]
pub struct CanAttackArg {
    #[builder(default = true)]
    check_can_target_unit: bool,
    #[builder(default = true)]
    check_can_issue_command_type: bool,
    #[builder(default = true)]
    check_commandibility: bool,
}