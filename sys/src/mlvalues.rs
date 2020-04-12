//! Contains OCaml types and conversion functions from runtime representations.
use crate::tag::Tag;

/// OCaml `value` type
pub type Value = usize;

/// OCaml's integer type
pub type Intnat = isize;

/// OCaml's unsigned integer type
pub type Uintnat = usize;

/// OCaml's size type
pub type Size = Uintnat;
pub type Color = Uintnat;
pub type Mark = Uintnat;

/// An OCaml heap-allocated block header. **NB**: this is currently unimplemented.
///
/// Structure of the header:
///
/// For 16-bit and 32-bit architectures:
///
///```text
///      +--------+-------+-----+
///      | wosize | color | tag |
///      +--------+-------+-----+
/// bits  31    10 9     8 7   0
///```
///
/// For 64-bit architectures:
///
///```text
///      +--------+-------+-----+
///      | wosize | color | tag |
///      +--------+-------+-----+
/// bits  63    10 9     8 7   0
///```
///
pub type Header = Uintnat;

/// #ifdef ARCH_BIG_ENDIAN
/// #define Tag_val(val) (((unsigned char *) (val)) [-1])
/// #else
/// #define Tag_val(val) (((unsigned char *) (val)) [-sizeof(value)])
/// #endif
#[cfg(target_endian = "big")]
#[inline]
pub const unsafe fn tag_val(val: Value) -> Tag {
    *(val as *const u8).offset(-1)
}

#[cfg(target_endian = "little")]
#[inline]
pub unsafe fn tag_val(val: Value) -> Tag {
    *(val as *const u8).offset(-(core::mem::size_of::<Value>() as isize))
}

#[inline]
pub unsafe fn hd_val(val: Value) -> Header {
    *(val as *const Header).offset(-1)
}

#[inline]
pub unsafe fn wosize_val(val: Value) -> Size {
    hd_val(val) >> 10
}

/// `(((intnat)(x) << 1) + 1)`
pub const fn val_int(i: isize) -> Value {
    ((i as usize) << 1) + 1
}

pub const fn int_val(val: Value) -> isize {
    ((val as usize) >> 1) as isize
}

pub fn is_block(v: Value) -> bool {
    (v & 1) == 0
}

pub fn is_long(v: Value) -> bool {
    (v & 1) != 0
}

// #define Max_long (((intnat)1 << (8 * sizeof(value) - 2)) - 1)
// #define Min_long (-((intnat)1 << (8 * sizeof(value) - 2)))

/// Extract a field from an OCaml value
///
/// # Safety
///
/// This function does no bounds checking or validation of the OCaml values
pub unsafe fn field(block: Value, index: usize) -> *mut Value {
    (block as *mut Value).add(index)
}

#[doc(hidden)]
pub unsafe fn as_slice<'a>(value: Value) -> &'a [Value] {
    ::core::slice::from_raw_parts((value as *const Value).offset(-1), wosize_val(value) + 1)
}

/// The OCaml `()` (`unit`) value
pub const UNIT: Value = val_int(0);

/// Empty list value
pub const EMPTY_LIST: Value = val_int(0);

/// The OCaml `true` value
pub const TRUE: Value = val_int(1);

/// OCaml `false` value
pub const FALSE: Value = val_int(0);

/// Pointer to the first byte
#[inline]
pub const unsafe fn bp_val(val: Value) -> *const u8 {
    val as *const u8
}

/// Extracts a machine `ptr` to the bytes making up an OCaml `string`
#[inline]
pub const unsafe fn string_val(val: Value) -> *mut u8 {
    val as *mut u8
}

extern "C" {
    /// Returns size of the string in `value` in bytes
    pub fn caml_string_length(value: Value) -> Size;
    pub fn caml_array_length(value: Value) -> Size;
    pub fn caml_hash_variant(tag: *const u8) -> Value;
    pub fn caml_get_public_method(obj: Value, tag: Value) -> Value;
}
