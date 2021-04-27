// todo
pub struct Color;

// required for ffi layer
unsafe impl cxx::ExternType for Color {
    type Id = cxx::type_id!("BWAPI::Color");
    type Kind = cxx::kind::Trivial;
}

pub enum TextSize {
    Small,
    Default,
    Large,
    Huge,
}

// required for ffi layer
unsafe impl cxx::ExternType for TextSize {
    type Id = cxx::type_id!("BWAPI::Text::Size::Enum");
    type Kind = cxx::kind::Trivial;
}
