#![allow(non_upper_case_globals)]

pub const Forward_tag: ::std::os::raw::c_uint = 250;
pub const Infix_tag: ::std::os::raw::c_uint = 249;
pub const Object_tag: ::std::os::raw::c_uint = 248;
pub const Closure_tag: ::std::os::raw::c_uint = 247;
pub const Lazy_tag: ::std::os::raw::c_uint = 246;
pub const Abstract_tag: ::std::os::raw::c_uint = 251;
pub const No_scan_tag: ::std::os::raw::c_uint = 251;
pub const String_tag: ::std::os::raw::c_uint = 252;
pub const Double_tag: ::std::os::raw::c_uint = 253;
pub const Double_array_tag: ::std::os::raw::c_uint = 254;
pub const Custom_tag: ::std::os::raw::c_uint = 255;

/// Tags are used to determine the type of value that is stored
/// in an OCaml value
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Tag {
    Tag(u8),
    Zero,
    Forward,
    Infix,
    Object,
    Closure,
    Lazy,
    Abstract,
    String,
    Double,
    DoubleArray,
    Custom,
}

impl Tag {
    /// Create a `Tag` from the given `u8`
    pub fn new(i: u8) -> Tag {
        match i as u32 {
            0 => Tag::Zero,
            Forward_tag => Tag::Forward,
            Infix_tag => Tag::Infix,
            Object_tag => Tag::Object,
            Closure_tag => Tag::Closure,
            Lazy_tag => Tag::Lazy,
            Abstract_tag => Tag::Abstract,
            String_tag => Tag::String,
            Double_tag => Tag::Double,
            Double_array_tag => Tag::DoubleArray,
            Custom_tag => Tag::Custom,
            n => Tag::Tag(n as u8),
        }
    }
}

impl From<Tag> for u8 {
    fn from(t: Tag) -> u8 {
        match t {
            Tag::Tag(x) => x,
            Tag::Zero => 0u8,
            Tag::Forward => Forward_tag as u8,
            Tag::Infix => Infix_tag as u8,
            Tag::Object => Object_tag as u8,
            Tag::Closure => Closure_tag as u8,
            Tag::Lazy => Lazy_tag as u8,
            Tag::Abstract => Abstract_tag as u8,
            Tag::String => String_tag as u8,
            Tag::Double => Double_tag as u8,
            Tag::DoubleArray => Double_array_tag as u8,
            Tag::Custom => Custom_tag as u8,
        }
    }
}
