type enum1 =
  | Empty
  | First of int
  | Second of string array

external enum1_empty: unit -> enum1 = "enum1_empty"
external enum1_first: int -> enum1 = "enum1_first"
external enum1_make_second: string -> enum1 = "enum1_make_second"
external enum1_get_second_value : enum1 -> string array option = "enum1_get_second_value"
external enum1_is_empty: enum1 -> bool = "enum1_is_empty"

let%test "enum1 empty" = (enum1_empty () = Empty)
let%test "enum1 first 1" = (enum1_first 1 = First 1)
let%test "enum1 first 9999" = (enum1_first 9999 = First 9999)

let test_second s =
  let second = enum1_make_second s in
  let value = enum1_get_second_value second in
  match value with
  | Some a -> Some a.(0)
  | None -> None

let%test "enum1 second" = (test_second "testing" = Some "testing")


let%test "enum1 is empty 0" = (enum1_is_empty Empty = true)
let%test "enum1 is empty 1" = (enum1_is_empty (First 1) = false)
