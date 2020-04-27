type testing

external testing_alloc: int64 -> testing = "testing_alloc"
external testing_set_c: testing -> string -> unit = "testing_set_c"
external testing_set_a: testing -> float -> unit = "testing_set_a"
external testing_get_values: testing -> (float * int64 * string) = "testing_get_values"

let%test "testing compare 1" = (testing_alloc 0L <> testing_alloc 1L)
let%test "testing compare 2" = (testing_alloc 99L = testing_alloc 99L)
let%test "testing set c" = (
  let t = testing_alloc 1L in
  testing_set_a t 3.14;
  testing_set_c t "FOOBAR";
  let (a, b, c) = testing_get_values t in
  a = 3.14 && b = 1L && c = "FOOBAR"
)

type testing_callback
external testing_callback_alloc: (int -> float) -> testing_callback = "testing_callback_alloc"
external testing_callback_call: testing_callback -> int -> float = "testing_callback_call"

let%test "testing callback 1" = (
  let c = testing_callback_alloc (fun x -> float_of_int x *. 2.) in
  Gc.minor ();
  Gc.full_major ();
  testing_callback_call c 1 = 2.0
)

let%test "testing callback 2" = (
  let c = testing_callback_alloc (fun x ->
    let () = Unix.sleep 2 in
    sin (float_of_int x)) in
  Gc.minor ();
  Gc.full_major ();
  testing_callback_call c 5 = sin 5.0
)
