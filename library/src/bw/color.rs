#[repr(i32)]
#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub enum Color {
    Black = 0,
    Brown = 19,
    Grey = 74,
    Red = 111,
    Green = 117,
    Cyan = 128,
    Yellow = 135,
    Teal = 159,
    Purple = 164,
    Blue = 165,
    Orange = 179,
    White = 255,
}

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
