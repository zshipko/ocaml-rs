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

let%test "struct1 c" = (
  let s = struct1_empty () in
  struct1_set_c s "testing";
  struct1_get_c s = s.c
)

let%test "struct1 c" = (
  let s = {a = 1; b = 2.0; c = Some "testing"; d = None} in
  struct1_get_c s = Some "testing" && struct1_get_c s = s.c

)

let%test "struct1 d" = (
  let s = {a = 1; b = 2.0; c = None; d = Some [| "abc"; "123" |]} in
  struct1_get_d s = Some [| "abc"; "123" |] && struct1_get_d s = s.d
)
