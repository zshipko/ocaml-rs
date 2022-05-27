open Rust

let%test "enum1 empty" = Util.check_leaks (fun () -> (enum1_empty () = Empty))
let%test "enum1 first 1" = Util.check_leaks (fun () -> (enum1_first 1 = First 1))
let%test "enum1 first 9999" = Util.check_leaks (fun () -> (enum1_first 9999 = First 9999))

let test_second s =
  let second = enum1_make_second s in
  let () = Util.gc () in
  let value = enum1_get_second_value second in
  let () = Util.gc () in
  match value with
  | Some a -> Some a.(0)
  | None -> None


let%test "enum1 second" = Util.check_leaks (fun () -> (test_second "testing" = Some "testing"))

let%test "enum1 is empty 0" =  Util.check_leaks (fun () -> (enum1_is_empty Empty = true))
let%test "enum1 is empty 1" = Util.check_leaks (fun () -> enum1_is_empty (First 1) = false)

let%test "struct1 c" = Util.check_leaks (fun () ->
  let s = struct1_empty () in
  let () = Util.gc () in
  let s = struct1_set_c s "testing" in
  Util.gc ();
  struct1_get_c s = Some "testing" && s.c = Some "testing"
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

let%test "string (non-copy)" = Util.check_leaks (fun () -> (
  let a = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789" in
  Util.gc ();
  string_non_copying a = a
))


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

let%test "get-pair-vec" = Util.check_leaks (fun () -> (
  pair_vec () = [| "foo", 1; "bar", 2 |]
))

let%test "get-string-array" = Util.check_leaks (fun () -> (
  let _foo = string_array () in
  true
))

let%test "get-array-conv" = Util.check_leaks (fun () -> (
  let a = Bytes.of_string "\x01\x02\x03\x04\x05" in
  let expected_b = Bytes.of_string "\x01\x02\x03\x04\x05\x0f\xff" in
  array_conv a = expected_b
))

let%test "result" = Util.check_leaks (fun () -> (
  let ok = result_ok "123" in
  let err = result_error (`Test 123) in
  result_get_ok ok = Some "123" && result_get_error err = Some (`Test 123) && result_get_ok err = None && result_get_error ok = None
))
