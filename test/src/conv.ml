type enum1 =
  | Empty
  | First of int
  | Second of string array

external enum1_empty: unit -> enum1 = "enum1_empty"
external enum1_first: int -> enum1 = "enum1_first"

let%test "enum1 empty" = (enum1_empty () = Empty)
let%test "enum1 first 1" = (enum1_first 1 = First 1)
let%test "enum1 first 9999" = (enum1_first 9999 = First 9999)

external enum1_make_second: string -> enum1 = "enum1_make_second"
external enum1_get_second_value : enum1 -> string array option = "enum1_get_second_value"

let test_second s =
  let second = enum1_make_second s in
  let value = enum1_get_second_value second in
  match value with
  | Some a -> Some a.(0)
  | None -> None

let%test "enum1 second" = (test_second "testing" = Some "testing")

external enum1_is_empty: enum1 -> bool = "enum1_is_empty"

let%test "enum1 is empty 0" = (enum1_is_empty Empty = true)
let%test "enum1 is empty 1" = (enum1_is_empty (First 1) = false)

type struct1 = {
  a: int;
  b: float;
  mutable c: string option;
  d: string array option;
}

external struct1_empty: unit -> struct1 = "struct1_empty"
external struct1_get_c: struct1 -> string option = "struct1_get_c"
external struct1_set_c: struct1 -> string -> unit = "struct1_set_c"
external struct1_get_d: struct1 -> string array option = "struct1_get_d"
external make_struct1: int -> float -> string option -> string array option -> struct1 = "make_struct1"

let%test "struct1 c" = (
  let s = struct1_empty () in
  struct1_set_c s "testing";
  struct1_get_c s = s.c
)

let%test "struct1 c (make)" = (
  let s = make_struct1 1 2.0 (Some "testing") None in
  struct1_get_c s = Some "testing"
)

let%test "struct1 c (make) 2" = (
  let s = make_struct1 1 2.0 None None in
  struct1_get_c s = None
)

let%test "struct1 c" = (
  let s = {a = 1; b = 2.0; c = Some "testing"; d = None} in
  struct1_get_c s = Some "testing" && struct1_get_c s = s.c

)

let%test "struct1 d" = (
  let s = {a = 1; b = 2.0; c = None; d = Some [| "abc"; "123" |]} in
  struct1_get_d s = Some [| "abc"; "123" |] && struct1_get_d s = s.d
)

let%test "struct1 d 2" = (
  let s = make_struct1 1 2.0 None (Some [| "abc"; "123" |]) in
  struct1_get_d s = Some [| "abc"; "123" |] && struct1_get_d s = s.d
)

external string_non_copying: string -> string = "string_non_copying"

let%test "string (non-copy)" = (
  let a = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789" in
  string_non_copying a = a
)


external direct_slice: Int64.t array -> Int64.t = "direct_slice"
external deep_clone : 'a -> 'a = "deep_clone"

let%test "direct slice 1" = (
  let arr = [| 1L; 2L; 3L |] in
  direct_slice arr = 6L
)

let%test "deep clone 1" = (
  let a = [1; 2; 3; 4; 5] in
  deep_clone a = a
)

external get_pair_vec: unit -> (string * int) array = "pair_vec"

let%test "get-pair-vec" = (
  get_pair_vec () = [| "foo", 1; "bar", 2 |]
)

type arr = (float, Bigarray.float32_elt, Bigarray.c_layout) Bigarray.Array2.t

external make_array2: int -> int -> arr = "make_array2"
external array2_format: arr -> string = "array2_format"
external array2_set: arr -> int -> int -> float -> unit = "array2_set"
external array2_get: arr -> int -> int -> float = "array2_get"

let test_array2_checked dim1 dim2 = (
  let arr = make_array2 dim1 dim2 in
  let rec check x y v =
    if not v || x == dim1 then v else
      if y == dim2 then
        check (x + 1) 0 v
      else
        let value = float_of_int (x * y) in
        array2_set arr x y value;
        check x (y + 1) (array2_get arr x y = value && arr.{x, y} = value)
  in
  arr, check 0 0 true
)

let%test "array2" = (
  let dim1 = 9000 and dim2 = 800 in
  let _, check = test_array2_checked dim1 dim2 in
  check
)

let%test "array2_format" = (
  let dim1 = 3 and dim2 = 3 in
  let arr, check = test_array2_checked dim1 dim2 in
  check && (array2_format arr) = "[[0, 0, 0], [0, 1, 2], [0, 2, 4]]"
)
