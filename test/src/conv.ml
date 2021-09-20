type enum1 =
  | Empty
  | First of int
  | Second of string array

external enum1_empty: unit -> enum1 = "enum1_empty"
external enum1_first: int -> enum1 = "enum1_first"

let%test "enum1 empty" = Util.check_leaks (fun () -> (enum1_empty () = Empty))
let%test "enum1 first 1" = Util.check_leaks (fun () -> (enum1_first 1 = First 1))
let%test "enum1 first 9999" = Util.check_leaks (fun () -> (enum1_first 9999 = First 9999))

external enum1_make_second: string -> enum1 = "enum1_make_second"
external enum1_get_second_value : enum1 -> string array option = "enum1_get_second_value"

let test_second s =
  let second = enum1_make_second s in
  let () = Util.gc () in
  let value = enum1_get_second_value second in
  let () = Util.gc () in
  match value with
  | Some a -> Some a.(0)
  | None -> None


let%test "enum1 second" = Util.check_leaks (fun () -> (test_second "testing" = Some "testing"))

external enum1_is_empty: enum1 -> bool = "enum1_is_empty"

let%test "enum1 is empty 0" =  Util.check_leaks (fun () -> (enum1_is_empty Empty = true))
let%test "enum1 is empty 1" = Util.check_leaks (fun () -> enum1_is_empty (First 1) = false)

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

let%test "struct1 c" = Util.check_leaks (fun () ->
  let s = struct1_empty () in
  let () = Util.gc () in
  struct1_set_c s "testing";
  Util.gc ();
  struct1_get_c s = s.c
)

let%test "struct1 c (make)" = Util.check_leaks (fun () ->
  let s = make_struct1 1 2.0 (Some "testing") None in
  Util.gc ();
  struct1_get_c s = Some "testing"
)

let%test "struct1 c (make) 2" = Util.check_leaks (fun () ->
  let s = make_struct1 1 2.0 None None in
  Util.gc ();
  struct1_get_c s = None
)

let%test "struct1 c" = Util.check_leaks (fun () ->
  let s = {a = 1; b = 2.0; c = Some "testing"; d = None} in
  Util.gc ();
  struct1_get_c s = Some "testing" && struct1_get_c s = s.c
)

let%test "struct1 d" = Util.check_leaks (fun () ->
  let s = {a = 1; b = 2.0; c = None; d = Some [| "abc"; "123" |]} in
  Util.gc ();
  struct1_get_d s = Some [| "abc"; "123" |] && struct1_get_d s = s.d
)

let%test "struct1 d 2" = Util.check_leaks (fun () -> (
  let s = make_struct1 1 2.0 None (Some [| "abc"; "123" |]) in
  Util.gc ();
  struct1_get_d s = Some [| "abc"; "123" |] && struct1_get_d s = s.d)
)

external string_non_copying: string -> string = "string_non_copying"

let%test "string (non-copy)" = Util.check_leaks (fun () -> (
  let a = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789" in
  Util.gc ();
  string_non_copying a = a
))


external direct_slice: Int64.t array -> Int64.t = "direct_slice"
external deep_clone : 'a -> 'a = "deep_clone"

let%test "direct slice 1" = Util.check_leaks (fun () -> (
  let arr = [| 1L; 2L; 3L |] in
  Util.gc ();
  direct_slice arr = 6L
))

let%test "deep clone 1" = Util.check_leaks (fun () -> (
  let a = [1; 2; 3; 4; 5] in
  Util.gc ();
  deep_clone a = a
))

external get_pair_vec: unit -> (string * int) array = "pair_vec"

let%test "get-pair-vec" = Util.check_leaks (fun () -> (
  get_pair_vec () = [| "foo", 1; "bar", 2 |]
))

external get_string_array: unit -> string array = "string_array"

let%test "get-string-array" = Util.check_leaks (fun () -> (
  let _foo = get_string_array () in
  true
))

external get_array_conv: bytes -> bytes = "array_conv"

let%test "get-array-conv" = Util.check_leaks (fun () -> (
  let a = Bytes.of_string "\x01\x02\x03\x04\x05" in
  let expected_b = Bytes.of_string "\x01\x02\x03\x04\x05\x0f\xff" in
  get_array_conv a = expected_b
))
