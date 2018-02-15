#![allow(non_upper_case_globals)]

pub const Forward_tag: ::std::os::raw::c_uint = 250;
pub const Infix_tag: ::std::os::raw::c_uint = 249;
pub const Object_tag: ::std::os::raw::c_uint = 248;
pub const Closure_tag: ::std::os::raw::c_uint = 247;
pub const Lazy_tag: ::std::os::raw::c_uint = 246;
pub const Abstract_tag: ::std::os::raw::c_uint = 251;
pub const String_tag: ::std::os::raw::c_uint = 252;
pub const Double_tag: ::std::os::raw::c_uint = 253;
pub const Double_array_tag: ::std::os::raw::c_uint = 254;
pub const Custom_tag: ::std::os::raw::c_uint = 255;

#[derive(Debug, Clone, Copy)]
pub enum Tag {
    Zero = 0,
    Forward = Forward_tag as isize,
    Infix = Infix_tag as isize,
    Object = Object_tag as isize,
    Closure = Closure_tag as isize,
    Lazy = Lazy_tag as isize,
    Abstract = Abstract_tag as isize,
    String = String_tag as isize,
    Double = Double_tag as isize,
    DoubleArray = Double_array_tag as isize,
    Custom = Custom_tag as isize
}

impl Tag {
    pub fn new(i: u32) -> Tag {
        match i {
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
            _ => Tag::Zero
        }
    }
}

