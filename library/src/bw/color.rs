#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextColor {
    Previous = 1,
    Default = 2,
    Yellow = 3,
    White = 4,
    Grey = 5,
    Red = 6,
    Green = 7,
    BrightRed = 8,
    Invisible = 11,
    Blue = 14,
    Teal = 15,
    Purple = 16,
    Orange = 17,
    AlignRight = 18,
    AlignCenter = 19,
    Invisible2 = 20,
    Brown = 21,
    PlayerWhite = 22,
    PlayerYellow = 23,
    DarkGreen = 24,
    LightYellow = 25,
    Cyan = 26,
    Tan = 27,
    GreyBlue = 28,
    GreyGreen = 29,
    GreyCyan = 30,
    Turquoise = 31,
}

// required for ffi layer
unsafe impl cxx::ExternType for TextColor {
    type Id = cxx::type_id!("BWAPI::Text::Enum");
    type Kind = cxx::kind::Trivial;
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TextSize {
    Small = 0,
    Default,
    Large,
    Huge,
}

unsafe impl cxx::ExternType for TextSize {
    type Id = cxx::type_id!("BWAPI::Text::Size::Enum");
    type Kind = cxx::kind::Trivial;
}
